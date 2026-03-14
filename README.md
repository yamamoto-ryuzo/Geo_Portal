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

- シンプルに起動時に `qgis_settings.json` を参照して QGIS を起動します。
- 起動優先順: `qgis_settings.json`（`settings_dir` 内、存在すれば）→ コマンドライン `--profile` 引数。
- デフォルトの設定ファイル位置: `C:\qgis_launcher\qgis_settings.json`（`--settings_dir` で上書き可）。実行ファイルと同階層に置けばそちらが優先されます。
- スタートアップ登録: `qgis_launcher.exe --register_startup` を実行するとスタートアップ用のショートカットが作成されます。作成されるショートカットの引数は次の形式です:

```
--profile <プロファイル名> --settings_dir "<設定ディレクトリのパス>"
```

- ビルド・実行例:

```powershell
cd qgis_launcher
cargo build --release
# 直接起動（コマンドライン指定のプロファイルで起動）
.
target\release\qgis_launcher.exe --profile geo_custum
```

変更や動作確認が必要であれば教えてください。

### qgis_launcher: 追加ドキュメント（統合内容）

- 必要な環境:
	- Rust と Cargo（未インストールなら [rustup](https://rustup.rs/) をインストールしてください）

- ビルド方法:

```powershell
cd qgis_launcher
cargo build --release
```

ビルド成功後は `qgis_launcher\target\release\qgis_launcher.exe` が生成されます。生成された `.exe` は単体で動作するため、LGWAN 環境等へ配布可能です。

- 使い方（例）:

直接起動（デフォルトプロファイル）:

```powershell
.\qgis_launcher.exe
```

プロファイル指定:

```powershell
.\qgis_launcher.exe --profile "LGWAN_Profile"
```

（注）本バージョンでは `qgis_settings.json` か CLI 引数でプロファイルを指定してください。
