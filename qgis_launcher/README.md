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

*(注意: `src/main.rs` 内の `qgis_path` を、お使いの環境の実際のQGISインストールパスに書き換えてからビルドしてください)*

## 使い方

### 1. LGWAN環境向け（直接起動モード）

コマンドライン引数なしで実行するか、ショートカットを作成してダブルクリックすると、そのままQGISを起動します。

```powershell
# そのまま実行 (defaultプロファイルで起動)
.\qgis_launcher.exe

# プロファイルを指定して実行
.\qgis_launcher.exe --profile "LGWAN_Profile"
```

### 2. 一般環境向け（ローカルサーバーモード）

Webポータルから起動要求を受け付けるために、`--server` 引数を付けてバックグラウンドで起動しておきます。

```powershell
.\qgis_launcher.exe --server
```

この状態で、Webポータル（Next.js）から以下のようなJavaScriptを実行するとQGISが立ち上がります。

```javascript
fetch('http://127.0.0.1:12345/launch/qgis', { method: 'POST' });
```
