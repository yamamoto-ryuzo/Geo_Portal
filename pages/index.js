import { useState } from "react";

const SERVICES = [
  { name: "Re:Earth", url: "https://c-01kcwqbkykrk15apgxeqrvr6rv.visualizer.reearth.io/" },
  { name: "BOX", url: "https://app.box.com/embed/s/28rzb0y0oqh25dv16p6j4swayqbg6z8a?sortColumn=date" },
  { name: "Backlog", url: "https://backlog.com/ja/" },
];

export default function Home() {
  const [selectedUrl, setSelectedUrl] = useState(SERVICES[0].url);
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const TAB_DEFS = {
    reearth: { id: "reearth", label: "Re:Earth", src: SERVICES[0].url },
    googlemap: { id: "googlemap", label: "Googleマップ", src: "https://www.google.com/maps/embed?pb=!1m14!1m12!1m3!1d12966.99268688849!2d139.7454329!3d35.6585805!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!5e0!3m2!1sja!2sjp!4v1631234567890!5m2!1sja!2sjp" },
    box: { id: "box", label: "BOX", src: SERVICES[1].url }
  };
  const [tabs, setTabs] = useState(["reearth","googlemap","box"]);
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
              {SERVICES.filter(s => !["Re:Earth", "BOX"].includes(s.name)).map((service) => (
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
            </div>

            <div style={{ marginTop: "auto", borderTop: "1px solid #ccc", paddingTop: "10px" }}>
              <a
                href={selectedUrl}
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
            src={SERVICES[0].url}
            width="100%"
            height="100%"
            style={{ border: "none", flex: 1, display: activeTab === "reearth" ? "block" : "none" }}
            allowFullScreen
            allow="clipboard-read; clipboard-write"
          />
          <iframe
            title="BOX Frame"
            src={SERVICES[1].url}
            width="100%"
            height="100%"
            style={{ border: "none", flex: 1, display: activeTab === "box" ? "block" : "none" }}
            allowFullScreen
            allow="clipboard-read; clipboard-write"
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
        </div>
      </div>
    </div>
  );
}
