import { useState } from "react";
import { PortalProvider, usePortal } from "../src/PortalContext";
import path from "path";
import fs from "fs";

const SERVICES = [
  { name: "Re:Earth", url: "https://c-01kcwqbkykrk15apgxeqrvr6rv.visualizer.reearth.io/" },
  { name: "Backlog", url: "https://backlog.com/ja/" },
];

function QgisLaunchButton() {
  return (
    <div style={{ marginTop: "20px", padding: "15px", border: "1px solid #ccc", borderRadius: "8px", backgroundColor: "white" }}>
      <h3 style={{ fontSize: "1rem", margin: "0 0 10px 0" }}>QGIS連携</h3>
      
      <div style={{ marginTop: "15px" }}>
        <a 
          href="https://github.com/yamamoto-ryuzo/Geo_Portal/raw/refs/heads/main/public/qgis_launcher.zip" 
          download 
          style={{ display: "block", textAlign: "center", padding: "8px", backgroundColor: "#4b5563", color: "white", textDecoration: "none", borderRadius: "4px", fontSize: "0.9rem" }}
        >
          QGISカスタムツール一式を<br />ZIPでDL
        </a>
      </div>
    </div>
  );
}

function Inner() {
  const [selectedUrl, setSelectedUrl] = useState(SERVICES[0].url);
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const {
    reearthUrl, previewReearth, setPreviewReearth,
    previewQgisProfile, previewQgisProjectPath,
    previewPathAliases, previewRcloneMounts,
    setPreviewQgisProfile, setPreviewQgisProjectPath,
    setPreviewPathAliases, setPreviewRcloneMounts,
    applyPreview, save, resetTo, applyPreviewAndSave, saveToFs, loadSettingsFromFile, loadSettingsFromDir
  } = usePortal();

  const TAB_DEFS = {
    reearth: { id: "reearth", label: "Re:Earth", src: SERVICES[0].url },
    googlemap: { id: "googlemap", label: "Googleマップ", src: "https://www.google.com/maps/embed?pb=!1m14!1m12!1m3!1d12966.99268688849!2d139.7454329!3d35.6585805!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!5e0!3m2!1sja!2sjp!4v1631234567890!5m2!1sja!2sjp" },
    // BOX tab removed
    settings: { id: "settings", label: "ポータル設定" }
  };
  const [tabs, setTabs] = useState(["reearth","googlemap","settings"]);
  const [activeTab, setActiveTab] = useState("reearth");
  const [activeSettingsTab, setActiveSettingsTab] = useState("common");

  return (
    <div style={{ display: "flex", height: "100vh", overflow: "hidden" }}>
      {/* 左サイドバー: コントロール */}
        {isSidebarOpen ? (
          <div style={{
            width: "250px",
            minWidth: "250px",
            backgroundColor: "#f5f5f5",
            borderRight: "1px solid #ddd",
            padding: "20px",
            display: "flex",
            flexDirection: "column"
          }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
              <h1 style={{ fontSize: "1.2rem", margin: 0 }}>ポータルサイト</h1>
              <button
                onClick={() => setIsSidebarOpen(false)}
                style={{
                  background: "none",
                  border: "none",
                  fontSize: "1.2rem",
                  cursor: "pointer",
                  padding: "0 5px"
                }}
                title="メニューを閉じる"
              >
                ✕
              </button>
            </div>

            <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
              {SERVICES.filter(s => !["Re:Earth", "BOX", "Backlog"].includes(s.name)).map((service) => (
                <button
                  key={service.name}
                  onClick={() => setSelectedUrl(service.url)}
                  style={{
                    padding: "10px",
                    textAlign: "left",
                    backgroundColor: selectedUrl === service.url ? "#0070f3" : "white",
                    color: selectedUrl === service.url ? "white" : "black",
                    border: "1px solid #ccc",
                    borderRadius: "4px",
                    cursor: "pointer",
                  }}
                >
                  {service.name}
                </button>
              ))}
              {/* (removed) 設定情報を開く button */}
            </div>

            <QgisLaunchButton />

            <div style={{ marginTop: "auto", borderTop: "1px solid #ccc", paddingTop: "10px" }}>
                <a
                  href={activeTab === 'reearth' ? reearthUrl : selectedUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  style={{ display: "block", textDecoration: "none", color: "#0070f3", fontSize: "0.9rem" }}
                >
                  別ウィンドウで開く ↗
                </a>
            </div>
          </div>
        ) : (
          <div style={{
            width: "32px",
            minWidth: "32px",
            backgroundColor: "#f5f5f5",
            borderRight: "1px solid #ddd",
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center"
          }}>
            <button
              onClick={() => setIsSidebarOpen(true)}
              style={{
                background: "none",
                border: "none",
                cursor: "pointer",
                fontSize: "1.5rem",
                padding: "0",
                marginTop: "8px"
              }}
              title="メニューを開く"
            >
              <span role="img" aria-label="帽子">🎩</span>
            </button>
          </div>
        )}

      {/* 右メインエリア: コンテンツ */}
      <div style={{ flex: 1, display: "flex", flexDirection: "column", position: "relative", minHeight: 0 }}>
        {/* タブ UI (ドラッグで並べ替え可) */}
        <div style={{ display: "flex", borderBottom: "1px solid #ddd", backgroundColor: "#f5f5f5" }}>
          {tabs.map((tabId, idx) => {
            const tab = TAB_DEFS[tabId];
            return (
              <button
                key={tab.id}
                draggable
                onDragStart={(e) => e.dataTransfer.setData('text/tab', String(idx))}
                onDragOver={(e) => e.preventDefault()}
                onDrop={(e) => {
                  const from = Number(e.dataTransfer.getData('text/tab'));
                  const to = idx;
                  if (from === to) return;
                  setTabs(prev => {
                    const arr = [...prev];
                    const [moved] = arr.splice(from, 1);
                    arr.splice(to, 0, moved);
                    return arr;
                  });
                }}
                onClick={() => setActiveTab(tab.id)}
                style={{
                  padding: "10px 20px",
                  border: "none",
                  backgroundColor: activeTab === tab.id ? "white" : "transparent",
                  borderBottom: activeTab === tab.id ? "2px solid #0070f3" : "none",
                  cursor: "pointer",
                  fontWeight: activeTab === tab.id ? "bold" : "normal",
                  color: activeTab === tab.id ? "#0070f3" : "#333",
                }}
              >
                {tab.label}
              </button>
            );
          })}
        </div>

        {/* コンテンツ表示エリア */}
        <div style={{ flex: 1, display: "flex", flexDirection: "column", minHeight: 0 }}>
          <iframe
            title="Re:Earth Frame"
            src={reearthUrl}
            width="100%"
            height="100%"
            style={{ border: "none", flex: 1, display: activeTab === "reearth" ? "block" : "none" }}
            allowFullScreen
            allow="clipboard-read; clipboard-write"
            loading="lazy"
          />
          <iframe
            title="Google Maps"
            src="https://www.google.com/maps/embed?pb=!1m14!1m12!1m3!1d12966.99268688849!2d139.7454329!3d35.6585805!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!5e0!3m2!1sja!2sjp!4v1631234567890!5m2!1sja!2sjp"
            width="100%"
            height="100%"
            style={{ border: 0, flex: 1, display: activeTab === "googlemap" ? "block" : "none" }}
            allowFullScreen=""
            loading="lazy"
          />

          {/* 統合設定パネル */}
          <div style={{ flex: 1, padding: "20px", display: activeTab === "settings" ? "flex" : "none", flexDirection: "column", overflow: "hidden", minHeight: 0 }}>
            <h2 style={{ margin: "0 0 20px 0" }}>ポータル設定</h2>
            
            {/* サブタブナビゲーション */}
            <div style={{ display: "flex", borderBottom: "2px solid #ddd", marginBottom: "20px" }}>
              {[
                { id: "common", label: "共通設定" },
                  { id: "reearth", label: "Re:Earth" },
                  { id: "qgis", label: "QGIS" }
              ].map(subTab => (
                <button
                  key={subTab.id}
                  onClick={() => setActiveSettingsTab(subTab.id)}
                  style={{
                    padding: "10px 20px",
                    border: "none",
                    backgroundColor: "transparent",
                    borderBottom: activeSettingsTab === subTab.id ? "3px solid #0070f3" : "3px solid transparent",
                    cursor: "pointer",
                    fontWeight: activeSettingsTab === subTab.id ? "bold" : "normal",
                    color: activeSettingsTab === subTab.id ? "#0070f3" : "#555",
                    marginBottom: "-2px"
                  }}
                >
                  {subTab.label}
                </button>
              ))}
            </div>

            {/* サブタブコンテンツ領域 */}
            <div style={{ flex: 1, overflow: "auto", paddingRight: "10px", minHeight: 0 }}>
              
              {/* 共通設定 */}
              <div style={{ display: activeSettingsTab === "common" ? "block" : "none" }}>
                <h3 style={{ marginTop: 0 }}>システム共通</h3>
                {/* Removed settings_dir */}
              </div>

              {/* Re:Earth設定 */}
              <div style={{ display: activeSettingsTab === "reearth" ? "block" : "none" }}>
                <h3 style={{ marginTop: 0 }}>Re:Earth 設定</h3>
                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontWeight: "bold" }}>表示アドレス (URL)</label>
                  <input
                    type="text"
                    value={previewReearth}
                    onChange={(e) => setPreviewReearth(e.target.value)}
                    style={{ width: '100%', padding: '10px', boxSizing: 'border-box', border: "1px solid #ccc", borderRadius: "4px" }}
                  />
                </div>
              </div>

              {/* BOX removed */}

              {/* QGIS設定 */}
              <div style={{ display: activeSettingsTab === "qgis" ? "block" : "none" }}>
                <h3 style={{ marginTop: 0 }}>ローカルQGIS 起動設定</h3>
                <p style={{ fontSize: "0.9rem", color: "#555", marginBottom: "20px" }}>
                  ランチャー（qgis_launcher.exe）経由でローカルのQGISを起動する際の動作を指定します。
                </p>
                
                <div style={{ marginBottom: '16px' }}>
                  <label style={{ display: 'block', marginBottom: '6px', fontWeight: "bold" }}>起動プロファイル名</label>
                  <input
                    type="text"
                    value={previewQgisProfile}
                    onChange={(e) => setPreviewQgisProfile(e.target.value)}
                    style={{ width: '100%', padding: '10px', boxSizing: 'border-box', border: "1px solid #ccc", borderRadius: "4px" }}
                    placeholder="geo_custom"
                  />
                </div>
                <div style={{ marginBottom: '24px' }}>
                  <label style={{ display: 'block', marginBottom: '6px', fontWeight: "bold" }}>起動時プロジェクトファイル / フォルダ (.qgs / .qgz)</label>
                  <p style={{ fontSize: "0.85rem", color: "#666", margin: "0 0 8px 0" }}>
                    1行に1パスを入力してください。ファイルまたはフォルダのパスを指定できます（相対パスはランチャー実行フォルダ基準）。
                  </p>
                  <textarea
                    value={Array.isArray(previewQgisProjectPath) ? previewQgisProjectPath.join('\n') : (previewQgisProjectPath || '')}
                    onChange={(e) => setPreviewQgisProjectPath(e.target.value.split('\n'))}
                    rows={4}
                    style={{ width: '100%', padding: '10px', boxSizing: 'border-box', border: "1px solid #ccc", borderRadius: "4px", fontFamily: "monospace", fontSize: "0.9rem", resize: "vertical" }}
                    placeholder={"ProjectFiles\nC:\\ProjectFiles2\\ProjectFile.qgs"}
                  />
                </div>

                {/* BOX パスエイリアス */}
                <div style={{ marginBottom: '24px', padding: '16px', background: '#f8fafc', border: '1px solid #e2e8f0', borderRadius: '8px' }}>
                  <h4 style={{ margin: '0 0 12px 0', fontSize: '0.95rem' }}>📁 BOX パスエイリアス (path_aliases)</h4>
                  <p style={{ fontSize: '0.82rem', color: '#666', margin: '0 0 12px 0' }}>
                    <code>BOX:\Geo_Portal</code> のように書いたとき展開されるベースパスを定義します。
                  </p>
                  {Object.entries(previewPathAliases || {}).map(([key, val], i) => (
                    <div key={i} style={{ display: 'flex', gap: '8px', marginBottom: '8px', alignItems: 'center' }}>
                      <input
                        type="text"
                        value={key}
                        onChange={(e) => {
                          const newKey = e.target.value;
                          const entries = Object.entries(previewPathAliases);
                          entries[i] = [newKey, val];
                          setPreviewPathAliases(Object.fromEntries(entries));
                        }}
                        style={{ width: '90px', padding: '6px 8px', border: '1px solid #ccc', borderRadius: '4px', fontFamily: 'monospace', fontSize: '0.85rem' }}
                        placeholder="BOX"
                      />
                      <span style={{ color: '#888' }}>→</span>
                      <input
                        type="text"
                        value={val}
                        onChange={(e) => {
                          const entries = Object.entries(previewPathAliases);
                          entries[i] = [key, e.target.value];
                          setPreviewPathAliases(Object.fromEntries(entries));
                        }}
                        style={{ flex: 1, padding: '6px 8px', border: '1px solid #ccc', borderRadius: '4px', fontFamily: 'monospace', fontSize: '0.85rem' }}
                        placeholder="%USERPROFILE%\\Box"
                      />
                      <button
                        onClick={() => {
                          const entries = Object.entries(previewPathAliases).filter((_, idx) => idx !== i);
                          setPreviewPathAliases(Object.fromEntries(entries));
                        }}
                        style={{ padding: '4px 10px', background: '#fee2e2', color: '#b91c1c', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '0.85rem' }}
                      >削除</button>
                    </div>
                  ))}
                  <button
                    onClick={() => setPreviewPathAliases({ ...previewPathAliases, '': '' })}
                    style={{ padding: '6px 14px', background: '#e0f2fe', color: '#0369a1', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '0.85rem', marginTop: '4px' }}
                  >＋ エイリアスを追加</button>
                </div>

                {/* ドライブ割り当て */}
                <div style={{ marginBottom: '24px', padding: '16px', background: '#f8fafc', border: '1px solid #e2e8f0', borderRadius: '8px' }}>
                  <h4 style={{ margin: '0 0 12px 0', fontSize: '0.95rem' }}>💾 ドライブ割り当て (rclone_mounts)</h4>
                  <p style={{ fontSize: '0.82rem', color: '#666', margin: '0 0 12px 0' }}>
                    QGIS起動前に subst でフォルダをドライブレターへ自動割り当てします。
                  </p>
                  {(previewRcloneMounts || []).map((mount, i) => (
                    <div key={i} style={{ background: 'white', border: '1px solid #d1d5db', borderRadius: '6px', padding: '12px', marginBottom: '12px' }}>
                      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
                        <span style={{ fontWeight: 'bold', fontFamily: 'monospace', fontSize: '0.95rem', color: '#1d4ed8' }}>{mount.drive || '(ドライブ未設定)'}</span>
                        <button
                          onClick={() => setPreviewRcloneMounts(previewRcloneMounts.filter((_, idx) => idx !== i))}
                          style={{ padding: '3px 10px', background: '#fee2e2', color: '#b91c1c', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '0.82rem' }}
                        >削除</button>
                      </div>
                      {[
                        { field: 'drive', label: 'ドライブ', placeholder: 'Q:' },
                        { field: 'mode', label: 'モード', placeholder: 'subst' },
                        { field: 'local_cache', label: 'ローカルパス (local_cache)', placeholder: 'C:\\qgis_cache\\master' },
                        { field: 'robocopy_src', label: 'コピー元 (robocopy_src) ※省略可', placeholder: 'BOX:\\Geo_Portal' },
                      ].map(({ field, label, placeholder }) => (
                        <div key={field} style={{ display: 'flex', gap: '8px', alignItems: 'center', marginBottom: '7px' }}>
                          <label style={{ width: '200px', fontSize: '0.83rem', color: '#555', flexShrink: 0 }}>{label}</label>
                          <input
                            type="text"
                            value={mount[field] || ''}
                            onChange={(e) => {
                              const updated = [...previewRcloneMounts];
                              updated[i] = { ...updated[i], [field]: e.target.value };
                              setPreviewRcloneMounts(updated);
                            }}
                            style={{ flex: 1, padding: '5px 8px', border: '1px solid #ccc', borderRadius: '4px', fontFamily: 'monospace', fontSize: '0.83rem' }}
                            placeholder={placeholder}
                          />
                        </div>
                      ))}
                      <div style={{ display: 'flex', gap: '8px', alignItems: 'flex-start', marginBottom: '7px' }}>
                        <label style={{ width: '200px', fontSize: '0.83rem', color: '#555', flexShrink: 0, paddingTop: '6px' }}>除外フォルダ (robocopy_exclude)<br /><span style={{ color: '#999', fontSize: '0.78rem' }}>カンマ区切り</span></label>
                        <input
                          type="text"
                          value={(mount.robocopy_exclude || []).join(', ')}
                          onChange={(e) => {
                            const updated = [...previewRcloneMounts];
                            updated[i] = { ...updated[i], robocopy_exclude: e.target.value.split(',').map(s => s.trim()).filter(Boolean) };
                            setPreviewRcloneMounts(updated);
                          }}
                          style={{ flex: 1, padding: '5px 8px', border: '1px solid #ccc', borderRadius: '4px', fontFamily: 'monospace', fontSize: '0.83rem' }}
                          placeholder="secret-folder, private-data"
                        />
                      </div>
                    </div>
                  ))}
                  <button
                    onClick={() => setPreviewRcloneMounts([...(previewRcloneMounts || []), { drive: '', mode: 'subst', local_cache: '', robocopy_src: '', robocopy_exclude: [] }])}
                    style={{ padding: '6px 14px', background: '#e0f2fe', color: '#0369a1', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '0.85rem', marginTop: '4px' }}
                  >＋ ドライブを追加</button>
                </div>

                <div style={{ textAlign: "center", marginTop: "20px" }}>
                  <img src="/image/qgis.png" alt="QGIS" style={{ maxWidth: "100%", height: "auto", borderRadius: "6px" }} />
                </div>
              </div>

            </div>

            {/* 共通フッター・操作ボタンエリア */}
            <div style={{ marginTop: 'auto', paddingTop: '20px', borderTop: '1px solid #ddd', display: 'flex', gap: '12px', flexWrap: "wrap", backgroundColor: "#fff" }}>
              <div style={{ position: "relative", display: "inline-block" }}>
                <input 
                  type="file" 
                  accept=".json" 
                  id="settings-upload"
                  style={{ display: "none" }}
                  onChange={async (e) => {
                    if (e.target.files && e.target.files[0]) {
                      try {
                        await loadSettingsFromFile(e.target.files[0]);
                        alert("設定ファイルから読み込みました。");
                      } catch (err) {
                        alert("ファイルの読み込みに失敗しました。正しいJSONファイルを選択してください。");
                      }
                      e.target.value = null; // reset input
                    }
                  }}
                />
                <button
                  onClick={async () => {
                    // File System Access API を使って直接ファイルを選ばせる（対応ブラウザ）
                    try {
                      if (window.showOpenFilePicker) {
                        const [handle] = await window.showOpenFilePicker({
                          types: [{ description: 'JSON', accept: { 'application/json': ['.json'] } }],
                          multiple: false
                        });
                        const file = await handle.getFile();
                        if (file) {
                          try {
                            await loadSettingsFromFile(file);
                            alert('設定ファイルから読み込みました。');
                          } catch (err) {
                            alert('ファイルの読み込みに失敗しました。正しいJSONファイルを選択してください。');
                          }
                          return;
                        }
                      }
                    } catch (e) {
                      // fallthrough to fallback
                    }

                    // フォールバック: 既存の非表示 input[type=file] を使う
                    document.getElementById('settings-upload').click();
                  }}
                  style={{ padding: '10px 16px', cursor: 'pointer', backgroundColor: "#6366f1", color: "white", border: "none", borderRadius: "4px", fontWeight: "bold" }}
                  title="手元にある qgis_settings.json を読み込んで、この画面に反映させます。"
                >
                  📂 設定ファイルを読み込む
                </button>
                
                <button
                  onClick={async () => {
                    try {
                      const ok = await saveToFs();
                      if (ok) alert("設定ファイルを保存しました。");
                      else alert("ブラウザの保存に失敗しました。ローカルランチャーが起動している場合は『パスに保存』をお試しください。");
                    } catch (e) {
                      alert("保存に失敗しました。操作がキャンセルされた可能性があります。");
                    }
                  }}
                  style={{ padding: '10px 16px', marginLeft: '8px', cursor: 'pointer', backgroundColor: "#0ea5e9", color: "white", border: "none", borderRadius: "4px", fontWeight: "bold" }}
                  title="ブラウザのファイル選択ダイアログで保存先を選び、このマシン上に qgis_settings.json を書き出します。"
                >
                  💾 ファイルとして保存
                </button>
              </div>

              <button 
                onClick={() => { applyPreview(); }} 
                style={{ padding: '10px 16px', cursor: 'pointer', backgroundColor: "#fff", border: "1px solid #ccc", borderRadius: "4px" }}
              >
                プレビュー適用
              </button>

              <button 
                onClick={() => { resetTo(); }} 
                style={{ padding: '10px 16px', cursor: 'pointer', backgroundColor: "#fee2e2", color: "#b91c1c", border: "none", borderRadius: "4px", marginLeft: "auto" }}
              >
                リセット
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default function Home({ initialSettings }) {
  return (
    <PortalProvider initialReearth={SERVICES[0].url} initialSettings={initialSettings}>
      <Inner />
    </PortalProvider>
  );
}

export async function getStaticProps() {
  try {
    const filePath = path.join(process.cwd(), 'qgis_launcher', 'download', 'qgis_settings.json');
    const raw = fs.readFileSync(filePath, 'utf-8');
    const initialSettings = JSON.parse(raw);
    return { props: { initialSettings } };
  } catch (e) {
    return { props: { initialSettings: null } };
  }
}
