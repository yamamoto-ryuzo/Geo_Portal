# Changelog
## [1.0.0] - 2026-03-21

### 概要
- ReEarth_Portal と qgis_launcher のこれまでの変更点を統合した安定版リリース。

### 機能（ReEarth_Portal）
- 左パネルで操作・設定、右パネルで表示を行うマスタ詳細UIを提供（`reearth`, `googlemap`, `settings` タブ）。
- 設定は `portal:settings` と `src/PortalContext.js` で一元管理。起動時の初期化優先順は「クエリ > localStorage > デフォルト」。
- ブラウザの File System Access を利用した設定読み込み/保存、iframe の遅延ロードによるパフォーマンス改善。

### 機能（QGIS Launcher）
- Rust 製ランチャーによる QGIS 起動・プロファイル管理・プロジェクト選択（GUI / CLI）。
- 複数の QGIS バージョン（インストール版/ポータブル）自動検出、`--qgis_executable` による実行ファイル指定対応。
- `qgis_settings.json` とユーザー/強制オーバーライド（`qgis_settings_{USERNAME}.json`, `qgis_settings_override.json`）のマージルールを実装。
- OneDrive / Google Drive 等を含む `path_aliases` によるクラウドストレージ対応とプロジェクト列挙の改善。

