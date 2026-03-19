use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

#[cfg(feature = "gui")]
use fltk::{prelude::*, *};
#[cfg(feature = "gui")]
use fltk::enums::Align;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QgisSettings {
    pub profile: String,
    pub project_path: Vec<String>,
    pub qgis_executable: Option<String>,
    pub reearth_url: Option<String>,
    pub box_url: Option<String>,
    pub settings_dir: Option<String>,
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

fn get_current_settings(custom_dir: &str) -> QgisSettings {
    let path = get_settings_path(custom_dir);
    if let Ok(data) = fs::read_to_string(path) {
        // Accept either string or array for `project_path` for backward compatibility.
        if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&data) {
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
    let p = PathBuf::from(settings_dir).join("profiles");
    if p.exists() {
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
        let mut status = status.clone();
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
            launch_qgis(&profile_val, &current.project_path, &settings_dir, &exe_path);
            status.set_label("Launch requested.");
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
    launch_qgis(&profile_to_use, &settings.project_path, &args.settings_dir, &qgis_exe);
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

fn launch_qgis(profile_name: &str, project_paths: &Vec<String>, settings_dir: &str, exe_path: &str) {
    let source_profiles = PathBuf::from(settings_dir).join("profiles");

    if source_profiles.exists() {
        let profiles_paths = qgis_launcher::get_qgis_profile_paths();
        for profiles_path in &profiles_paths {
            if !profiles_path.exists() {
                if let Err(e) = fs::create_dir_all(&profiles_path) {
                    eprintln!("profiles フォルダ作成失敗 ({:?}): {}", profiles_path, e);
                    continue;
                }
            }

            // 既存 profiles は削除せず、設定側のファイルを上書きせずに追加する
            if let Err(e) = copy_dir_contents_skip(&source_profiles, &profiles_path) {
                eprintln!("profiles のコピーに失敗しました ({:?}): {}", profiles_path, e);
            }
        }
    }

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
                            let qt6_bat_path = bin_dir.join("qgis-qt6.bat");
                            let exe_path = bin_dir.join("qgis-bin.exe");

                            if bat_path.exists() {
                                versions.push((format!("{} (qgis.bat)", name), bat_path.to_string_lossy().to_string()));
                            }
                            if qt6_bat_path.exists() {
                                versions.push((format!("{} (qgis-qt6.bat)", name), qt6_bat_path.to_string_lossy().to_string()));
                            }
                            if !bat_path.exists() && !qt6_bat_path.exists() && exe_path.exists() {
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
