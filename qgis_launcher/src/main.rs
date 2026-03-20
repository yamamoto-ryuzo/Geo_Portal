#![cfg_attr(feature = "gui", windows_subsystem = "windows")]

use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::os::windows::process::CommandExt;
use serde::{Deserialize, Serialize};
use winreg::enums::*;

/// 子プロセスのコンソールウィンドウを非表示にする Windows フラグ
const CREATE_NO_WINDOW: u32 = 0x08000000;
use winreg::RegKey;

#[cfg(feature = "gui")]
use fltk::{prelude::*, *};
#[cfg(feature = "gui")]
use fltk::enums::Align;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RcloneMount {
    pub remote: Option<String>,
    pub drive: String,
    #[serde(default)]
    pub read_only: bool,
    /// "subst"（デフォルト）/ "sync" / "mount"
    pub mode: Option<String>,
    /// ドライブに割り当てるローカルフォルダ（例: "C:\\qgis_cache\\master"）
    pub local_cache: Option<String>,
    /// robocopy のコピー元フォルダ。指定時は subst 前に robocopy を実行
    pub robocopy_src: Option<String>,
    /// robocopy で除外するサブフォルダ名のリスト（例: ["secret-folder", "private-data"]）
    #[serde(default)]
    pub robocopy_exclude: Vec<String>,
    // mount モード用オプション
    pub vfs_cache_mode: Option<String>,
    pub vfs_cache_max_age: Option<String>,
    pub vfs_cache_max_size: Option<String>,
    pub vfs_cache_poll_interval: Option<String>,
    pub vfs_write_back: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QgisSettings {
    pub profile: String,
    pub project_path: Vec<String>,
    pub qgis_executable: Option<String>,
    pub reearth_url: Option<String>,
    pub box_url: Option<String>,
    pub settings_dir: Option<String>,
    pub rclone_exe: Option<String>,
    #[serde(default)]
    pub rclone_mounts: Vec<RcloneMount>,
    /// パスエイリアス表。キーが "BOX" なら "BOX:\\path" と書ける。
    /// デフォルト: {"BOX": "%USERPROFILE%\\Box"}
    #[serde(default)]
    pub path_aliases: HashMap<String, String>,
}

impl Default for QgisSettings {
    fn default() -> Self {
        Self {
            profile: "".to_string(),
            project_path: Vec::new(),
            qgis_executable: None,
            reearth_url: None,
            box_url: None,
            settings_dir: Some(r"C:\qgis_launcher".to_string()),
            rclone_exe: None,
            rclone_mounts: Vec::new(),
            path_aliases: HashMap::new(),
        }
    }
}

/// QGIS起動用ランチャー
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// スタートアップに登録する

    /// 適用する環境設定プロファイル名
    #[arg(short, long, default_value = "geo_custom")]
    profile: String,

    /// 設定ファイル(qgis_settings.json)を配置するディレクトリパス
    #[arg(long, default_value = r"C:\qgis_launcher")]
    settings_dir: String,
    /// コマンドラインモードで動作する（指定がなければ GUI を起動）
    #[arg(long, default_value_t = false)]
    cli: bool,

    /// QGISの実行ファイルパス（指定がなければ自動検出）
    #[arg(long)]
    qgis_executable: Option<String>,
}

fn get_settings_path(custom_dir: &str) -> PathBuf {
    let target_dir = PathBuf::from(custom_dir);
    let target_path = target_dir.join("qgis_settings.json");

    if target_dir.exists() {
        return target_path;
    }

    let mut path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop();
    path.join("qgis_settings.json")
}

/// JSON文字列値内の不正な単一バックスラッシュを \\ に修正する。
/// Windowsのエクスプローラからコピーしたパス ("C:\foo" 等) に対応。
fn fix_backslashes_in_json(text: &str) -> String {
    // JSON文字列リテラルを1つずつ処理し、有効なエスケープ以外の \ を \\ に置換する
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '"' {
            result.push(c);
            continue;
        }
        // 文字列リテラルの開始
        result.push('"');
        loop {
            match chars.next() {
                None => break,
                Some('"') => { result.push('"'); break; }
                Some('\\') => {
                    match chars.peek().copied() {
                        // 有効な JSON エスケープ: " \ / b f n r t u → 両文字を消費して出力
                        Some('"') | Some('\\') | Some('/') |
                        Some('b') | Some('f') | Some('n') | Some('r') | Some('t') | Some('u') => {
                            let next = chars.next().unwrap();
                            result.push('\\');
                            result.push(next);
                        }
                        // 無効なエスケープ → \\ に変換（次文字は消費しない）
                        _ => {
                            result.push('\\');
                            result.push('\\');
                        }
                    }
                }
                Some(other) => { result.push(other); }
            }
        }
    }
    result
}

/// ユーザーオーバーライド用 JSON ファイルを探して、ベース JSON Value にマージする。
/// ファイル名: qgis_settings_{USERNAME}.json（例: qgis_settings_yamamoto.json）
/// 存在しない場合は base をそのまま返す。
/// マージはシャロー: オーバーライド側のトップレベルキーがベースを上書き。
fn apply_user_override(base_dir: &str, base: serde_json::Value) -> serde_json::Value {
    let username = env::var("USERNAME").unwrap_or_default();
    if username.is_empty() {
        return base;
    }
    let override_path = PathBuf::from(base_dir).join(format!("qgis_settings_{}.json", username));
    if !override_path.exists() {
        return base;
    }
    apply_override_value(base, &override_path)
}

/// 無条件オーバーライド用 JSON ファイル（qgis_settings_override.json）を
/// ユーザー名に関係なく常に適用する。
/// ユーザーオーバーライド（qgis_settings_{USERNAME}.json）の後に適用されるため、
/// すべてのユーザー設定を上書きする最終フィルタとして機能する。
fn apply_force_override(base_dir: &str, base: serde_json::Value) -> serde_json::Value {
    let override_path = PathBuf::from(base_dir).join("qgis_settings_override.json");
    if !override_path.exists() {
        return base;
    }
    // apply_user_override と同じマージロジックを再利用
    apply_override_value(base, &override_path)
}

/// apply_user_override / apply_force_override 共通のマージ処理
fn apply_override_value(mut base: serde_json::Value, override_path: &PathBuf) -> serde_json::Value {
    if let Ok(data) = fs::read_to_string(override_path) {
        let fixed = fix_backslashes_in_json(&data);
        if let Ok(override_val) = serde_json::from_str::<serde_json::Value>(&fixed) {
            if let (Some(base_obj), Some(over_obj)) = (base.as_object_mut(), override_val.as_object()) {
                for (k, v) in over_obj {
                    match k.as_str() {
                        "path_aliases" => {
                            if let Some(over_map) = v.as_object() {
                                let base_map = base_obj
                                    .entry(k)
                                    .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
                                if let Some(bm) = base_map.as_object_mut() {
                                    for (ak, av) in over_map {
                                        bm.insert(ak.clone(), av.clone());
                                    }
                                }
                            }
                        }
                        "rclone_mounts" => {
                            if let Some(over_mounts) = v.as_array() {
                                let base_mounts = base_obj
                                    .entry(k)
                                    .or_insert_with(|| serde_json::Value::Array(vec![]));
                                if let Some(bm) = base_mounts.as_array_mut() {
                                    for om in over_mounts {
                                        let over_drive = om.get("drive").and_then(|d| d.as_str());
                                        if let Some(drive) = over_drive {
                                            if let Some(base_entry) = bm.iter_mut().find(|e| {
                                                e.get("drive").and_then(|d| d.as_str()) == Some(drive)
                                            }) {
                                                if let (Some(be), Some(oe)) =
                                                    (base_entry.as_object_mut(), om.as_object())
                                                {
                                                    for (fk, fv) in oe {
                                                        be.insert(fk.clone(), fv.clone());
                                                    }
                                                }
                                            } else {
                                                bm.push(om.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            base_obj.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
    }
    base
}

fn get_current_settings(custom_dir: &str) -> QgisSettings {
    let path = get_settings_path(custom_dir);
    if let Ok(data) = fs::read_to_string(path) {
        // 読み込み時に常にバックスラッシュを事前修正してからパースする。
        // Accept either string or array for `project_path` for backward compatibility.
        let fixed = fix_backslashes_in_json(&data);
        if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&fixed) {
            // ユーザーオーバーライドファイルをマージする
            v = apply_user_override(custom_dir, v);
            // 無条件オーバーライドファイルをマージする（常に最後に適用）
            v = apply_force_override(custom_dir, v);
            if let Some(p) = v.get("project_path") {
                if p.is_string() {
                    let s = p.as_str().unwrap_or("");
                    v["project_path"] = serde_json::Value::Array(vec![serde_json::Value::String(s.to_string())]);
                }
            }
            if let Ok(s) = serde_json::from_value::<QgisSettings>(v) {
                return s;
            }
        }
        QgisSettings::default()
    } else {
        QgisSettings::default()
    }
}

fn save_settings(custom_dir: &str, s: &QgisSettings) -> Result<(), String> {
    let p = get_settings_path(custom_dir);
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("dir create error: {}", e))?;
        }
    }
    let data = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    fs::write(&p, data).map_err(|e| e.to_string())
}

#[cfg(feature = "gui")]
fn get_available_profiles(settings_dir: &str, current_val: &str) -> Vec<String> {
    let mut profiles = Vec::new();
    if !current_val.is_empty() {
        profiles.push(current_val.to_string());
    }
    
    // 1. APPDATA 以下の既存の QGIS プロファイルを検索する
    for p in qgis_launcher::get_qgis_profile_paths() {
        if let Ok(entries) = fs::read_dir(&p) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        if let Ok(name) = entry.file_name().into_string() {
                            if !profiles.contains(&name) {
                                profiles.push(name);
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. settings_dir/profiles (配布・コピー用) のプロファイルを検索する
    // 構造: profiles/QGIS3/profiles/<プロファイル名>/ または profiles/<プロファイル名>/
    let p = PathBuf::from(settings_dir).join("profiles");
    if p.exists() {
        if let Ok(entries) = fs::read_dir(&p) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        // QGIS3 / QGIS4 のようなバージョンフォルダ → その中の profiles/ を列挙
                        let is_version_dir = name.to_uppercase().starts_with("QGIS")
                            && name[4..].chars().all(|c| c.is_ascii_digit());
                        if is_version_dir {
                            let inner = entry.path().join("profiles");
                            if let Ok(inner_entries) = fs::read_dir(&inner) {
                                for ie in inner_entries.flatten() {
                                    if let Ok(ift) = ie.file_type() {
                                        if ift.is_dir() {
                                            if let Ok(iname) = ie.file_name().into_string() {
                                                if !profiles.contains(&iname) {
                                                    profiles.push(iname);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // バージョンフォルダなし → 直下をプロファイルとして使用
                            if !profiles.contains(&name) {
                                profiles.push(name);
                            }
                        }
                    }
                }
            }
        }
    }
    profiles
}

#[cfg(feature = "gui")]
/// ファイルの絶対パスから GUI 表示名「親フォルダ名 - ファイル名」を生成する。
/// ルートレベル（C:\ 直下など）の場合はドライブ文字を親名として使用。
fn display_name_for(abs_path: &str) -> String {
    let pb = PathBuf::from(abs_path);
    let fname = match pb.file_name().and_then(|n| n.to_str()) {
        Some(s) => s.to_string(),
        None => return abs_path.to_string(),
    };
    let parent = match pb.parent() {
        Some(p) => p,
        None => return fname,
    };
    // 通常フォルダ: 末尾コンポーネント名を使用
    if let Some(dir_name) = parent.file_name().and_then(|n| n.to_str()) {
        return format!("{} - {}", dir_name, fname);
    }
    // ルート（C:\ など）: ドライブ文字を使用
    let root = parent.to_str().unwrap_or("").trim_end_matches(['/', '\\']);
    if !root.is_empty() {
        format!("{} - {}", root, fname)
    } else {
        fname
    }
}

#[cfg(feature = "gui")]
/// GUI 用プロジェクト一覧を返す。
/// 戻り値: (表示名, 実絶対パス) のペアのリスト
/// - 拡張子 .qgs/.qgz → ファイル指定: 存在すれば1エントリ追加
/// - それ以外          → フォルダ指定: 直下の .qgs/.qgz を列挙して追加
fn get_available_projects(settings_dir: &str, current_val: &Vec<String>) -> Vec<(String, String)> {
    let mut projects: Vec<(String, String)> = Vec::new();
    let base = PathBuf::from(settings_dir);

    for path_str in current_val {
        let path_str = path_str.trim();
        if path_str.is_empty() {
            continue;
        }

        let pb = PathBuf::from(path_str);
        let lower = path_str.to_lowercase();
        let is_qgis_file = lower.ends_with(".qgs") || lower.ends_with(".qgz");

        // 絶対パスはそのまま、相対パスは settings_dir 基準で解決
        let effective = if pb.is_absolute() { pb.clone() } else { base.join(&pb) };

        if is_qgis_file {
            // ファイル指定: 存在すれば追加
            if effective.is_file() {
                let actual = effective.to_string_lossy().to_string();
                let display = display_name_for(&actual);
                if !projects.iter().any(|(_, a)| a == &actual) {
                    projects.push((display, actual));
                }
            }
        } else {
            // フォルダ指定: 直下の .qgs/.qgz を列挙
            if effective.is_dir() {
                if let Ok(entries) = fs::read_dir(&effective) {
                    let mut file_entries: Vec<_> = entries.flatten()
                        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                        .filter(|e| {
                            let name = e.file_name().to_string_lossy().to_lowercase();
                            name.ends_with(".qgs") || name.ends_with(".qgz")
                        })
                        .collect();
                    file_entries.sort_by_key(|e| e.file_name());
                    for entry in file_entries {
                        let actual = effective.join(entry.file_name()).to_string_lossy().to_string();
                        let display = display_name_for(&actual);
                        if !projects.iter().any(|(_, a)| a == &actual) {
                            projects.push((display, actual));
                        }
                    }
                }
            }
        }
    }
    projects
}

#[cfg(feature = "gui")]
/// コンボボックスを更新し、プロジェクトの (表示名, 実パス) マッピングを返す
fn update_choices(
    profile_in: &mut misc::InputChoice,
    project_in: &mut misc::InputChoice,
    settings_dir: &str,
    current_profile: &str,
    current_project: &Vec<String>,
) -> Vec<(String, String)> {
    profile_in.clear();
    for p in get_available_profiles(settings_dir, current_profile) {
        profile_in.add(&p);
    }
    project_in.clear();
    let project_map = get_available_projects(settings_dir, current_project);
    for (display, _) in &project_map {
        project_in.add(display);
    }
    project_map
}

#[cfg(feature = "gui")]
fn run_gui(args: Args) {
    let app = app::App::default();

    // --- ウィンドウ ---
    let mut wind = window::Window::new(200, 150, 500, 260, "QGIS Launcher");
    wind.set_color(enums::Color::from_rgb(245, 245, 245));

    // --- タイトル ---
    let mut title = frame::Frame::new(0, 8, 500, 30, "QGIS Launcher");
    title.set_label_size(18);
    title.set_label_color(enums::Color::from_rgb(40, 80, 160));

    // --- ラベル + 入力フィールド (x=20, 各行 y を固定) ---
    let lw = 120; // ラベル幅
    let iw = 340; // 入力幅
    let lx = 20;
    let ix = lx + lw + 8;
    let row_h = 28;

    let y1 = 52;
    let mut profile_label = frame::Frame::new(lx, y1, lw, row_h, "Profile:");
    profile_label.set_align(Align::Right | Align::Inside);
    profile_label.set_label_size(13);
    let mut profile_in = misc::InputChoice::new(ix, y1, iw, row_h, "");

    let y2 = y1 + row_h + 14;
    let mut project_label = frame::Frame::new(lx, y2, lw, row_h, "Project Path:");
    project_label.set_align(Align::Right | Align::Inside);
    project_label.set_label_size(13);
    let mut project_in = misc::InputChoice::new(ix, y2, iw, row_h, "");

    let y3 = y2 + row_h + 14;
    let mut version_label = frame::Frame::new(lx, y3, lw, row_h, "QGIS Version:");
    version_label.set_align(Align::Right | Align::Inside);
    version_label.set_label_size(13);
    let mut version_in = misc::InputChoice::new(ix, y3, iw, row_h, "");

    // --- 区切り線 ---
    let sep_y = y3 + row_h + 14;
    let mut sep = frame::Frame::new(20, sep_y, 460, 2, "");
    sep.set_frame(enums::FrameType::ThinDownBox);

    // --- ステータス ---
    let status_y = sep_y + 8;
    let mut status = frame::Frame::new(20, status_y, 460, 22, "");
    status.set_align(Align::Center | Align::Inside);
    status.set_label_size(12);
    status.set_label_color(enums::Color::from_rgb(100, 100, 100));

    // --- ボタン ---
    let btn_y = status_y + 28;
    let btn_w = 160;
    let btn_h = 36;
    let btn_start = (500 - btn_w) / 2; // 中央配置
    let mut launch_btn = button::Button::new(btn_start, btn_y, btn_w, btn_h, "Launch QGIS");

    wind.end();
    wind.show();

    // initial values
    let settings_dir = args.settings_dir.clone();
    let settings = get_current_settings(&settings_dir);

    let project_map = update_choices(&mut profile_in, &mut project_in, &settings_dir, &settings.profile, &settings.project_path);
    profile_in.set_value(&settings.profile);
    // 初期表示: settings.project_path[0] を絶対パスに解決してマップを検索し、
    // 見つからない場合は display_name_for() で表示名を計算する
    if !settings.project_path.is_empty() {
        let first_raw = &settings.project_path[0];
        let first_pb = PathBuf::from(first_raw);
        let first_effective = if first_pb.is_absolute() {
            first_pb.to_string_lossy().to_string()
        } else {
            PathBuf::from(&settings_dir).join(first_raw).to_string_lossy().to_string()
        };
        let display = project_map.iter()
            // ファイル指定の完全一致
            .find(|(_, actual)| *actual == first_effective)
            .or_else(|| {
                // フォルダ指定の場合: そのフォルダ配下の最初のエントリを前方一致で探す
                let prefix_s = format!("{}/",  first_effective.trim_end_matches(['/', '\\']));
                let prefix_b = format!("{}\\", first_effective.trim_end_matches(['/', '\\']));
                project_map.iter().find(|(_, actual)| {
                    actual.starts_with(&prefix_s) || actual.starts_with(&prefix_b)
                })
            })
            .map(|(d, _)| d.clone())
            // map に存在しない場合（ファイルが存在しない等）は display_name_for で計算
            .unwrap_or_else(|| display_name_for(&first_effective));
        project_in.set_value(&display);
    } else {
        project_in.set_value("");
    }

    let available_versions = get_available_qgis_versions();
    for (name, _) in &available_versions {
        version_in.add(name);
    }
    
    if let Some(exe) = &settings.qgis_executable {
        if let Some((name, _)) = available_versions.iter().find(|(_, path)| path == exe) {
            version_in.set_value(name);
        } else {
            version_in.set_value(exe);
        }
    } else if let Some((name, _)) = available_versions.first() {
        version_in.set_value(name);
    }

    // Launch
    {
        let profile_in = profile_in.clone();
        let project_in = project_in.clone();
        let version_in = version_in.clone();
        let _status = status.clone();
        let available_versions = available_versions.clone();
        // 表示名→実パスのマッピングをクロージャにムーブ
        let project_map = project_map.clone();
        launch_btn.set_callback(move |_| {
            let project_display = project_in.value().unwrap_or_default();
            let profile_val = profile_in.value().unwrap_or_default();
            let version_val = version_in.value().unwrap_or_default();
            
            let exe_path = available_versions.iter()
                .find(|(name, _)| name == &version_val)
                .map(|(_, path)| path.clone())
                .unwrap_or(version_val.clone());

            // 表示名から実パスを解決（手入力の場合はそのまま使用）
            let project_actual = project_map.iter()
                .find(|(display, _)| display == &project_display)
                .map(|(_, actual)| actual.clone())
                .unwrap_or_else(|| project_display.clone());

            // 設定の保存（実パスで保存）
            let mut current = get_current_settings(&settings_dir);
            current.profile = profile_val.clone();
            current.project_path = vec![project_actual.clone()];
            current.qgis_executable = Some(exe_path.clone());
            let _ = save_settings(&settings_dir, &current);

            // Call the launcher with an array of project paths
            launch_qgis(&profile_val, &current.project_path, &settings_dir, &exe_path, &current.rclone_mounts, &current);
            // QGIS 起動後にランチャーを終了
            std::process::exit(0);
        });
    }

    app.run().unwrap();
}

fn main() {
    let args = Args::parse();
    // 起動時の実行ファイルパスとカレントディレクトリをログ出力（デバッグ用）
    match env::current_exe() {
        Ok(p) => println!("DEBUG: current_exe = {:?}", p),
        Err(e) => eprintln!("DEBUG: current_exe 取得失敗: {}", e),
    }
    match env::current_dir() {
        Ok(d) => println!("DEBUG: current_dir (before) = {:?}", d),
        Err(e) => eprintln!("DEBUG: current_dir 取得失敗: {}", e),
    }

    // 実行ファイルのフォルダをカレントディレクトリに設定する
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            if let Err(e) = env::set_current_dir(parent) {
                eprintln!("カレントディレクトリ設定失敗: {}", e);
            } else if let Ok(d) = env::current_dir() {
                println!("DEBUG: current_dir (after) = {:?}", d);
            }
        }

    }

    let settings = get_current_settings(&args.settings_dir);

    // EXE 起動時にドライブ割り当て・robocopy を実行（GUI/CLI 共通）
    mount_rclone_drives(&settings.rclone_mounts, &settings);
    // EXE 起動時にインストール済みQGISバージョンを検出してプロファイルをコピー
    // get_settings_path と同じフォールバックロジックで実際の settings_dir を解決する
    let resolved_settings_dir = {
        let p = get_settings_path(&args.settings_dir);
        p.parent().map(|d| d.to_string_lossy().to_string()).unwrap_or_else(|| args.settings_dir.clone())
    };
    copy_profiles_at_startup(&resolved_settings_dir);

    let profile_to_use = if !settings.profile.trim().is_empty() {
        settings.profile.clone()
    } else if !args.profile.trim().is_empty() {
        args.profile.clone()
    } else {
        "default".to_string()
    };

    #[cfg(feature = "gui")]
    if !args.cli {
        run_gui(args);
        return;
    }

    // CLI 起動
    let qgis_exe = if let Some(exe) = &args.qgis_executable {
        exe.clone()
    } else if let Some(exe) = &settings.qgis_executable {
        exe.clone()
    } else {
        "".to_string()
    };

    println!("起動: プロファイル '{}' でQGISを起動します...", profile_to_use);
    launch_qgis(&profile_to_use, &settings.project_path, &args.settings_dir, &qgis_exe, &settings.rclone_mounts, &settings);
}

fn find_qgis_path_from_registry() -> Option<String> {
    println!("レジストリからQGISのパスを検索中...");
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    let prog_id = match hkcr.open_subkey(r".qgs") {
        Ok(key) => match key.get_value::<String, _>("") {
            Ok(val) => val,
            Err(e) => {
                eprintln!(".qgs の ProgID 取得に失敗: {}", e);
                return None;
            }
        },
        Err(e) => {
            eprintln!(".qgs キーが見つかりません: {}", e);
            return None;
        }
    };

    let cmd_path = format!(r"{}\\shell\\open\\command", prog_id);
    let command_string = match hkcr.open_subkey(&cmd_path) {
        Ok(key) => match key.get_value::<String, _>("") {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{} の既定値取得に失敗: {}", cmd_path, e);
                return None;
            }
        },
        Err(e) => {
            eprintln!("{} キーが見つかりません: {}", cmd_path, e);
            return None;
        }
    };

    let exe_path = if command_string.starts_with('"') {
        command_string.split('"').nth(1).unwrap_or(&command_string)
    } else {
        command_string.split_whitespace().next().unwrap_or(&command_string)
    };

    if exe_path.is_empty() {
        None
    } else {
        Some(exe_path.to_string())
    }
}

/// rclone.exe のパスを解決する。
/// 検索順:
///   1. settings.rclone_exe で明示指定されたパス
///   2. qgis_launcher.exe と同じフォルダ
///   3. settings_dir（C:\qgis_launcher\ 等）
///   4. システム PATH
fn find_rclone_exe(settings: &QgisSettings) -> Option<String> {
    // 1. 明示指定
    if let Some(p) = &settings.rclone_exe {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            println!("rclone: 指定パスを使用: {}", p);
            return Some(p.clone());
        }
        eprintln!("rclone: 指定パス '{}' が見つかりません。他の場所を検索します。", p);
    }

    // 2. qgis_launcher.exe と同じフォルダ
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join("rclone.exe");
            if candidate.is_file() {
                println!("rclone: EXEフォルダから発見: {:?}", candidate);
                return Some(candidate.to_string_lossy().to_string());
            }
        }
    }

    // 3. settings_dir
    if let Some(dir) = &settings.settings_dir {
        let candidate = PathBuf::from(dir).join("rclone.exe");
        if candidate.is_file() {
            println!("rclone: settings_dirから発見: {:?}", candidate);
            return Some(candidate.to_string_lossy().to_string());
        }
    }

    // 4. システム PATH
    if Command::new("rclone").arg("version").output().is_ok() {
        println!("rclone: システムPATHから使用します。");
        return Some("rclone".to_string());
    }

    eprintln!("rclone: rclone.exe が見つかりません。");
    eprintln!("  → qgis_launcher.exe と同じフォルダか C:\\qgis_launcher\\ に rclone.exe を置いてください。");
    eprintln!("  → ダウンロード: https://rclone.org/downloads/");
    None
}

/// パス文字列内の %VAR_NAME% を環境変数値に展開する（展開後の値にさらに %VAR% が含まれる場合も再展開）
fn expand_env_vars(s: &str) -> String {
    let mut result = s.to_string();
    for _ in 0..10 {
        let prev = result.clone();
        let mut output = String::new();
        let mut i = 0;
        while i < result.len() {
            if let Some(start) = result[i..].find('%') {
                let abs_start = i + start;
                output.push_str(&result[i..abs_start]);
                if let Some(end) = result[abs_start + 1..].find('%') {
                    let abs_end = abs_start + 1 + end;
                    let var_name = &result[abs_start + 1..abs_end];
                    let replacement = env::var(var_name)
                        .unwrap_or_else(|_| format!("%{}%", var_name));
                    output.push_str(&replacement);
                    i = abs_end + 1;
                } else {
                    output.push_str(&result[abs_start..]);
                    i = result.len();
                }
            } else {
                output.push_str(&result[i..]);
                break;
            }
        }
        result = output;
        if result == prev {
            break;
        }
    }
    result
}

/// パスエイリアスを適用した後に環境変数展開を行う。
/// "BOX:\\path" など 2文字以上のエイリアス名:path 形式を変換する。
/// エイリアスは settings.path_aliases で定義。
/// "BOX" が未定義の場合のデフォルト: %USERPROFILE%\Box
fn resolve_path(s: &str, aliases: &HashMap<String, String>) -> String {
    // "ALIAS:\\..." または "ALIAS:/..." 形式を検出（2文字以上 = ドライブレターでない）
    let resolved = if let Some(colon_pos) = s.find(':') {
        let prefix = &s[..colon_pos];
        // 単一英字文字（標準ドライブレター）はエイリアスとして扱わない
        if prefix.len() >= 2 && prefix.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            let alias_upper = prefix.to_uppercase();
            let alias_root = if let Some(v) = aliases.get(&alias_upper) {
                v.clone()
            } else if alias_upper == "BOX" {
                // BOX のデフォルト: %USERPROFILE%\Box
                let user_profile = env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\user".to_string());
                format!("{}\\Box", user_profile)
            } else {
                return expand_env_vars(s);
            };
            let rest = &s[colon_pos + 1..];
            let rest = rest.trim_start_matches(['\\', '/']);
            if rest.is_empty() {
                alias_root
            } else {
                format!("{}\\{}", alias_root.trim_end_matches(['\\', '/']), rest)
            }
        } else {
            s.to_string()
        }
    } else {
        s.to_string()
    };
    expand_env_vars(&resolved)
}

/// rclone_mounts の設定に従って rclone マウント / 同期を起動する。
fn mount_rclone_drives(mounts: &[RcloneMount], settings: &QgisSettings) {
    if mounts.is_empty() {
        return;
    }
    // subst モードは rclone 不要なので先に処理する
    for m in mounts {
        if m.mode.as_deref().unwrap_or("subst") == "subst" {
            subst_drive(m, &settings.path_aliases);
        }
    }
    // sync / mount モードは rclone が必要
    let needs_rclone = mounts.iter().any(|m| {
        matches!(m.mode.as_deref().unwrap_or("subst"), "sync" | "mount")
    });
    if !needs_rclone {
        return;
    }
    let rclone_path = match find_rclone_exe(settings) {
        Some(p) => p,
        None => return,
    };
    for m in mounts {
        match m.mode.as_deref().unwrap_or("subst") {
            "sync"  => sync_drive(m, &rclone_path),
            "mount" => mount_drive(m, &rclone_path),
            _ => {}  // subst は上で処理済み
        }
    }
}

/// robocopy でコピー元からローカルキャッシュへミラーリング
fn run_robocopy(src: &str, dst: &str, exclude: &[String], aliases: &HashMap<String, String>) {
    let src = resolve_path(src, aliases);
    let dst = resolve_path(dst, aliases);
    let src = src.as_str();
    let dst = dst.as_str();
    if !PathBuf::from(src).exists() {
        eprintln!("robocopy: コピー元フォルダ '{}' が見つかりません。スキップします。", src);
        return;
    }
    if let Err(e) = fs::create_dir_all(dst) {
        eprintln!("robocopy: コピー先フォルダ作成失敗 ({}): {}", dst, e);
        return;
    }
    println!("robocopy: {} → {} コピー中...", src, dst);
    // /MIR: 完全ミラー（削除も反映）, /MT:8: 並列8スレッド, /R:1 /W:0: リトライ省略, /NP: 進捗表示なし
    let mut cmd = Command::new("robocopy");
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.args([src, dst, "/MIR", "/MT:8", "/R:1", "/W:0", "/NP"]);
    // 除外フォルダ /XD フォルダ名...
    if !exclude.is_empty() {
        cmd.arg("/XD");
        for dir in exclude {
            cmd.arg(dir);
        }
    }
    let status = cmd.status();
    match status {
        // robocopy は成功時も exit code 1〜7 を返すため 8 以上をエラーとする
        Ok(s) => {
            let code = s.code().unwrap_or(-1);
            if code < 8 {
                println!("robocopy: 完了 (exit {})", code);
            } else {
                eprintln!("robocopy: エラー終了 (exit {})", code);
            }
        }
        Err(e) => eprintln!("robocopy 起動エラー: {}", e),
    }
}

/// subst モード: 指定フォルダをドライブに割り当てる（rclone不要・WinFsp不要）
fn subst_drive(m: &RcloneMount, aliases: &HashMap<String, String>) {
    let folder = match &m.local_cache {
        Some(p) => resolve_path(p, aliases),
        None => {
            eprintln!("subst: local_cache の指定が必要です (drive: {})", m.drive);
            return;
        }
    };
    // robocopy_src が指定されていれば subst の前にミラーリング
    if let Some(src) = &m.robocopy_src {
        run_robocopy(src, &folder, &m.robocopy_exclude, aliases);
    }
    let check = if m.drive.ends_with(':') { format!("{}\\" , m.drive) } else { m.drive.clone() };
    if PathBuf::from(&check).exists() {
        println!("subst: {} は既に割り当て済み。スキップします。", m.drive);
        return;
    }
    if !PathBuf::from(&folder).exists() {
        eprintln!("subst: フォルダ '{}' が見つかりません。", folder);
        return;
    }
    match Command::new("subst").creation_flags(CREATE_NO_WINDOW).args([&m.drive, &folder]).status() {
        Ok(s) if s.success() => println!("subst: {} → {} 割り当て完了", m.drive, folder),
        Ok(_)  => eprintln!("subst 失敗: {} → {}", m.drive, folder),
        Err(e) => eprintln!("subst エラー: {}", e),
    }
}

/// sync モード: rclone sync（BOX→ローカル）+ subst（WinFsp不要）
fn sync_drive(m: &RcloneMount, rclone_path: &str) {
    let cache = match &m.local_cache {
        Some(p) => p.clone(),
        None => {
            eprintln!("rclone: mode=sync の場合 local_cache の指定が必要です (drive: {})", m.drive);
            return;
        }
    };
    if let Err(e) = fs::create_dir_all(&cache) {
        eprintln!("キャッシュフォルダ作成失敗 ({}): {}", cache, e);
        return;
    }
    // BOX → ローカルに同期（変更分のみ）
    let remote = match &m.remote {
        Some(r) => r.clone(),
        None => {
            eprintln!("rclone sync: remote の指定が必要です (drive: {})", m.drive);
            return;
        }
    };
    println!("rclone sync: {} → {} 同期中（変更分のみ）...", remote, cache);
    let mut cmd = Command::new(rclone_path);
    cmd.args(["sync", &remote, &cache]);
    match cmd.status() {
        Ok(s) if s.success() => println!("rclone sync: 完了"),
        Ok(_) => eprintln!("rclone sync: 失敗"),
        Err(e) => eprintln!("rclone sync エラー: {}", e),
    }
    // subst でドライブレターを割り当て（既存なら先に解除）
    let check = if m.drive.ends_with(':') { format!("{}\\", m.drive) } else { m.drive.clone() };
    if PathBuf::from(&check).exists() {
        // 既に同じフォルダが割り当て済みなら再割り当て不要
        println!("rclone: {} は既に割り当て済みです。", m.drive);
        return;
    }
    match Command::new("subst").args([&m.drive, &cache]).status() {
        Ok(s) if s.success() => println!("subst: {} → {} 完了", m.drive, cache),
        Ok(_) => eprintln!("subst 失敗: {} → {}", m.drive, cache),
        Err(e) => eprintln!("subst エラー: {}", e),
    }
}

/// mount モード: rclone mount（WinFsp必要）
fn mount_drive(m: &RcloneMount, rclone_path: &str) {
    let check = if m.drive.ends_with(':') { format!("{}\\", m.drive) } else { m.drive.clone() };
    if PathBuf::from(&check).exists() {
        println!("rclone: {} は既にマウント済みです。スキップします。", m.drive);
        return;
    }
    let mut cmd = Command::new(rclone_path);
    let remote = match &m.remote {
        Some(r) => r.clone(),
        None => {
            eprintln!("rclone mount: remote の指定が必要です (drive: {})", m.drive);
            return;
        }
    };
    cmd.args(["mount", &remote, &m.drive, "--no-console"]);
    if m.read_only {
        cmd.arg("--read-only");
    }
    if let Some(v) = &m.vfs_cache_mode { cmd.args(["--vfs-cache-mode", v]); }
    if let Some(v) = &m.vfs_cache_max_age { cmd.args(["--vfs-cache-max-age", v]); }
    if let Some(v) = &m.vfs_cache_max_size { cmd.args(["--vfs-cache-max-size", v]); }
    if let Some(v) = &m.vfs_cache_poll_interval { cmd.args(["--vfs-cache-poll-interval", v]); }
    if let Some(v) = &m.vfs_write_back { cmd.args(["--vfs-write-back", v]); }
    match cmd.spawn() {
        Ok(_) => {
            println!("rclone: {} を {} にマウント開始しました。完了を待機中...", remote, m.drive);
            let mut mounted = false;
            for _ in 0..30 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if PathBuf::from(&check).exists() {
                    println!("rclone: {} のマウント完了。", m.drive);
                    mounted = true;
                    break;
                }
            }
            if !mounted {
                eprintln!("rclone: {} のマウントが30秒以内に完了しませんでした。続行します。", m.drive);
            }
        },
        Err(e) => eprintln!("rclone マウント失敗 ({} -> {}): {}", remote, m.drive, e),
    }
}

/// EXE 起動時: インストール済みQGISのバージョンを検出し、対応するプロファイルフォルダをコピーする
/// profiles\QGIS3\ → APPDATA\QGIS\QGIS3\
/// profiles\QGIS4\ → APPDATA\QGIS\QGIS4\
/// バージョン別フォルダが無い場合は profiles\ 直下を共通フォルダとして使用
fn copy_profiles_at_startup(settings_dir: &str) {
    let base_profiles = PathBuf::from(settings_dir).join("profiles");
    if !base_profiles.exists() {
        return;
    }

    // インストール済みQGISのメジャーバージョンを収集
    let installed = get_available_qgis_versions();
    let mut major_versions: Vec<u32> = installed.iter()
        .filter_map(|(_, exe)| detect_qgis_major_version(exe))
        .collect();
    major_versions.sort();
    major_versions.dedup();

    if major_versions.is_empty() {
        return;
    }

    let all_profile_paths = qgis_launcher::get_qgis_profile_paths();
    for major in &major_versions {
        let target = all_profile_paths.iter()
            .find(|p| p.to_string_lossy().to_lowercase().contains(&format!("qgis{}", major)));
        let target = match target {
            Some(t) => t,
            None => continue,
        };
        // ソース: profiles\QGIS{major}\ があればそちら、なければ profiles\ 直下
        let versioned_src = base_profiles.join(format!("QGIS{}", major));
        let source = if versioned_src.exists() { versioned_src } else { base_profiles.clone() };
        if !source.exists() {
            continue;
        }
        if let Err(_) = fs::create_dir_all(target) {
            continue;
        }
        let _ = copy_dir_contents_skip(&source, target);
    }
}

/// QGISの実行ファイルパスからメジャーバージョン番号（3, 4, ...）を推定する
fn detect_qgis_major_version(qgis_exe: &str) -> Option<u32> {
    // パス中の "QGIS 3." "QGIS 4."、"QGIS3." "QGIS4." などを探す
    let lower = qgis_exe.to_lowercase();
    // "qgis 3" / "qgis3" / "3." などを順に試す
    for major in [4u32, 3u32] {
        let patterns = [
            format!("qgis {}", major),
            format!("qgis{}", major),
            format!("\\{}.\\", major),
        ];
        if patterns.iter().any(|p| lower.contains(p.as_str())) {
            return Some(major);
        }
    }
    None
}

fn launch_qgis(profile_name: &str, project_paths: &Vec<String>, settings_dir: &str, exe_path: &str, _rclone_mounts: &[RcloneMount], _settings: &QgisSettings) {
    // QGISのパスを決定（プロファイルコピーは EXE 起動時に完了済み）
    let qgis_path = if exe_path.is_empty() {
        match find_qgis_path_from_registry() {
            Some(p) => p,
            None => {
                eprintln!("QGISの実行ファイルが見つかりませんでした。レジストリの関連付けを確認してください。");
                return;
            }
        }
    } else {
        exe_path.to_string()
    };

    // Helper to spawn one process with optional project
    let spawn_with_project = |maybe_project: Option<PathBuf>| {
        let mut cmd = Command::new(&qgis_path);
        // QGIS プロセスの作業ディレクトリを実行ファイルのフォルダに設定
        if let Ok(exe_path) = env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                cmd.current_dir(parent);
            }
        }
        cmd.creation_flags(CREATE_NO_WINDOW);
        cmd.arg("--profile").arg(profile_name);
        if let Some(p) = maybe_project {
            if let Some(s) = p.to_str() {
                cmd.arg(s);
            }
        }
        match cmd.spawn() {
            Ok(_) => println!("QGISの起動リクエストに成功しました。"),
            Err(e) => eprintln!("QGISの起動に失敗しました: {}", e),
        }
    };

    if project_paths.is_empty() {
        spawn_with_project(None);
        return;
    }

    for project_path in project_paths {
        let effective_project: Option<PathBuf> = if project_path.trim().is_empty() {
            None
        } else {
            let real_project_path = if project_path.contains(" - ") {
                project_path.replace(" - ", "\\")
            } else {
                project_path.to_string()
            };

            let pb = PathBuf::from(&real_project_path);
            if pb.exists() {
                Some(pb)
            } else {
                let candidate = PathBuf::from(settings_dir).join(&pb);
                if candidate.exists() {
                    Some(candidate)
                } else {
                    if pb.parent().is_none() {
                        if let Some(proj_stem) = pb.file_stem().and_then(|s| s.to_str().map(|s| s.to_string())) {
                            if let Ok(mut exe_path) = env::current_exe() {
                                exe_path.pop();
                                let fallback_candidate = exe_path.join(proj_stem).join(&pb);
                                if fallback_candidate.exists() { Some(fallback_candidate) } else { None }
                            } else { None }
                        } else { None }
                    } else { None }
                }
            }
        };

        spawn_with_project(effective_project);
    }
}



fn copy_dir_contents_skip(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    if !dst.exists() { fs::create_dir_all(dst)?; }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() {
            if !to.exists() { fs::create_dir_all(&to)?; }
            copy_dir_contents_skip(&from, &to)?;
        } else if file_type.is_file() {
            if !to.exists() { fs::copy(&from, &to)?; }
        }
    }
    Ok(())
}

fn get_available_qgis_versions() -> Vec<(String, String)> {
    let mut versions = Vec::new();

    let default_path = find_qgis_path_from_registry();
    let mut default_base_dir = None;

    if let Some(p) = &default_path {
        let pb = PathBuf::from(p);
        let mut current = pb.as_path();
        while let Some(parent) = current.parent() {
            if let Some(name) = current.file_name().and_then(|n| n.to_str()) {
                if name.to_lowercase().starts_with("qgis") {
                    default_base_dir = Some(parent.to_path_buf());
                    break;
                }
            }
            current = parent;
        }
    }

    let mut base_dirs_to_check = Vec::new();
    if let Some(dir) = default_base_dir {
        base_dirs_to_check.push(dir);
    }
    if let Ok(pf) = env::var("ProgramFiles") {
        let pb = PathBuf::from(pf);
        if !base_dirs_to_check.contains(&pb) {
            base_dirs_to_check.push(pb);
        }
    }
    let osgeo4w = PathBuf::from(r"C:\OSGeo4W");
    if !base_dirs_to_check.contains(&osgeo4w) {
        base_dirs_to_check.push(osgeo4w);
    }

    for base_dir in base_dirs_to_check {
        if let Ok(entries) = fs::read_dir(&base_dir) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let folder_path = entry.path();
                        
                        let lower_name = name.to_lowercase();
                        if lower_name.starts_with("qgis") {
                            let bin_dir = folder_path.join("bin");
                            let bat_path = bin_dir.join("qgis.bat");
                            let ltr_bat_path = bin_dir.join("qgis-ltr.bat");
                            let qt6_bat_path = bin_dir.join("qgis-qt6.bat");
                            let exe_path = bin_dir.join("qgis-bin.exe");

                            if bat_path.exists() {
                                versions.push((format!("{} (qgis.bat)", name), bat_path.to_string_lossy().to_string()));
                            }
                            if ltr_bat_path.exists() {
                                versions.push((format!("{} (qgis-ltr.bat)", name), ltr_bat_path.to_string_lossy().to_string()));
                            }
                            if qt6_bat_path.exists() {
                                versions.push((format!("{} (qgis-qt6.bat)", name), qt6_bat_path.to_string_lossy().to_string()));
                            }
                            if !bat_path.exists() && !ltr_bat_path.exists() && !qt6_bat_path.exists() && exe_path.exists() {
                                versions.push((format!("{} (qgis-bin.exe)", name), exe_path.to_string_lossy().to_string()));
                            }
                        } else if lower_name.starts_with("qfield") {
                            let qfield_exe = folder_path.join("usr").join("bin").join("qfield.exe");
                            if qfield_exe.exists() {
                                versions.push((format!("QFieldインストール版 {}", name), qfield_exe.to_string_lossy().to_string()));
                            }
                        }
                    }
                }
            }
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.to_lowercase().starts_with("qgis") {
                            let osgeo4w_root = entry.path().join("qgis");
                            let qgis_ltr_bat = osgeo4w_root.join("bin").join("qgis-ltr.bat");
                            let qgis_bat = osgeo4w_root.join("bin").join("qgis.bat");

                            if qgis_ltr_bat.exists() {
                                versions.push((format!("ポータブル版 {} (LTR)", name), qgis_ltr_bat.to_string_lossy().to_string()));
                            }
                            if qgis_bat.exists() && !qgis_ltr_bat.exists() {
                                versions.push((format!("ポータブル版 {}", name), qgis_bat.to_string_lossy().to_string()));
                            }
                        } else if name.to_lowercase().starts_with("qfield") {
                            let qfield_exe = entry.path().join("usr").join("bin").join("qfield.exe");
                            if qfield_exe.exists() {
                                versions.push((format!("QFieldポータブル版 {}", name), qfield_exe.to_string_lossy().to_string()));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut unique_versions: Vec<(String, String)> = Vec::new();
    for v in versions {
        let p = v.1.to_lowercase();
        if !unique_versions.iter().any(|(_, path)| path.to_lowercase() == p) {
            unique_versions.push(v);
        }
    }

    if let Some(p) = &default_path {
        let mut found = false;
        
        let mut folder_name = String::new();
        let pb = PathBuf::from(p);
        let mut current = pb.as_path();
        while let Some(parent) = current.parent() {
            if let Some(name) = current.file_name().and_then(|n| n.to_str()) {
                let lower = name.to_lowercase();
                if (lower.starts_with("qgis") || lower.starts_with("qfield")) && !lower.ends_with(".bat") && !lower.ends_with(".exe") {
                    folder_name = name.to_string();
                    break;
                }
            }
            current = parent;
        }

        let filename = pb.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let final_display_name = if !folder_name.is_empty() {
            format!("{} ({})", folder_name, filename)
        } else {
            "システム既定のQGIS".to_string()
        };

        for (name, path) in &mut unique_versions {
            if path.to_lowercase() == p.to_lowercase() {
                *name = format!("{} (システム既定)", final_display_name);
                found = true;
                break;
            }
        }
        if !found {
            unique_versions.insert(0, (format!("{} (システム既定)", final_display_name), p.clone()));
        }
    }

    unique_versions
}
