use axum::{routing::post, Router};
use clap::Parser;
use std::process::Command;
use tower_http::cors::CorsLayer;

/// QGIS起動用ランチャー
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Webポータルからのリクエストを待機するサーバーモードで起動するかどうか
    #[arg(short, long)]
    server: bool,

    /// （直接起動時）適用する環境設定プロファイル名
    #[arg(short, long, default_value = "default")]
    profile: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.server {
        // --- A: 一般環境向け（ローカルサーバーモード） ---
        println!("サーバーモードで起動します。Webポータルからのリクエストを待機中 (ポート: 12345)...");

        // CORS設定（Next.jsからのリクエストを許可）
        let cors = CorsLayer::permissive();
        let app = Router::new()
            .route("/launch/qgis", post(handle_web_request))
            .layer(cors);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:12345").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        // --- B: LGWAN環境向け（直接起動モード） ---
        println!("直接起動モード: プロファイル '{}' でQGISを起動します...", args.profile);
        launch_qgis(&args.profile);
    }
}

// QGISを環境変数を設定して起動する実処理
fn launch_qgis(profile_name: &str) {
    // FIXME: 実際のQGISのインストールパスに合わせて変更してください
    let qgis_path = r"C:\Program Files\QGIS 3.34.4\bin\qgis-bin.exe"; 

    println!("QGISを起動しています... パス: {}", qgis_path);

    match Command::new(qgis_path)
        .arg("--profile")
        .arg(profile_name)
        // 必要に応じて環境変数も追加可能
        // .env("QGIS_PREFIX_PATH", r"C:\CustomPath") 
        .spawn()
    {
        Ok(_) => println!("QGISの起動リクエストに成功しました。"),
        Err(e) => eprintln!("QGISの起動に失敗しました: {}", e),
    }
}

// Webポータルからのリクエストを受け取った時のハンドラ
async fn handle_web_request() -> &'static str {
    println!("Webポータルからの起動リクエストを受信しました。");
    // Webからのリクエスト時は、例えば "web_profile" 等を指定して起動
    launch_qgis("web_profile");
    "QGIS launched"
}
