use axum::{extract::State, routing::{get, post}, Json, Router};
use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use winreg::enums::*;
use winreg::RegKey;
use mslnk::ShellLink;

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
            // 入力欄のデフォルトを単純なファイル名にする
            project_path: "ProjectFile.qgs".to_string(),
            reearth_url: None,
            box_url: None,
            settings_dir: Some(r"C:\qgis_launcher".to_string()),
        }
    }
}

#[derive(Clone)]
struct AppState {
    settings_dir: Arc<Mutex<String>>,
}

fn get_settings_path(custom_dir: &str) -> PathBuf {
    // ユーザーが指定した（またはデフォルトの）ディレクトリ
    let target_dir = PathBuf::from(custom_dir);
    let target_path = target_dir.join("qgis_settings.json");
    
    // 指定ディレクトリが存在する場合はそこを使う
    if target_dir.exists() {
        return target_path;
    }

    // 存在しない場合は、フォールバックとして実行ファイルと同じディレクトリを使う
    let mut path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop(); // Remove exe name
    path.join("qgis_settings.json")
}

/// QGIS起動用ランチャー
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Webポータルからのリクエストを待機するサーバーモードで起動するかどうか
    #[arg(short, long)]
    server: bool,

    /// スタートアップにサーバーモードで登録する
    #[arg(long)]
    register_startup: bool,

    /// （直接起動時）適用する環境設定プロファイル名
    #[arg(short, long, default_value = "default")]
    profile: String,

    /// 設定ファイル(qgis_settings.json)を配置するディレクトリパス
    #[arg(long, default_value = r"C:\qgis_launcher")]
    settings_dir: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.register_startup {
        register_startup_shortcut();
        return;
    }

    if args.server {
        // --- A: 一般環境向け（ローカルサーバーモード） ---
        println!("サーバーモードで起動します。Webポータルからのリクエストを待機中 (ポート: 12345)...");
        println!("初期設定ディレクトリ: {}", args.settings_dir);

        let state = AppState {
            settings_dir: Arc::new(Mutex::new(args.settings_dir.clone())),
        };

        // CORS設定（Next.jsからのリクエストを許可）
        let cors = CorsLayer::permissive();
        let app = Router::new()
            .route("/launch/qgis", post(handle_web_request))
                .route("/settings", get(get_settings).post(save_settings))
                .route("/open-folder", post(open_folder))
            .layer(cors)
            .with_state(state);

        // Webブラウザでポータルを開く（オンラインURLを試し、ダメならローカルの index.html を開く）
        // ここでは簡単にローカルのファイルを開くフォールバック動作を実装します
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let mut portal_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
            portal_path.pop();
            portal_path.push("out"); // Next.jsのexport先
            portal_path.push("index.html");
            if portal_path.exists() {
                // オフライン環境を想定し、ローカルのポータルを開く
                let _ = Command::new("cmd").args(&["/C", "start", "", portal_path.to_str().unwrap()]).spawn();
            }
        });

        let listener = tokio::net::TcpListener::bind("127.0.0.1:12345").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        // --- B: LGWAN環境向け（直接起動モード） ---
        println!("直接起動モード: プロファイル '{}' でQGISを起動します...", args.profile);
        launch_qgis(&args.profile, "", &args.settings_dir);
    }
}

// レジストリからQGISのパスを自動検索する
fn find_qgis_path() -> Option<String> {
    println!("レジストリからQGISのパスを検索中...");
    
    // HKEY_CLASSES_ROOT にアクセス
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    // 1. .qgs の関連付け先 (ProgID) を取得
    let prog_id = match hkcr.open_subkey(r".qgs") {
        Ok(key) => match key.get_value::<String, _>("") {
            Ok(val) => {
                println!("成功: .qgs のProgIDが見つかりました -> {}", val);
                val
            },
            Err(e) => {
                println!("失敗: .qgs キーの既定値が取得できませんでした -> {}", e);
                return None;
            }
        },
        Err(e) => {
            println!("失敗: .qgs キーがレジストリ(HKEY_CLASSES_ROOT)に見つかりませんでした -> {}", e);
            return None;
        }
    };

    // 2. ProgID から実行ファイルのパスを取得
    let cmd_path = format!(r"{}\shell\open\command", prog_id);
    let command_string = match hkcr.open_subkey(&cmd_path) {
        Ok(key) => match key.get_value::<String, _>("") {
            Ok(val) => {
                println!("成功: 起動コマンドが見つかりました -> {}", val);
                val
            },
            Err(e) => {
                println!("失敗: {} の既定値が取得できませんでした -> {}", cmd_path, e);
                return None;
            }
        },
        Err(e) => {
            println!("失敗: {} キーがレジストリに見つかりませんでした -> {}", cmd_path, e);
            return None;
        }
    };

    // 3. レジストリの値 (例: "C:\Program Files\QGIS\bin\qgis-bin.exe" "%1") からパス部分だけを抽出
    // ダブルクォーテーションで囲まれている場合を考慮
    let exe_path = if command_string.starts_with('"') {
        command_string.split('"').nth(1).unwrap_or(&command_string)
    } else {
        command_string.split_whitespace().next().unwrap_or(&command_string)
    };

    println!("抽出されたQGIS実行ファイルパス: {}", exe_path);

    if exe_path.is_empty() {
        println!("失敗: 実行ファイルパスが空でした。");
        None
    } else {
        Some(exe_path.to_string())
    }
}

// QGISを環境変数を設定して起動する実処理
fn launch_qgis(profile_name: &str, project_path: &str, settings_dir: &str) {
    let qgis_path = match find_qgis_path() {
        Some(path) => path,
        None => {
            eprintln!("エラー: QGISのインストールパスをレジストリ(.qgsの関連付け)から見つけることができませんでした。QGISが正しくインストールされているか確認してください。");
            return;
        }
    };

    println!("QGISを起動しています... パス: {}", qgis_path);

    let mut cmd = Command::new(&qgis_path);
    cmd.arg("--profile").arg(profile_name);

    // Determine project path: fixed default is settings_dir/ProjectFiles/ProjectFile.qgs (or .qgz)
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
        if pb.exists() { Some(pb) } else { None }
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

// Webポータルからのリクエストを受け取った時のハンドラ
async fn handle_web_request(State(state): State<AppState>) -> &'static str {
    println!("Webポータルからの起動リクエストを受信しました。");
    // 設定ファイルから読み込む
    let dir = state.settings_dir.lock().unwrap().clone();
    let settings = get_current_settings(&dir);
    launch_qgis(&settings.profile, &settings.project_path, &dir);
    "QGIS launched"
}

fn get_current_settings(custom_dir: &str) -> QgisSettings {
    let path = get_settings_path(custom_dir);
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_else(|_| QgisSettings::default())
    } else {
        QgisSettings::default()
    }
}

async fn get_settings(State(state): State<AppState>) -> Json<QgisSettings> {
    let dir = state.settings_dir.lock().unwrap().clone();
    let mut settings = get_current_settings(&dir);
    settings.settings_dir = Some(dir.clone());

    // UI の入力欄に表示される project_path の既定値を調整する。
    // フルパス（ドライブ文字や区切り文字を含む）や空文字の場合は
    // 単純なファイル名 `ProjectFile.qgs` を表示する。
    let pp = settings.project_path.trim();
    if pp.is_empty() || pp.contains('\\') || pp.contains('/') || pp.contains(':') {
        settings.project_path = "ProjectFile.qgs".to_string();
    }

    Json(settings)
}

async fn save_settings(State(state): State<AppState>, Json(mut settings): Json<QgisSettings>) -> Json<QgisSettings> {
    let new_dir = settings.settings_dir.clone().unwrap_or_else(|| r"C:\qgis_launcher".to_string());
    
    // 状態を更新
    {
        let mut dir = state.settings_dir.lock().unwrap();
        *dir = new_dir.clone();
    }

    let path = get_settings_path(&new_dir);
    
    // もしそのディレクトリに既にファイルが存在すれば、既存のものを優先して読み込み直す（UIへの反映用）
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(mut existing_settings) = serde_json::from_str::<QgisSettings>(&data) {
                existing_settings.settings_dir = Some(new_dir);
                return Json(existing_settings);
            }
        }
    }

    // ディレクトリが存在しない場合は作成する
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // ファイルが存在しなかった場合は、受け取った設定で新しく作成する
    if let Ok(data) = serde_json::to_string_pretty(&settings) {
        let _ = fs::write(path, data);
    }
    Json(settings)
}

#[derive(Deserialize, Debug)]
struct OpenFolderRequest {
    settings_dir: Option<String>,
}

async fn open_folder(State(state): State<AppState>, Json(req): Json<OpenFolderRequest>) -> Json<serde_json::Value> {
    let mut dir = state.settings_dir.lock().unwrap().clone();
    if let Some(s) = req.settings_dir {
        if !s.trim().is_empty() {
            dir = s;
            // update state
            let mut locked = state.settings_dir.lock().unwrap();
            *locked = dir.clone();
        }
    }

    // Normalize: if it's a path to qgis_settings.json, use parent
    if dir.to_lowercase().ends_with("qgis_settings.json") {
        if let Some(idx) = dir.rfind('\\') {
            dir = dir[..idx].to_string();
        } else if let Some(idx) = dir.rfind('/') {
            dir = dir[..idx].to_string();
        }
    }

    // Ensure directory exists
    let path = PathBuf::from(&dir);
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }

    // Try to open in Explorer (Windows)
    #[cfg(target_os = "windows")]
    {
        let cmd = format!("start \"\" \"{}\"", dir.replace('"', "\""));
        let _ = Command::new("cmd").args(&["/C", &cmd]).spawn();
    }

    Json(serde_json::json!({"ok": true, "settings_dir": dir}))
}

// 自身をスタートアップフォルダに登録する
fn register_startup_shortcut() {
    println!("スタートアップへの登録を開始します...");

    // 現在の実行ファイルのパスを取得
    let current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("実行ファイルのパス取得に失敗しました: {}", e);
            return;
        }
    };

    // スタートアップフォルダのパスを取得
    let startup_dir = match get_startup_folder() {
        Some(path) => path,
        None => {
            eprintln!("スタートアップフォルダのパスが取得できませんでした。");
            return;
        }
    };

    let shortcut_path = startup_dir.join("QGIS_Launcher.lnk");

    // ショートカットを作成
    let mut sl = match ShellLink::new(current_exe.to_str().unwrap()) {
        Ok(link) => link,
        Err(e) => {
            eprintln!("ショートカットの初期化に失敗しました: {}", e);
            return;
        }
    };
    
    // 引数として --server を設定
    sl.set_arguments(Some("--server".to_string()));
    
    // 作業ディレクトリを設定（実行ファイルのあるディレクトリ）
    if let Some(dir) = current_exe.parent() {
        sl.set_working_dir(Some(dir.to_str().unwrap().to_string()));
    }

    match sl.create_lnk(&shortcut_path) {
        Ok(_) => println!("スタートアップに登録しました: {:?}", shortcut_path),
        Err(e) => eprintln!("ショートカットの作成に失敗しました: {}", e),
    }
}

// Windowsのスタートアップフォルダのパスを取得する
fn get_startup_folder() -> Option<PathBuf> {
    if let Ok(appdata) = env::var("APPDATA") {
        let mut path = PathBuf::from(appdata);
        path.push(r"Microsoft\Windows\Start Menu\Programs\Startup");
        if path.exists() {
            return Some(path);
        }
    }
    None
}
