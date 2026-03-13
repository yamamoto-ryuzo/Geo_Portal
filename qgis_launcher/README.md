# QGIS ランチャー (qgis_launcher)

Webポータル連携機能（ローカルサーバー）と、直接起動（LGWAN向け）の両方を兼ね備えたRust製のQGIS起動アプリです。

## 必要な環境
- Rust / Cargo (インストールされていない場合は [rustup](https://rustup.rs/) を使用)

## ビルド方法

コマンドプロンプトやPowerShellでこのフォルダ（`qgis_launcher`）に移動し、以下のコマンドを実行してビルドします。

```powershell
cd qgis_launcher
cargo build --release
```

成功すると、`qgis_launcher\target\release\qgis_launcher.exe` に実行ファイルが作成されます。
この `.exe` ファイル単体で動作するため、LGWAN環境などにコピーして使用できます。


## 使い方

### 1. LGWAN環境向け（直接起動モード）

コマンドライン引数なしで実行するか、ショートカットを作成してダブルクリックすると、そのままQGISを起動します。

```powershell
# そのまま実行 (defaultプロファイルで起動)
.\qgis_launcher.exe

# プロファイルを指定して実行
.\qgis_launcher.exe --profile "LGWAN_Profile"
```

### 2. 一般環境・オフライン環境共通（ローカルサーバーモード）

Webポータルから起動要求や設定を受け付けるために、`--server` 引数を付けてバックグラウンドで起動しておきます。

```powershell
.\qgis_launcher.exe --server
```

**オフライン環境での利用について:**
`--server` 起動時に、同じフォルダにある `out/index.html`（Next.jsのビルド成果物）を自動でブラウザで開きます。
これにより、ネットから分断されたLGWAN環境などでも、モダンなWebポータルのUIからQGISの起動や設定変更が可能です。

### 3. QGIS起動設定の変更（API）

ローカルサーバー起動中は、以下のエンドポイントでQGISの起動プロファイルやプロジェクトファイルパスの設定を共有（取得・保存）できます。
設定内容は `qgis_settings.json` に保存されます。

```javascript
// QGISを起動する
fetch('http://127.0.0.1:12345/launch/qgis', { method: 'POST' });

// 設定を保存する
fetch('http://127.0.0.1:12345/settings', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ profile: "my_profile", project_path: "C:\\path\\to\\project.qgs" })
});
```
