import React, { createContext, useContext, useEffect, useState } from "react";

const PortalContext = createContext(null);

export function PortalProvider({ children, initialReearth, initialBox }) {
  const STORAGE_REEARTH = "portal:reearthUrl";
  const STORAGE_BOX = "portal:boxUrl";

  function getReearthFromQuery() {
    if (typeof window === "undefined") return "";
    const params = new URLSearchParams(window.location.search);
    return params.get("EARTH") || params.get("earth") || "";
  }

  // Normalize preview value for project path: if it's empty or contains path characters,
  // show simple default filename 'ProjectFile.qgs' in the UI input.
  function normalizePreviewProjectPath(p) {
    if (!p) return "ProjectFile.qgs";
    if (typeof p !== "string") return "ProjectFile.qgs";
    if (p.includes('\\') || p.includes('/') || p.includes(':')) return "ProjectFile.qgs";
    return p;
  }

  // Initialize with provided defaults; read persisted values on client mount.
  const [reearthUrl, setReearthUrl] = useState(initialReearth || "");
  const [boxUrl, setBoxUrl] = useState(initialBox || "");
  const [qgisProfile, setQgisProfile] = useState("default");
  const [qgisProjectPath, setQgisProjectPath] = useState("");
  const [launcherDir, setLauncherDir] = useState("C:\\qgis_launcher");

  // On client mount, attempt to load persisted values from localStorage.
  useEffect(() => {
    try {
      const earthFromQuery = getReearthFromQuery();
      const storedRe = localStorage.getItem(STORAGE_REEARTH);
      const storedBox = localStorage.getItem(STORAGE_BOX);

      if (earthFromQuery) {
        setReearthUrl(earthFromQuery);
      } else if (storedRe) {
        setReearthUrl(storedRe);
      }
      if (storedBox) setBoxUrl(storedBox);
    } catch (e) {
      // ignore (localStorage not available)
    }

    // Load QGIS and Portal settings from local launcher API
    fetch("http://127.0.0.1:12345/settings")
      .then(res => res.json())
      .then(data => {
        if (data.profile) {
          setQgisProfile(data.profile);
        }
        if (data.project_path !== undefined) {
          setQgisProjectPath(data.project_path);
          setPreviewQgisProjectPath(normalizePreviewProjectPath(data.project_path));
        }
        if (data.reearth_url) {
          setReearthUrl(data.reearth_url);
          setPreviewReearth(data.reearth_url);
        }
        if (data.box_url) {
          setBoxUrl(data.box_url);
          setPreviewBox(data.box_url);
        }
        if (data.settings_dir) {
          setLauncherDir(data.settings_dir);
          setPreviewLauncherDir(data.settings_dir);
        }
      })
      .catch(err => {
        console.warn("Could not load QGIS settings from local launcher", err);
      });
  }, []);

  const [previewReearth, setPreviewReearth] = useState(reearthUrl);
  const [previewBox, setPreviewBox] = useState(boxUrl);
  const [previewQgisProfile, setPreviewQgisProfile] = useState(qgisProfile);
  const [previewQgisProjectPath, setPreviewQgisProjectPath] = useState(normalizePreviewProjectPath(qgisProjectPath));
  const [previewLauncherDir, setPreviewLauncherDir] = useState(launcherDir);

  useEffect(() => {
    setPreviewReearth(reearthUrl);
  }, [reearthUrl]);
  useEffect(() => {
    setPreviewBox(boxUrl);
  }, [boxUrl]);
  useEffect(() => {
    setPreviewQgisProfile(qgisProfile);
  }, [qgisProfile]);
  useEffect(() => {
    setPreviewQgisProjectPath(normalizePreviewProjectPath(qgisProjectPath));
  }, [qgisProjectPath]);
  useEffect(() => {
    setPreviewLauncherDir(launcherDir);
  }, [launcherDir]);

  function applyPreview() {
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
    setQgisProfile(previewQgisProfile);
    setQgisProjectPath(previewQgisProjectPath);
    setLauncherDir(previewLauncherDir);
  }

  // File upload handler
  function loadSettingsFromFile(file) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        try {
          const data = JSON.parse(e.target.result);
          if (data.profile) {
            setQgisProfile(data.profile);
            setPreviewQgisProfile(data.profile);
          }
          if (data.project_path !== undefined) {
            setQgisProjectPath(data.project_path);
            setPreviewQgisProjectPath(normalizePreviewProjectPath(data.project_path));
          }
          if (data.reearth_url) {
            setReearthUrl(data.reearth_url);
            setPreviewReearth(data.reearth_url);
          }
          if (data.box_url) {
            setBoxUrl(data.box_url);
            setPreviewBox(data.box_url);
          }
          if (data.settings_dir) {
            setLauncherDir(data.settings_dir);
            setPreviewLauncherDir(data.settings_dir);
          }
          resolve(true);
        } catch (err) {
          reject(err);
        }
      };
      reader.onerror = (err) => reject(err);
      reader.readAsText(file);
    });
  }

  // Load settings from the specified directory via the local launcher API
  async function loadSettingsFromDir(dirPath) {
    try {
      // 正規化: ユーザがファイルパス（.../qgis_settings.json）を入力している場合は親ディレクトリに変換
      if (typeof dirPath === "string") {
        const lower = dirPath.toLowerCase();
        if (lower.endsWith("qgis_settings.json")) {
          const idx = Math.max(dirPath.lastIndexOf("\\"), dirPath.lastIndexOf("/"));
          if (idx !== -1) {
            dirPath = dirPath.substring(0, idx);
          }
        }
      }
      const res = await fetch("http://127.0.0.1:12345/settings", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          profile: previewQgisProfile,
          project_path: previewQgisProjectPath,
          reearth_url: previewReearth,
          box_url: previewBox,
          settings_dir: dirPath
        })
      });
      const data = await res.json();

      if (data.profile) {
        setQgisProfile(data.profile);
        setPreviewQgisProfile(data.profile);
      }
      if (data.project_path !== undefined) {
        setQgisProjectPath(data.project_path);
        setPreviewQgisProjectPath(normalizePreviewProjectPath(data.project_path));
      }
      if (data.reearth_url) {
        setReearthUrl(data.reearth_url);
        setPreviewReearth(data.reearth_url);
      }
      if (data.box_url) {
        setBoxUrl(data.box_url);
        setPreviewBox(data.box_url);
      }
      if (data.settings_dir) {
        setLauncherDir(data.settings_dir);
        setPreviewLauncherDir(data.settings_dir);
      }
      return true;
    } catch (e) {
      console.warn("Could not load QGIS settings from specific directory", e);
      return false;
    }
  }

  function save() {
    try {
      localStorage.setItem(STORAGE_REEARTH, reearthUrl || "");
      localStorage.setItem(STORAGE_BOX, boxUrl || "");
      return true;
    } catch (e) {
      return false;
    }
  }

  // Apply the preview values and persist them immediately.
  async function applyPreviewAndSave() {
    try {
      // Persist preview values first to avoid async state timing issues
      localStorage.setItem(STORAGE_REEARTH, previewReearth || "");
      localStorage.setItem(STORAGE_BOX, previewBox || "");
    } catch (e) {
      // ignore
    }
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
    setQgisProfile(previewQgisProfile);
    setQgisProjectPath(previewQgisProjectPath);
    setLauncherDir(previewLauncherDir);

    try {
      // 送信前に settings_dir の正規化（ファイルパスが入っていたらフォルダへ）
      let sendDir = previewLauncherDir;
      if (typeof sendDir === "string") {
        const lower = sendDir.toLowerCase();
        if (lower.endsWith("qgis_settings.json")) {
          const idx = Math.max(sendDir.lastIndexOf("\\"), sendDir.lastIndexOf("/"));
          if (idx !== -1) sendDir = sendDir.substring(0, idx);
        }
      }
      await fetch("http://127.0.0.1:12345/settings", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          profile: previewQgisProfile,
          project_path: previewQgisProjectPath,
          reearth_url: previewReearth,
          box_url: previewBox,
          settings_dir: sendDir
        })
      });
      return true;
    } catch (e) {
      console.warn("Could not save QGIS settings to local launcher", e);
      return false;
    }
  }

  // Save current preview/settings to a local file using File System Access API
  async function saveToFs(filename = "qgis_settings.json") {
    if (typeof window === "undefined") return false;
    const payload = {
      profile: previewQgisProfile,
      project_path: previewQgisProjectPath,
      reearth_url: previewReearth,
      box_url: previewBox,
      settings_dir: previewLauncherDir
    };
    try {
      // Preferred: showSaveFilePicker (allows user to choose exact file)
      if (window.showSaveFilePicker) {
        const handle = await window.showSaveFilePicker({
          suggestedName: filename,
          types: [
            {
              description: "JSON",
              accept: { "application/json": [".json"] }
            }
          ]
        });
        const writable = await handle.createWritable();
        await writable.write(JSON.stringify(payload, null, 2));
        await writable.close();
        return true;
      }

      // Fallback: showDirectoryPicker + write file inside selected directory
      if (window.showDirectoryPicker) {
        const dirHandle = await window.showDirectoryPicker();
        const fileHandle = await dirHandle.getFileHandle(filename, { create: true });
        const writable = await fileHandle.createWritable();
        await writable.write(JSON.stringify(payload, null, 2));
        await writable.close();
        return true;
      }

      return false;
    } catch (e) {
      console.warn("FS API save failed", e);
      return false;
    }
  }

  function resetTo(initialVals = {}) {
    const queryReearth = getReearthFromQuery();
    const { reearth = queryReearth || initialReearth, box = initialBox } = initialVals;
    setPreviewReearth(reearth || "");
    setPreviewBox(box || "");
    setPreviewQgisProfile("default");
    setPreviewQgisProjectPath("ProjectFile.qgs");
    setPreviewLauncherDir("C:\\qgis_launcher");
    setReearthUrl(reearth || "");
    setBoxUrl(box || "");
    setQgisProfile("default");
    setQgisProjectPath("");
    setLauncherDir("C:\\qgis_launcher");
    try {
      localStorage.removeItem(STORAGE_REEARTH);
      localStorage.removeItem(STORAGE_BOX);
    } catch (e) {}
  }

  return (
    <PortalContext.Provider
      value={{
        reearthUrl,
        boxUrl,
        qgisProfile,
        qgisProjectPath,
        launcherDir,
        previewReearth,
        previewQgisProfile,
        previewQgisProjectPath,
        previewLauncherDir,
        setPreviewReearth,
        setPreviewBox,
        setPreviewQgisProfile,
        setPreviewQgisProjectPath,
        setPreviewLauncherDir,
        applyPreview,
        save,
        resetTo,
        applyPreviewAndSave,
        saveToFs,
        loadSettingsFromFile,
        loadSettingsFromDir
      }}
    >
      {children}
    </PortalContext.Provider>
  );
}

export function usePortal() {
  const ctx = useContext(PortalContext);
  if (!ctx) throw new Error("usePortal must be used within PortalProvider");
  return ctx;
}
