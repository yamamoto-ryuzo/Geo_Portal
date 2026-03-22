# Changelog
## [1.1.0] - 2026-03-22

### 変更点
- GUI: `Reset Profiles` ボタンを追加（プロファイル破損時の再構築対応）
- ランチャー: `reset_profiles()` を追加 — 既存プロファイルを強制削除して配布プロファイルを再コピー
- UX: リセット処理をバックグラウンド実行し、モーダル風の進捗ウィンドウ（メッセージ + プログレスバー）を表示
- UI: リセット完了後は進捗ウィンドウを自動で閉じ、メインウィンドウのステータスに結果を表示
- その他: 内部ロジックの微調整とビルドスクリプトの更新

## [1.0.2] - 2026-03-22

### 変更点
- `rclone_mounts` を `drive_mappings` にリネーム（フロントエンド / ランチャー / ドキュメント / サンプルを更新）
- Web ポータル側では `rclone` を実行しない旨を明確化し、ブラウザでの編集を無効化（表示のみ）
- ランチャー側の関数/呼び出しとサンプル JSON を更新、ビルド成果物を再生成

## [1.0.1] - 2026-03-22

### 変更点
- `qgis_settings` の適用順序を「ベース → 全ユーザー強制 → ユーザー個別上書き」に変更。

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

