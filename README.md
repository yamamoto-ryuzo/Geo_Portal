# ReEarth_Portal — 左右パネル連携方針（プロトタイプ）

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

---
作業で追加・編集した主なファイル: [pages/index.js](pages/index.js#L1-L400)、[src/PortalContext.js](src/PortalContext.js#L1-L200)

必要なら上記の次候補を試作します。どれを優先しますか？

## QGIS Launcher (qgis_launcher)

- 概要: 起動設定（プロファイル/プロジェクトパス/QGIS Version）に基づいて QGIS・QField を起動するランチャーです。`qgis_settings.json` を優先して読み込み、存在しない場合は CLI 引数を参照します。

- GUI: デフォルトで設定用の簡易 GUI ウィンドウを表示します。GUI ではプルダウンリストからプロファイル、プロジェクト、QGIS Versionを選択し、「Launch QGIS」ボタンで起動できます。リスト候補には `%APPDATA%` 内のプロファイルや、設定ディレクトリ（デフォルトは `C:\qgis_launcher`）直下・1階層下の `.qgs`/`.qgz` ファイルが自動的に追加されます。また、システム内にインストールされているQGIS・QFieldが自動検出されバージョン選択候補として表示されます。

- CLI モード: コマンドラインやショートカットから GUI を出さずに即座に起動させたい場合は `--cli` オプションを付けて実行してください。

- 起動優先順: `qgis_settings.json`（`settings_dir` 内、存在すれば）→ コマンドライン `--profile` 引数。

- デフォルト設定ファイル位置: `C:\qgis_launcher\qgis_settings.json`（`--settings_dir` で上書き可）。実行ファイルと同階層に置けばそちらが優先されます。

- スタートアップ登録: `qgis_launcher.exe --register_startup` を実行するとスタートアップ用のショートカットが作成されます。作成されるショートカットの引数は次の形式です:

```
--profile <プロファイル名> --settings_dir "<設定ディレクトリのパス>"
```

- ビルド・実行例:

```powershell
cd qgis_launcher
cargo build --release
# ランチャー（デフォルトは GUI を起動）
cargo run --release
# CLI モードで即座に起動
cargo run --release -- --cli
```

- 備考: FLTK は `Cargo.toml` 上で `fltk-bundled` を利用する設定になっています（FLTK をソースからバンドルビルド）。ローカルに CMake/FLTK を用意する必要はありませんが、初回のビルドは時間がかかります。

変更や動作確認が必要であれば教えてください。
