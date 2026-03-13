import { useState } from "react";
import { PortalProvider, usePortal } from "../src/PortalContext";

const SERVICES = [
  { name: "Re:Earth", url: "https://c-01kcwqbkykrk15apgxeqrvr6rv.visualizer.reearth.io/" },
  { name: "BOX", url: "https://app.box.com/embed/s/" },
  { name: "Backlog", url: "https://backlog.com/ja/" },
];

function QgisLaunchButton() {
  const [status, setStatus] = useState('idle');

  const handleLaunch = async () => {
    setStatus('checking');
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 2000);

      const response = await fetch('http://127.0.0.1:12345/launch/qgis', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        signal: controller.signal
      });
      clearTimeout(timeoutId);

      if (response.ok) {
        setStatus('success');
        setTimeout(() => setStatus('idle'), 3000);
      } else {
        throw new Error('Agent responded with an error');
      }
    } catch (error) {
      setStatus('fallback');
    }
  };

  return (
    <div style={{ marginTop: "20px", padding: "15px", border: "1px solid #ccc", borderRadius: "8px", backgroundColor: "white" }}>
      <h3 style={{ fontSize: "1rem", margin: "0 0 10px 0" }}>ローカルQGIS連携</h3>
      
      {status === 'idle' || status === 'checking' ? (
        <button 
          onClick={handleLaunch} 
          disabled={status === 'checking'}
          style={{ width: "100%", padding: "10px", backgroundColor: "#0070f3", color: "white", border: "none", borderRadius: "4px", cursor: status === 'checking' ? "wait" : "pointer" }}
        >
          {status === 'checking' ? '通信確認中...' : 'QGISを起動する'}
        </button>
      ) : status === 'success' ? (
        <p style={{ color: "green", fontSize: "0.9rem", margin: 0 }}>✓ QGISを起動しました。</p>
      ) : (
        <div>
          <p style={{ color: "#d97706", fontSize: "0.8rem", margin: "0 0 10px 0" }}>
            ⚠️ 自動起動できませんでした。（LGWAN環境の可能性があります）
          </p>
          <a 
            href="/downloads/qgis_launcher.exe" 
            download 
            style={{ display: "block", textAlign: "center", padding: "8px", backgroundColor: "#10b981", color: "white", textDecoration: "none", borderRadius: "4px", fontSize: "0.9rem" }}
          >
            直接起動用アプリをDL
          </a>
        </div>
      )}
    </div>
  );
}

function Inner() {
  const [selectedUrl, setSelectedUrl] = useState(SERVICES[0].url);
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const { 
    reearthUrl, boxUrl, previewReearth, previewBox, setPreviewReearth, setPreviewBox, 
    previewQgisProfile, previewQgisProjectPath, setPreviewQgisProfile, setPreviewQgisProjectPath,
    applyPreview, save, resetTo, applyPreviewAndSave 
  } = usePortal();
  const TAB_DEFS = {
    reearth: { id: "reearth", label: "Re:Earth", src: SERVICES[0].url },
    googlemap: { id: "googlemap", label: "Googleマップ", src: "https://www.google.com/maps/embed?pb=!1m14!1m12!1m3!1d12966.99268688849!2d139.7454329!3d35.6585805!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!5e0!3m2!1sja!2sjp!4v1631234567890!5m2!1sja!2sjp" },
    box: { id: "box", label: "BOX", src: SERVICES[1].url },
    settings: { id: "settings", label: "ポータル設定" },
    qgis_settings: { id: "qgis_settings", label: "QGIS設定" }
  };
  const [tabs, setTabs] = useState(["reearth","googlemap","box","settings","qgis_settings"]);
  const [activeTab, setActiveTab] = useState("reearth");

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
              {(() => {
                const currentOpenUrl = activeTab === 'reearth' ? reearthUrl : activeTab === 'box' ? boxUrl : selectedUrl;
                return (
                  <a
                    href={currentOpenUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    style={{ display: "block", textDecoration: "none", color: "#0070f3", fontSize: "0.9rem" }}
                  >
                    別ウィンドウで開く ↗
                  </a>
                );
              })()}
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
      <div style={{ flex: 1, display: "flex", flexDirection: "column", position: "relative" }}>
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
        <div style={{ flex: 1, display: "flex", flexDirection: "column" }}>
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
            title="BOX Frame"
            src={boxUrl}
            width="100%"
            height="100%"
            style={{ border: "none", flex: 1, display: activeTab === "box" ? "block" : "none" }}
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
          {/* 設定パネル: Re:Earth と BOX の表示URLを設定 */}
          <div style={{ flex: 1, padding: "20px", display: activeTab === "settings" ? "block" : "none", overflow: "auto" }}>
            <h2>ポータル設定</h2>
            <div style={{ marginTop: '8px' }}>
              <label style={{ display: 'block', marginBottom: '6px' }}>1. Re:Earth 表示アドレス</label>
              <input
                type="text"
                value={previewReearth}
                onChange={(e) => setPreviewReearth(e.target.value)}
                style={{ width: '100%', padding: '8px', boxSizing: 'border-box' }}
              />
            </div>
            <div style={{ marginTop: '16px' }}>
              <label style={{ display: 'block', marginBottom: '6px' }}>2. BOX ウィジェット表示アドレス</label>
              <input
                type="text"
                value={previewBox}
                onChange={(e) => setPreviewBox(e.target.value)}
                style={{ width: '100%', padding: '8px', boxSizing: 'border-box' }}
              />
            </div>

            <div style={{ marginTop: '24px', display: 'flex', gap: '8px' }}>
              <button onClick={() => { applyPreview(); }} style={{ padding: '8px 12px', cursor: 'pointer' }}>プレビュー</button>
              <button onClick={async () => { const ok = await applyPreviewAndSave(); if (ok) { alert('保存しました'); } else { alert('設定の保存に失敗しました。'); } }} style={{ padding: '8px 12px', cursor: 'pointer' }}>保存</button>
              <button onClick={() => { resetTo(); }} style={{ padding: '8px 12px', cursor: 'pointer' }}>初期化</button>
            </div>
          </div>

          {/* QGIS設定パネル */}
          <div style={{ flex: 1, padding: "20px", display: activeTab === "qgis_settings" ? "block" : "none", overflow: "auto", backgroundColor: "#f0f8ff" }}>
            <h2>ローカルQGIS 起動設定</h2>
            <p style={{ fontSize: "0.9rem", color: "#555", marginBottom: "20px" }}>
              この設定は、ランチャー（qgis_launcher.exe）を経由してローカルのQGISを起動する際の動作を指定します。
            </p>
            
            <div style={{ marginTop: '16px' }}>
              <label style={{ display: 'block', marginBottom: '6px', fontWeight: "bold" }}>1. 起動プロファイル名</label>
              <input
                type="text"
                value={previewQgisProfile}
                onChange={(e) => setPreviewQgisProfile(e.target.value)}
                style={{ width: '100%', padding: '8px', boxSizing: 'border-box', border: "1px solid #ccc", borderRadius: "4px" }}
                placeholder="default"
              />
            </div>
            <div style={{ marginTop: '16px' }}>
              <label style={{ display: 'block', marginBottom: '6px', fontWeight: "bold" }}>2. 起動時プロジェクトファイルパス (.qgs / .qgz)</label>
              <input
                type="text"
                value={previewQgisProjectPath}
                onChange={(e) => setPreviewQgisProjectPath(e.target.value)}
                style={{ width: '100%', padding: '8px', boxSizing: 'border-box', border: "1px solid #ccc", borderRadius: "4px" }}
                placeholder="C:\Users\...\Desktop\project.qgs"
              />
            </div>

            <div style={{ marginTop: '24px', display: 'flex', gap: '12px', flexWrap: "wrap" }}>
              <button 
                onClick={async () => { 
                  const ok = await applyPreviewAndSave(); 
                  if (ok) { alert('ランチャーに設定を保存しました。'); } 
                  else { alert('保存に失敗しました。ローカルランチャーが起動していない可能性があります。'); } 
                }} 
                style={{ padding: '10px 16px', cursor: 'pointer', backgroundColor: "#0070f3", color: "white", border: "none", borderRadius: "4px" }}
              >
                ローカルランチャーに保存
              </button>
              
              <button
                onClick={() => {
                  // 設定内容をJSONファイルとしてダウンロード（オフライン環境への持ち込み用）
                  const payload = { profile: previewQgisProfile || "default", project_path: previewQgisProjectPath || "" };
                  const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(payload, null, 2));
                  const downloadAnchorNode = document.createElement('a');
                  downloadAnchorNode.setAttribute("href",     dataStr);
                  downloadAnchorNode.setAttribute("download", "qgis_settings.json");
                  document.body.appendChild(downloadAnchorNode);
                  downloadAnchorNode.click();
                  downloadAnchorNode.remove();
                }}
                style={{ padding: '10px 16px', cursor: 'pointer', backgroundColor: "#10b981", color: "white", border: "none", borderRadius: "4px" }}
                title="オンラインで作成した設定を qgis_settings.json としてダウンロードし、オフライン環境のexeと同じフォルダに配置できます。"
              >
                📥 設定ファイル (JSON) をダウンロード
              </button>

              <button 
                onClick={() => { resetTo(); }} 
                style={{ padding: '10px 16px', cursor: 'pointer', backgroundColor: "#e5e7eb", border: "none", borderRadius: "4px" }}
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

export default function Home() {
  return (
    <PortalProvider initialReearth={SERVICES[0].url} initialBox={SERVICES[1].url}>
      <Inner />
    </PortalProvider>
  );
}
