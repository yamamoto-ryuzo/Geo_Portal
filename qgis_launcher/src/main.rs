use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

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
            profile: "default".to_string(),
            project_path: "ProjectFile.qgs".to_string(),
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

fn main() {
    let args = Args::parse();
    // 実行ファイルのフォルダをカレントディレクトリに設定する
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            if let Err(e) = env::set_current_dir(parent) {
                eprintln!("カレントディレクトリ設定失敗: {}", e);
            }
        }
    }


    let settings = get_current_settings(&args.settings_dir);
    let profile_to_use = if !settings.profile.trim().is_empty() && settings.profile != "default" {
        settings.profile.clone()
    } else {
        args.profile.clone()
    };

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
    if let Ok(appdata) = env::var("APPDATA") {
        let source_profiles = PathBuf::from(settings_dir).join("profiles");

        if source_profiles.exists() {
            let bases = ["QGIS3", "QGIS4"];
            for base in &bases {
                let profiles_path = PathBuf::from(&appdata).join("QGIS").join(base).join("profiles");

                if !profiles_path.exists() {
                    if let Err(e) = fs::create_dir_all(&profiles_path) {
                        eprintln!("profiles フォルダ作成失敗 ({}): {}", base, e);
                        continue;
                    }
                }

                // 既存 profiles は削除せず、設定側のファイルを上書きせずに追加する
                if let Err(e) = copy_dir_contents_skip(&source_profiles, &profiles_path) {
                    eprintln!("profiles のコピーに失敗しました ({}): {}", base, e);
                }
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
        let mut def = PathBuf::from(settings_dir);
        def.push("ProjectFiles");
        def.push("ProjectFile.qgs");
        if def.exists() {
            Some(def)
        } else {
            let mut def2 = PathBuf::from(settings_dir);
            def2.push("ProjectFiles");
            def2.push("ProjectFile.qgz");
            if def2.exists() { Some(def2) } else { None }
        }
    } else {
        let pb = PathBuf::from(project_path);
        // 既に存在する絶対/相対パスならそれを使う
        if pb.exists() {
            Some(pb)
        } else {
            // project_path が単なるファイル名（親ディレクトリが無い）なら
            // 実行ファイルと同じ階層にある「実行ファイル名（拡張子前）」フォルダ内を探す
            if pb.parent().is_none() {
                // project_path が単なるファイル名の場合、プロジェクト名のフォルダを探す
                if let Some(proj_stem) = pb.file_stem().and_then(|s| s.to_str().map(|s| s.to_string())) {
                    if let Ok(mut exe_path) = env::current_exe() {
                        exe_path.pop(); // exe のあるフォルダ
                        let candidate = exe_path.join(proj_stem).join(&pb);
                        if candidate.exists() { Some(candidate) } else { None }
                    } else { None }
                } else { None }
            } else { None }
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



fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    if !dst.exists() { fs::create_dir_all(dst)?; }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() { copy_dir_all(&from, &to)?; }
        else if file_type.is_file() { fs::copy(&from, &to)?; }
    }
    Ok(())
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
