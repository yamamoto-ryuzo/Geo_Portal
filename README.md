# ReEarth_Portal　バージョン: 1.0.0

このリポジトリでは「左パネルは操作／コントロール、右パネルは表示／詳細」を原則とするマスタ詳細パターンを採用しています。

## 目的
- 左側の簡潔な操作領域で設定・絞り込みを行い、右側で対象コンテンツ（Re:Earth 等）を同時に確認・操作できるUXを実現する。

## 主要設計

- 左パネル: クイックアクション、プリセット、表示切替などのコントロールを配置。画面幅に合わせて折り畳みやリサイズを可能にする。
- 右パネル: タブで複数のビューを切り替え（`reearth`, `googlemap`, `settings`）。各タブは共通の状態を参照して表示を更新する。
- 状態管理: `src/PortalContext.js` を導入。`reearthUrl` をコンテキストで管理し、設定は `portal:settings`（localStorage）へ統一して永続化・復元します。

## 設定ワークフロー（現在のプロトタイプ）

1. `設定` タブで URL を編集（プレビューは即時適用可能）。
2. 「保存」ボタンで localStorage に永続化。
3. 「初期化」でデフォルト値に戻す。

## 起動時クエリによる初期値指定

- 起動 URL に `EARTH` または `earth` クエリを付けると、`Re:Earth` の初期表示 URL として読み込みます。
- 優先順位は **クエリ (`EARTH` / `earth`) > localStorage 保存値 > デフォルト値** です。
- 例: `https://re-earth-portal.vercel.app/?EARTH=https%3A%2F%2Fomoya.visualizer.reearth.io%2F`

実装ファイル:

- [pages/index.js](pages/index.js#L1-L400) — UI（左パネル、右パネル、設定パネル）、iframe 表示制御
- [src/PortalContext.js](src/PortalContext.js#L1-L200) — URL 状態・プレビュー・保存ロジック

## セキュリティと通信

- 外部サイト埋め込み（iframe）と通信する場合、オリジン検証付きの `postMessage` を利用する設計を推奨します。必要に応じて `sandbox` 属性で権限を限定してください。

## パフォーマンス

- iframe インスタンスは常にマウントされ、タブ切替は CSS の `display` で行って表示/非表示を切り替えます。これにより iframe 内の状態（地図の位置やログイン状態など）を保持します。`loading="lazy"` は併用しています。
- 設定入力のデバウンスやプリフェッチ制御で過剰なロードを避けることを推奨します。

## UX 改善案（優先度順）

1. プリセット保存／読み込み（URL のセットを名前付きで保存）。
2. クイック履歴（最近使った URL）を表示。
3. 左パネルの折り畳み／リサイズ、ショートカットキー対応。
4. 保存前に「プレビュー → 確定」ワークフローを明確に（誤反映回避）。

## 運用とテスト

- ブラウザの localStorage に依存するため、プライベートモードやブラウザポリシーでの動作を確認してください。
- 変更後は `npm run dev`（Next.js 開発サーバ）で動作確認してください。

## 次の試作候補

1. `postMessage` を用いた iframe 双方向通信の雛形追加
2. 設定の共有（URL クエリや短縮リンク生成）
3. プリセット UI とエクスポート/インポート機能

作業で追加・編集した主なファイル: [pages/index.js](pages/index.js#L1-L400)、[src/PortalContext.js](src/PortalContext.js#L1-L200)

---

# QGIS Launcher (qgis_launcher)



- 概要: 起動設定（プロファイル / プロジェクトパス / QGIS Version）に基づいて QGIS / QField を起動するランチャーです。設定は `qgis_settings.json`（JSON 構造は `QgisSettings`）で保持できます。
- `project_path` の配列対応:
	- `qgis_settings.json` の `project_path` は「起動候補プロジェクトの一覧」です。文字列と配列の両方を読み込めますが、保存や新しい設定では配列を推奨します。
	- **GUI での選択・起動時に `project_path` は書き換えられません。** 選択したプロジェクトは `current_project` フィールドに保存されます。
	- 各エントリの解釈ルール:
		1. **ファイル指定**（拡張子 `.qgs` / `.qgz`）: そのファイルが存在すれば候補に追加します。
		2. **フォルダ指定**（拡張子なし）: そのフォルダ直下にある `.qgs` / `.qgz` を列挙して候補に追加します。
		3. **パス解決の優先順位**: 絶対パスはそのまま使用し、相対パスは 設定基準フォルダ（デフォルトはEXEと同じフォルダ）を基準に解決します。
	- GUI での表示名は常に **「直近1階層上のフォルダ名 - ファイル名」** 形式（例: `ProjectFiles - ProjectFile.qgs`）に統一されます。ルート直下のファイルはドライブ文字を親名として表示します（例: `C: - ProjectFile.qgs`）。

- `current_project` フィールド（カレントプロジェクト）:
	- GUI で「Launch QGIS」を押すと、選択したプロジェクトの絶対パスが `current_project` に保存されます。
	- 次回起動時は `current_project` を優先してプロジェクトの初期選択を復元します。`current_project` が未設定の場合は `project_path[0]` の先頭エントリをデフォルト選択します。
	- `project_path`（候補一覧）は変更されないため、JSON を編集せずにプロジェクト一覧を維持したまま、前回の選択状態を再現できます。
	- 設定例:
		```json
		{
		  "project_path": [
		    "Q:\\ProjectFiles"
		  ],
		  "current_project": "Q:\\ProjectFiles\\ProjectFile.qgs"
		}
		```

- `qgis_settings.json` のバックスラッシュ自動修正:
	- Windows のエクスプローラからコピーした単一バックスラッシュのパス（例: `"C:\ProjectFiles2"`）が JSON に記載されていても、ランチャー起動時に自動的に `\\` へ修正してから読み込みます。
	- これにより、JSON の正式仕様（`\\` を使う）に準拠していないファイルでも正常に動作します。
	- 有効な JSON エスケープ（`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\u`）はそのまま保持されます。
	- 例:
		```json
		"project_path": [
		  "C:\github\\ReEarth_Portal\qgis_launcher\download\ProjectFiles",
		  "C:\ProjectFiles2"
		]
		```
		上記は読み込み時に自動的に正規化されます。

- GUI: デフォルトでは簡易 GUI（FLTK）を起動します。GUI はプルダウンでプロファイル、プロジェクト、QGIS バージョンを選択して「Launch QGIS」で起動します。プロファイル候補は `%APPDATA%` 下の QGIS プロファイルパス（`get_qgis_profile_paths()`）と、設定基準フォルダ配下の `profiles` フォルダを参照します。プロジェクト候補は `project_path` に指定されたファイル／フォルダを上記ルールで解決して列挙します。QGIS / QField の実行ファイル候補はシステム上の標準インストール箇所やポータブル配布を探索して自動検出します。

- **QGIS バージョン別プロファイル配信**:
	- 設定基準フォルダ配下の `profiles` フォルダに `QGIS3\` / `QGIS4\` サブフォルダを作成することで、インストール済みの QGIS バージョンごとに専用プロファイルを自動配信できます。
	- **コピーのタイミング**: EXE 起動時に自動実行されます（「Launch QGIS」ボタン押下前）。コピー先にファイルが既に存在する場合はスキップされるため、複数回起動しても無駄なコピーは発生しません。
	- **バージョン自動検出**: システムにインストールされている全 QGIS を自動検出し（`ProgramFiles`、`C:\OSGeo4W`、レジストリ等を探索）、検出したメジャーバージョンごとにコピーを行います。QGIS 3 と QGIS 4 が両方インストールされている場合は両方に同時にコピーされます。
	- バージョンの判定はインストールフォルダ名から行います（例: `QGIS 3.44.8` → バージョン 3）。
	- `QGIS3\` / `QGIS4\` フォルダが存在しない場合は `profiles\` 直下（共通）を全バージョンに使用します。
	- フォルダ構成例:
		```
		(設定基準フォルダ)/
		  profiles/
		    QGIS3/
		      geo_custom/    ← QGIS 3.x 用プロファイル → %APPDATA%\QGIS\QGIS3\ へコピー
		    QGIS4/
		      geo_custom/    ← QGIS 4.x 用プロファイル → %APPDATA%\QGIS\QGIS4\ へコピー
		```
	- バージョン検出の判定順位: パス中に `QGIS 4` / `QGIS4` → バージョン 4。`QGIS 3` / `QGIS3` → バージョン 3。検出不能な場合は `profiles\` 直下を使用。
	- **注意**: コピー先に既存ファイルがある場合は上書きしません（ユーザーの設定変更を保護）。配信ファイルを更新してユーザー側に反映させたい場合は、コピー先の該当ファイルを削除してから再起動してください。

- CLI モード: `--cli` を指定すると GUI を起動せずに CLI モードで動作します。

---

## ユーザーロール制御

GUI の「User Role」ドロップダウンで `Viewer` / `Editor` / `Administrator` を選択すると、QGIS の UI がロールに応じて制限されます。

### 仕組み

QGIS 起動時に以下の 3 つの引数が自動で設定されます：

| 引数 | ファイル | 内容 |
|---|---|---|
| `--customizationfile` | `ini/<role>.xml`（QGIS4） または `ini/<role>.ini`（QGIS3） | ランチャーが QGIS 実行ファイルパスからメジャーバージョンを判定し、QGIS4 では `.xml`（Customization XML）を、QGIS3 では `.ini` をそれぞれ必ず渡します（フォールバックは行いません）。 |
| `--globalsettingsfile` | `ini/qgis_global_settings.ini` | `userrole` を QGIS グローバル変数として設定（プロジェクト式 `@userrole` で参照可能）。ランチャーはこのファイルを生成して渡します。 |
| `--code` | `ini/startup.py` | 起動後に動的制御（ツールバー表示・レイヤーのReadOnly設定）|

```
qgis.exe --profile geo_custom
         --customizationfile   ini/Viewer.xml  # QGIS4 の場合
         --customizationfile   ini/Viewer.ini  # QGIS3 の場合
         --globalsettingsfile  ini/qgis_global_settings.ini
         --code                ini/startup.py
```

ランチャーは子プロセスの環境に `PORTAL_USERROLE` を注入します（`launch_qgis` が `cmd.env("PORTAL_USERROLE", role)` を設定）。`startup.py` は QGIS グローバル変数 `userrole` を参照し、存在しなければ `PORTAL_USERROLE` を参照するため、両者を併用することで確実にロール情報を伝達します。

### ロールごとの制御内容

| ロール | カスタマイズファイル（QGIS4 / QGIS3） | UI 制御 | レイヤー |
|---|---|---|---|
| **Viewer** | `Viewer.xml`（QGIS4 / 推奨） / `Viewer.ini`（QGIS3） | 編集メニュー・編集ツールバーを非表示 | プロジェクト読み込み時に全ベクターレイヤーを ReadOnly に設定 |
| **Editor** | `Editor.xml`（QGIS4） / `Editor.ini`（QGIS3） | プロジェクト新規作成・保存メニューを非表示（編集操作は許可） | 制限なし |
| **Administrator** | `Administrator.xml`（QGIS4） / `Administrator.ini`（QGIS3） | 制限なし | 制限なし |

基本は QGIS4 を想定しており、ランチャーは QGIS の実行ファイルパスからメジャーバージョンを判定して、可能な限り QGIS4 用の `*.xml`（Customization XML）を渡します。QGIS3 を使用する環境では `*.ini` を渡します。現在の実装ではメジャーバージョンに応じて必ず適切な拡張子を使用し、フォールバックは行いません。

### ファイル構成

```
qgis_launcher.exe と同階層/
  ini/
    Viewer.xml                  ← QGIS4 用 Customization XML（推奨）
    Viewer.ini                  ← QGIS3 用 INI フォーマット（互換）
    Editor.xml
    Editor.ini
    Administrator.xml
    Administrator.ini
    qgis_global_settings.ini    ← 起動のたびに自動生成（userrole を記録）
    startup.py                  ← QGIS 起動後に自動実行されるスクリプト
```

> **注意**: `ini/` フォルダが実行ファイルと同階層に存在しない場合、ロール制御はスキップされます（エラーにはなりません）。

### startup.py の動作タイミング

| タイミング | 処理 |
|---|---|
| 起動後 500ms | UI 制御（ツールバー表示/非表示、ボタンの有効/無効）|
| プロジェクト読み込みのたびに | Viewer の場合、全ベクターレイヤーを ReadOnly に設定 |

- 設定ファイルの探索と優先度:
	- ランチャー実行時の設定基準フォルダは **`qgis_launcher.exe` と同じディレクトリ** になります。
	- ※以前のバージョンで存在した `C:\qgis_launcher` への固定フォールバックや、JSON内部での絶対パス指定は廃止され、常に「EXEの配置場所」ベースでポータブルに動作するようになりました。

- プロファイル選択の優先順位:
	1. `qgis_settings.json` の `profile` が空でない場合はそれを使用
	2. そうでなければ CLI の `--profile` 引数
	3. それでも無ければ `default` を使用

- QGIS 実行ファイルパスの優先順位（重要）:
	1. CLI 引数 `--qgis_executable`（明示指定）
	2. `qgis_settings.json` の `qgis_executable` フィールド
	3. レジストリやファイル関連付けからの自動検出（`find_qgis_path_from_registry()`）
	4. 見つからない場合はエラーで終了します（実装上、レジストリ検出に失敗すると起動を中止します）。

- ビルド・実行例:

```powershell
cd qgis_launcher
cargo build --release
# ランチャー（デフォルトは GUI を起動）
cargo run --release
# CLI モードで即座に起動
cargo run --release -- --cli
``` 

- 備考: `Cargo.toml` は `fltk-bundled` を feature として利用する設定（`gui` feature）になっており、FLTK をソースからバンドルビルドします。初回ビルドは時間がかかる点に注意してください。

---

## クラウドドライブ自動割り当て（rclone_mounts）

QGIS 起動前に任意のフォルダをドライブレターへ自動割り当てする機能です。
追加インストール不要の `subst` モードを採用しています。

**対応クラウドストレージ**: BOX Drive / OneDrive / Google Drive for Desktop など、ローカルフォルダとして同期されるすべてのクラウドストレージに対応します。

### ドライブ構成と役割（BOX の例）

```
BOX Drive (BOX:\Geo_Portal = %USERPROFILE%\Box\Geo_Portal)
    │
    │  robocopy /MIR（起動時に自動実行）
    │  ← BOX → ローカルの一方向コピー
    ▼
ローカルキャッシュ (C:\qgis_cache\master)
    │
    │  subst
    ▼
  Q:  参照用・高速読み取り専用

BOX Drive (BOX:\Geo_Portal = %USERPROFILE%\Box\Geo_Portal)
    │
    │  subst
    ▼
  R:  編集用・BOX Drive アプリが BOX へ自動同期
```

| ドライブ | 書き込み先 | BOX への反映 | 用途 |
|---|---|---|---|
| Q: | ローカル SSD | **されない** | QGIS 高速参照専用。起動時に robocopy でキャッシュを最新化 |
| R: | BOX Drive フォルダ | BOX Drive アプリが自動同期 | データ編集・保存 |

QGIS プロジェクトファイル（.qgs）のデータソースパスを `Q:\data\道路.gpkg` のように
ドライブレターで統一でき、チーム全員が同じ `.qgs` ファイルを共有できます。

> **注意**: Q: に直接保存したデータは BOX へ反映されません。編集・保存は R: を使用してください。

### qgis_settings.json の設定例

```json
{
  "path_aliases": {
    "BOX": "%USERPROFILE%\\Box",
    "OneDrive": "%OneDrive%",
    "OneDriveBiz": "%OneDriveCommercial%",
    "GoogleDrive": "G:\\マイドライブ"
  },
  "rclone_mounts": [
    {
      "drive": "Q:",
      "mode": "subst",
      "local_cache": "C:\\qgis_cache\\master",
      "robocopy_src": "BOX:\\Geo_Portal",
      "robocopy_exclude": ["secret-folder", "private-data"]
    },
    {
      "drive": "R:",
      "mode": "subst",
      "local_cache": "BOX:\\Geo_Portal"
    }
  ]
}
```

| フィールド | 説明 |
|---|---|
| `drive` | 割り当て先ドライブレター（例: `Q:`） |
| `mode` | `subst`（省略時は `subst`） |
| `local_cache` | 割り当て元フォルダのパス（必須。`BOX:\\path` ・ `%VAR%` 記法対応） |
| `robocopy_src` | 指定時は起動時に `robocopy /MIR` で `local_cache` へミラーリング（`BOX:\\path` ・ `%VAR%` 記法対応）。**コピー方向は `robocopy_src` → `local_cache` の一方向** |
| `robocopy_exclude` | robocopy の除外サブフォルダ名の配列（例: `["secret-folder", "private-data"]`）|

`path_aliases` ではエイリアス名（2文字以上）をパスにマッピングできます。`BOX` は未定義時のデフォルトで `%USERPROFILE%\Box` に解決されます。OneDrive / Google Drive 等も同様に定義できます。

| 記法例 | 展開後 |
|---|---|
| `BOX:\Geo_Portal` | `C:\Users\<ユーザ>\Box\Geo_Portal`（`BOX` エイリアス展開） |
| `OneDrive:\Documents` | `C:\Users\<ユーザ>\OneDrive\Documents`（`%OneDrive%` 展開） |
| `OneDriveBiz:\Geo_Portal` | `C:\Users\<ユーザ>\OneDrive - 会社名\Geo_Portal`（`%OneDriveCommercial%` 展開） |
| `GoogleDrive:\Geo_Portal` | `G:\マイドライブ\Geo_Portal`（Google Drive のドライブレターを指定） |
| `%USERPROFILE%\Box\Geo_Portal` | `C:\Users\<ユーザ>\Box\Geo_Portal` |
| `C:\qgis_cache\master` | そのまま |

### 事前準備

**使用するクラウドストレージに応じて以下を設定してください。**

| クラウド | 準備 | エイリアスの基準 |
|---|---|---|
| **BOX Drive** | BOX Drive アプリをインストール、対象フォルダをオフライン同期（常にこのデバイス上に保持）に設定 | `%USERPROFILE%\Box`（デフォルト、または `path_aliases` で変更） |
| **OneDrive（個人）** | OneDrive アプリが動作していれば `%OneDrive%` 環境変数が自動設定される | `%OneDrive%` |
| **OneDrive（法人/Microsoft 365）** | サインイン後に `%OneDriveCommercial%` 環境変数が自動設定される | `%OneDriveCommercial%` |
| **Google Drive for Desktop** | Google Drive for Desktop をインストール、ドライブレター（例: `G:`）を確認 | `G:\マイドライブ`（ドライブレターは環境により異なるため `path_aliases` で指定） |

- Q: の `local_cache`（`C:\qgis_cache\master` 等）は初回起動時に自動作成されます。
- OneDrive は会社名がパスに含まれる（例: `OneDrive - 株式会社XXX`）ため、ユーザーごとにパスが異なる場合があります。その場合は[ユーザーオーバーライド](#ユーザーごとの設定オーバーライドqgis_settingsusername.json)で `path_aliases` を上書きしてください。

---

## ユーザーごとの設定オーバーライド（qgis_settings_{USERNAME}.json）

BOX のフォルダ階層はユーザーによって異なる場合があります。`qgis_settings.json` と同じディレクトリに `qgis_settings_{Windowsログイン名}.json` を置くと、全員共通のベース設定を上書きできます。

### ファイル命名規則

```
C:\qgis_launcher\
  qgis_settings.json              ← 全員共通のベース設定
  qgis_settings_yamamoto.json     ← yamamoto ユーザーのみ上書き
  qgis_settings_tanaka.json       ← tanaka ユーザーのみ上書き
```

`{USERNAME}` は Windows の `%USERNAME%` 環境変数（ログインユーザー名）と一致させてください。

### マージ動作

| キー | マージ方式 |
|---|---|
| `rclone_mounts` | `drive` キーで既存エントリを照合してフィールド単位で上書き（未指定フィールドはベースを維持） |
| `path_aliases` | マップキー単位でマージ（未指定キーはベースを維持） |
| その他のキー | 値ごと置き換え |

### オーバーライドファイルの例

`robocopy_src` だけを変えたい場合（他のフィールドはベースのまま）:

```json
{
  "rclone_mounts": [
    {
      "drive": "Q:",
      "robocopy_src": "BOX:\\MyFolder\\Geo_Portal"
    }
  ]
}
```

`path_aliases` の BOX パスだけをユーザーごとに変えたい場合:

```json
{
  "path_aliases": {
    "BOX": "D:\\Box"
  }
}
```

サンプルファイル: [qgis_launcher/download/qgis_settings_USERNAME.json.example](qgis_launcher/download/qgis_settings_USERNAME.json.example)

---

## 全ユーザー強制オーバーライド（qgis_settings_override.json）

`qgis_settings.json` と同じディレクトリに `qgis_settings_override.json` を置くと、**ユーザー名に関係なくすべてのユーザーに対して**設定を強制上書きできます。

適用順序は次のとおりです：

```
qgis_settings.json              ①ベース設定
  ↓
qgis_settings_{USERNAME}.json   ②ユーザー個別オーバーライド（任意）
  ↓
qgis_settings_override.json     ③全ユーザー強制オーバーライド（任意）← 最終適用
```

③が最後に適用されるため、ユーザー個別設定（②）よりも優先されます。

### ファイル配置例

```
C:\qgis_launcher\
  qgis_settings.json              ← 全員共通のベース設定
  qgis_settings_yamamoto.json     ← yamamoto ユーザーのみ上書き（任意）
  qgis_settings_override.json     ← 全ユーザーに強制適用（任意）
```

### マージ動作

マージルールはユーザーオーバーライドと同じです：

| キー | マージ方式 |
|---|---|
| `rclone_mounts` | `drive` キーで既存エントリを照合してフィールド単位で上書き（未指定フィールドは維持） |
| `path_aliases` | マップキー単位でマージ（未指定キーは維持） |
| その他のキー | 値ごと置き換え |

### 用途例

特定の `profile` を全員に強制したい場合：

```json
{
  "profile": "corporate_profile"
}
```

特定のドライブマウントを全員に強制追加したい場合（他のフィールドはベースのまま）:

```json
{
  "rclone_mounts": [
    {
      "drive": "S:",
      "mode": "subst",
      "local_cache": "C:\\shared_data"
    }
  ]
}
```

サンプルファイル: [qgis_launcher/download/qgis_settings_override.json.example](qgis_launcher/download/qgis_settings_override.json.example)

---

## ライセンス

### このリポジトリ（ReEarth_Portal / qgis_launcher）

MIT License — 詳細は [LICENSE](LICENSE) ファイルを参照してください（未作成の場合は MIT として扱います）。

### 同梱・依存ソフトウェアのライセンス

| ソフトウェア | ライセンス | 配布 | 備考 |
|---|---|---|---|
| **FLTK** (fltk-rs) | LGPL v2 + 例外条項 | 同梱可 | スタティックリンク時も再配布可 |
| **QGIS** | GPL v2 以降 | 別途インストール | qgis_launcher とは独立したソフトウェア |

### 同梱配布する場合のパッケージ構成

```
配布パッケージの構成例:
  qgis_launcher.exe
  qgis_settings.json
```
## 免責事項

本システムは個人のPCで作成・テストされたものです。  
ご利用によるいかなる損害も責任を負いません。  

<p align="center">
  <a href="https://giphy.com/explore/free-gif" target="_blank">
    <img src="https://github.com/yamamoto-ryuzo/QGIS_portable_3x/raw/master/imgs/giphy.gif" width="500" title="avvio QGIS">
  </a>
</p>



