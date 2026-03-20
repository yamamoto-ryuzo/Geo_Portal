# ReEarth_Portal　バージョン: 0.12.1

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
	- `qgis_settings.json` の `project_path` は文字列と配列の両方を読み込めますが、保存や新しい設定では配列を推奨します。
	- 配列に複数のパスを指定すると、ランチャーは配列要素ごとに QGIS の起動リクエストを行います（GUI からは単一選択を行い、保存時に配列へ変換して保存します）。
	- 各エントリの解釈ルール:
		1. **ファイル指定**（拡張子 `.qgs` / `.qgz`）: そのファイルが存在すれば候補に追加します。
		2. **フォルダ指定**（拡張子なし）: そのフォルダ直下にある `.qgs` / `.qgz` を列挙して候補に追加します。
		3. **パス解決の優先順位**: 絶対パスはそのまま使用し、相対パスは `settings_dir` を基準に解決します。
	- GUI での表示名は常に **「直近1階層上のフォルダ名 - ファイル名」** 形式（例: `ProjectFiles - ProjectFile.qgs`）に統一されます。ルート直下のファイルはドライブ文字を親名として表示します（例: `C: - ProjectFile.qgs`）。

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

- GUI: デフォルトでは簡易 GUI（FLTK）を起動します。GUI はプルダウンでプロファイル、プロジェクト、QGIS バージョンを選択して「Launch QGIS」で起動します。プロファイル候補は `%APPDATA%` 下の QGIS プロファイルパス（`get_qgis_profile_paths()`）と、`settings_dir` 配下の `profiles` フォルダを参照します。プロジェクト候補は `project_path` に指定されたファイル／フォルダを上記ルールで解決して列挙します。QGIS / QField の実行ファイル候補はシステム上の標準インストール箇所やポータブル配布を探索して自動検出します。

- CLI モード: `--cli` を指定すると GUI を起動せずに CLI モードで動作します。

- 設定ファイルの探索と優先度:
	- 指定された `--settings_dir` が存在する場合、まずそのディレクトリ内の `qgis_settings.json` を使用します。
	- `--settings_dir` が存在しない（またはファイルが無い）場合は、実行ファイルと同階層の `qgis_settings.json` を参照します。
	- デフォルトの `--settings_dir` 値は `C:\qgis_launcher` です（ただしディレクトリが無ければ上記の実行ファイル同階層を参照します）。

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

### ドライブ構成と役割

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
    "BOX": "%USERPROFILE%\\Box"
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

`path_aliases` ではエイリアス名（2文字以上）をパスにマッピングできます。`BOX` は未定義時のデフォルトで `%USERPROFILE%\Box` に解決されます。

| 記法例 | 展開後 |
|---|---|
| `BOX:\Geo_Portal` | `C:\Users\<ユーザ>\Box\Geo_Portal` |
| `%USERPROFILE%\Box\Geo_Portal` | `C:\Users\<ユーザ>\Box\Geo_Portal` |
| `C:\qgis_cache\master` | そのまま |

### 事前準備

- **BOX Drive アプリ**をインストールし、対象フォルダをオフライン同期（常にこのデバイス上に保持）に設定してください。
- Q: の `local_cache`（`C:\qgis_cache\master` 等）は初回起動時に自動作成されます。

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
