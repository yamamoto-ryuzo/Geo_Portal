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
  const [reearthUrl, setReearthUrl] = useState(() => {
    if (typeof window !== "undefined") {
      const q = getReearthFromQuery();
      const stored = (() => { try { return localStorage.getItem(STORAGE_REEARTH); } catch (e) { return null; }})();
      if (q) return q;
      if (stored) return stored;
      return initialReearth || "";
    }
    return initialReearth || "";
  });

  const [boxUrl, setBoxUrl] = useState(() => {
    if (typeof window !== "undefined") {
      const stored = (() => { try { return localStorage.getItem(STORAGE_BOX); } catch (e) { return null; }})();
      return stored || initialBox || "";
    }
    return initialBox || "";
  });
  const [qgisProfile, setQgisProfile] = useState("default");
  const [qgisProjectPath, setQgisProjectPath] = useState("");
  const [launcherDir, setLauncherDir] = useState("C:\\qgis_launcher");

  // On client mount, attempt to load persisted values from localStorage.
  useEffect(() => {
    // Note: local launcher HTTP integration removed. Initial values for
    // `reearthUrl` and `boxUrl` are read from localStorage in the lazy
    // initializer above so no further action is required here.
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
          console.log("loadSettingsFromFile: raw content:", e.target.result);
          const data = JSON.parse(e.target.result);
          console.log("loadSettingsFromFile: parsed:", data);
          if (data.profile) {
            setQgisProfile(data.profile);
            setPreviewQgisProfile(data.profile);
          }
          if (data.project_path !== undefined) {
            setQgisProjectPath(data.project_path);
            // When loading from a user-selected file, reflect the exact path in the UI
            setPreviewQgisProjectPath(data.project_path);
          }
          if (data.reearth_url) {
            setReearthUrl(data.reearth_url);
            setPreviewReearth(data.reearth_url);
          }
          if (data.box_url) {
            console.log("loadSettingsFromFile: setting box_url ->", data.box_url);
            setBoxUrl(data.box_url);
            setPreviewBox(data.box_url);
          }
          if (data.settings_dir) {
            setLauncherDir(data.settings_dir);
            setPreviewLauncherDir(data.settings_dir);
          }
          // Values are already applied to state via setBoxUrl/setPreviewBox etc.
          // Do not call applyPreview() here because React state updates are async
          // and calling applyPreview() immediately can overwrite newly set values.
          resolve(true);
        } catch (err) {
          console.error("loadSettingsFromFile: parse error", err);
          reject(err);
        }
      };
      reader.onerror = (err) => reject(err);
      reader.readAsText(file);
    });
  }

  // Load settings from the specified directory via the local launcher API
  async function loadSettingsFromDir(dirPath) {
    // Local launcher integration removed. This operation is no-op in browser context.
    console.warn("loadSettingsFromDir: local launcher integration removed; no action taken");
    return false;
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
    // Local launcher integration removed; do not POST to 127.0.0.1. Persisted to localStorage only.
    return true;
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
