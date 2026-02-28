import { useState } from "react";

const SERVICES = [
  { name: "Re:Earth", url: "https://c-01kcwqbkykrk15apgxeqrvr6rv.visualizer.reearth.io/" },
  { name: "BOX", url: "https://app.box.com/login" },
  { name: "Backlog", url: "https://your-backlog-url.example.com" },
];

export default function Home() {
  const [selectedUrl, setSelectedUrl] = useState(SERVICES[0].url);

  return (
    <div style={{ maxWidth: 900, margin: "0 auto", padding: 32 }}>
      <h1>ポータルサイト</h1>
      <div style={{ marginBottom: 16 }}>
        <label htmlFor="service-select">サービス選択: </label>
        <select
          id="service-select"
          value={selectedUrl}
          onChange={e => setSelectedUrl(e.target.value)}
        >
          {SERVICES.map(s => (
            <option key={s.name} value={s.url}>{s.name}</option>
          ))}
        </select>
      </div>
      <iframe
        title="Service Frame"
        src={selectedUrl}
        width="100%"
        height="600"
        style={{ border: "1px solid #ccc", borderRadius: 8 }}
      />
    </div>
  );
}
