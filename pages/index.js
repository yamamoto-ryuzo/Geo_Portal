import { useState } from "react";

const SERVICES = [
  { name: "Re:Earth", url: "https://c-01kcwqbkykrk15apgxeqrvr6rv.visualizer.reearth.io/" },
  { name: "BOX", url: "https://app.box.com/embed/s/28rzb0y0oqh25dv16p6j4swayqbg6z8a?sortColumn=date" },
  { name: "Backlog", url: "https://backlog.com/ja/" },
];

export default function Home() {
  const [selectedUrl, setSelectedUrl] = useState(SERVICES[0].url);

  return (
    <div style={{ display: "flex", height: "100vh", overflow: "hidden" }}>
      {/* 左サイドバー: コントロール */}
      <div style={{
        width: "250px",
        backgroundColor: "#f5f5f5",
        borderRight: "1px solid #ddd",
        padding: "20px",
        display: "flex",
        flexDirection: "column"
      }}>
        <h1 style={{ fontSize: "1.2rem", marginBottom: "20px" }}>ポータルサイト</h1>

        <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
          {SERVICES.map((service) => (
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

      {/* 右メインエリア: コンテンツ */}
      <div style={{ flex: 1, display: "flex", flexDirection: "column" }}>
        <iframe
          title="Service Frame"
          src={selectedUrl}
          width="100%"
          height="100%"
          style={{ border: "none", flex: 1 }}
          allowFullScreen
          allow="clipboard-read; clipboard-write"
        />
      </div>
    </div>
  );
}
