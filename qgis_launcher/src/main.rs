use axum::{routing::{get, post}, Json, Router};
use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use winreg::enums::*;
use winreg::RegKey;
use mslnk::ShellLink;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QgisSettings {
    pub profile: String,
    pub project_path: String,
}

impl Default for QgisSettings {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            project_path: "".to_string(),
        }
    }
}

fn get_settings_path() -> PathBuf {
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

        // CORS設定（Next.jsからのリクエストを許可）
        let cors = CorsLayer::permissive();
        let app = Router::new()
            .route("/launch/qgis", post(handle_web_request))
            .route("/settings", get(get_settings).post(save_settings))
            .layer(cors);

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
        launch_qgis(&args.profile, "");
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
fn launch_qgis(profile_name: &str, project_path: &str) {
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
    
    if !project_path.trim().is_empty() {
        cmd.arg(project_path.trim());
    }

    match cmd.spawn()
    {
        Ok(_) => println!("QGISの起動リクエストに成功しました。"),
        Err(e) => eprintln!("QGISの起動に失敗しました: {}", e),
    }
}

// Webポータルからのリクエストを受け取った時のハンドラ
async fn handle_web_request() -> &'static str {
    println!("Webポータルからの起動リクエストを受信しました。");
    // 設定ファイルから読み込む
    let settings = get_current_settings();
    launch_qgis(&settings.profile, &settings.project_path);
    "QGIS launched"
}

fn get_current_settings() -> QgisSettings {
    let path = get_settings_path();
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_else(|_| QgisSettings::default())
    } else {
        QgisSettings::default()
    }
}

async fn get_settings() -> Json<QgisSettings> {
    Json(get_current_settings())
}

async fn save_settings(Json(settings): Json<QgisSettings>) -> Json<QgisSettings> {
    let path = get_settings_path();
    if let Ok(data) = serde_json::to_string_pretty(&settings) {
        let _ = fs::write(path, data);
    }
    Json(settings)
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
