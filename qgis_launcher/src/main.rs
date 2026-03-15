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
    pub project_path: String,
    pub reearth_url: Option<String>,
    pub box_url: Option<String>,
    pub settings_dir: Option<String>,
}

impl Default for QgisSettings {
    fn default() -> Self {
        Self {
            profile: "".to_string(),
            project_path: "".to_string(),
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
        serde_json::from_str(&data).unwrap_or_else(|_| QgisSettings::default())
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
fn get_available_projects(settings_dir: &str, current_val: &str) -> Vec<String> {
    let mut projects = Vec::new();
    if !current_val.is_empty() {
        projects.push(current_val.to_string());
    }
    
    let base_dir = PathBuf::from(settings_dir);
    if let Ok(entries) = fs::read_dir(&base_dir) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_file() {
                    // Check file in Settings Dir directly
                    if let Ok(name) = entry.file_name().into_string() {
                        let lower = name.to_lowercase();
                        if lower.ends_with(".qgs") || lower.ends_with(".qgz") {
                            if !projects.contains(&name) {
                                projects.push(name);
                            }
                        }
                    }
                } else if ft.is_dir() {
                    // Check 1 level deep inside subdirectories
                    if let Ok(dir_name) = entry.file_name().into_string() {
                        if let Ok(sub_entries) = fs::read_dir(entry.path()) {
                            for sub_entry in sub_entries.flatten() {
                                if let Ok(sub_ft) = sub_entry.file_type() {
                                    if sub_ft.is_file() {
                                        if let Ok(name) = sub_entry.file_name().into_string() {
                                            let lower = name.to_lowercase();
                                            if lower.ends_with(".qgs") || lower.ends_with(".qgz") {
                                                // FLTK のコンボボックスで `/` はサブメニュー判定されるため、見えやすい別の文字「 - 」を使う
                                                let relative_path = format!("{} - {}", dir_name, name);
                                                if !projects.contains(&relative_path) {
                                                    projects.push(relative_path);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    projects
}

#[cfg(feature = "gui")]
fn update_choices(
    profile_in: &mut misc::InputChoice,
    project_in: &mut misc::InputChoice,
    settings_dir: &str,
    current_profile: &str,
    current_project: &str,
) {
    profile_in.clear();
    for p in get_available_profiles(settings_dir, current_profile) {
        profile_in.add(&p);
    }
    project_in.clear();
    for p in get_available_projects(settings_dir, current_project) {
        project_in.add(&p);
    }
}

#[cfg(feature = "gui")]
fn run_gui(mut args: Args) {
    let app = app::App::default();

    // --- ウィンドウ ---
    let mut wind = window::Window::new(200, 150, 500, 220, "QGIS Launcher");
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

    // --- 区切り線 ---
    let sep_y = y2 + row_h + 14;
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

    update_choices(&mut profile_in, &mut project_in, &settings_dir, &settings.profile, &settings.project_path);
    profile_in.set_value(&settings.profile);
    project_in.set_value(&settings.project_path);

    // Launch
    {
        let profile_in = profile_in.clone();
        let project_in = project_in.clone();
        let mut status = status.clone();
        launch_btn.set_callback(move |_| {
            let project_val = project_in.value().unwrap_or_default();
            let profile_val = profile_in.value().unwrap_or_default();
            
            // GUIから起動ボタンを押された場合は CLI 処理をそのまま実行するのと同様に launch_qgis を呼ぶ
            launch_qgis(&profile_val, &project_val, &settings_dir);
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
    println!("起動: プロファイル '{}' でQGISを起動します...", profile_to_use);
    launch_qgis(&profile_to_use, &settings.project_path, &args.settings_dir);
}

fn find_qgis_path() -> Option<String> {
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

fn launch_qgis(profile_name: &str, project_path: &str, settings_dir: &str) {
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

    let qgis_path = match find_qgis_path() {
        Some(p) => p,
        None => {
            eprintln!("QGISの実行ファイルが見つかりませんでした。レジストリの関連付けを確認してください。");
            return;
        }
    };

    let mut cmd = Command::new(&qgis_path);
    // QGIS プロセスの作業ディレクトリを実行ファイルのフォルダに設定
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            cmd.current_dir(parent);
        }
    }
    cmd.arg("--profile").arg(profile_name);

    let effective_project: Option<PathBuf> = if project_path.trim().is_empty() {
        None // 空文字の場合はQGISをプロジェクト無しで起動する
    } else {
        // "FolderName - FileName.qgs" 形式で選ばれた場合、もとの相対パスに復元する
        let real_project_path = if project_path.contains(" - ") {
            project_path.replace(" - ", "\\")
        } else {
            project_path.to_string()
        };

        let pb = PathBuf::from(&real_project_path);
        // 既に存在する絶対/相対パスならそれを使う
        if pb.exists() {
            Some(pb)
        } else {
            // パスが見つからない場合は Settings Dir を基準にした相対パスとして評価する
            let candidate = PathBuf::from(settings_dir).join(&pb);
            if candidate.exists() {
                Some(candidate)
            } else {
                // 古いロジックのフォールバック (実行ファイルと同じ階層にある「ファイル名と同名のフォルダ」内を探す)
                if pb.parent().is_none() {
                    if let Some(proj_stem) = pb.file_stem().and_then(|s| s.to_str().map(|s| s.to_string())) {
                        if let Ok(mut exe_path) = env::current_exe() {
                            exe_path.pop(); // exe のあるフォルダ
                            let fallback_candidate = exe_path.join(proj_stem).join(&pb);
                            if fallback_candidate.exists() { Some(fallback_candidate) } else { None }
                        } else { None }
                    } else { None }
                } else { None }
            }
        }
    };

    if let Some(p) = effective_project {
        if let Some(s) = p.to_str() {
            cmd.arg(s);
        }
    }

    match cmd.spawn() {
        Ok(_) => println!("QGISの起動リクエストに成功しました。"),
        Err(e) => eprintln!("QGISの起動に失敗しました: {}", e),
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
