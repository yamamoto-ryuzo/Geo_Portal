import React, { createContext, useContext, useEffect, useState } from "react";

const PortalContext = createContext(null);

export function PortalProvider({ children, initialReearth, initialBox, initialSettings }) {
  // Settings are persisted as a single JSON object under 'portal:settings'
  // initialSettings: getStaticProps でサーバーサイドから渡されるデフォルト設定

  function getReearthFromQuery() {
    if (typeof window === "undefined") return "";
    const params = new URLSearchParams(window.location.search);
    return params.get("EARTH") || params.get("earth") || "";
  }

  // project_path は常に string[] として扱う（文字列・null・undefined も正規化）
  function toProjectPathArray(p) {
    if (!p) return [];
    if (Array.isArray(p)) return p;
    return [p];
  }

  // Initialize with provided defaults; read persisted values on client mount.
  // Read saved portal settings (single-object) if available.
  function getSavedSettings() {
    if (typeof window === "undefined") return null;
    try {
      const raw = localStorage.getItem("portal:settings");
      if (!raw) return null;
      return JSON.parse(raw);
    } catch (e) {
      return null;
    }
  }

  const [reearthUrl, setReearthUrl] = useState(() => {
    const saved = getSavedSettings();
    const q = getReearthFromQuery();
    if (saved && saved.reearth_url) return saved.reearth_url;
    if (q) return q;
    if (initialSettings && initialSettings.reearth_url) return initialSettings.reearth_url;
    return initialReearth || "";
  });

  const [boxUrl, setBoxUrl] = useState(() => {
    const saved = getSavedSettings();
    if (saved && saved.box_url) return saved.box_url;
    if (initialSettings && initialSettings.box_url) return initialSettings.box_url;
    return initialBox || "";
  });

  const [qgisProfile, setQgisProfile] = useState(() => {
    const saved = getSavedSettings();
    if (saved && saved.profile) return saved.profile;
    return (initialSettings && initialSettings.profile) || "default";
  });

  const [qgisProjectPath, setQgisProjectPath] = useState(() => {
    const saved = getSavedSettings();
    if (saved && saved.project_path !== undefined) return toProjectPathArray(saved.project_path);
    return toProjectPathArray(initialSettings && initialSettings.project_path);
  });

  const [pathAliases, setPathAliases] = useState(() => {
    const saved = getSavedSettings();
    if (saved && saved.path_aliases) return saved.path_aliases;
    return (initialSettings && initialSettings.path_aliases) || { BOX: "%USERPROFILE%\\Box" };
  });

  const [rcloneMounts, setRcloneMounts] = useState(() => {
    const saved = getSavedSettings();
    if (saved && saved.rclone_mounts) return saved.rclone_mounts;
    return (initialSettings && initialSettings.rclone_mounts) || [];
  });

  const [previewReearth, setPreviewReearth] = useState(reearthUrl);
  const [previewBox, setPreviewBox] = useState(boxUrl);
  const [previewQgisProfile, setPreviewQgisProfile] = useState(qgisProfile);
  const [previewQgisProjectPath, setPreviewQgisProjectPath] = useState(toProjectPathArray(qgisProjectPath));
  const [previewPathAliases, setPreviewPathAliases] = useState(pathAliases);
  const [previewRcloneMounts, setPreviewRcloneMounts] = useState(rcloneMounts);

  // 起動時: localStorage に保存値がなければ initialSettings (SSR) を適用済みのためフェッチ不要

  useEffect(() => {
    setPreviewReearth(reearthUrl);
  }, [reearthUrl]);
  useEffect(() => {
    setPreviewBox(boxUrl);
  }, [boxUrl]);

  // Debug: expose and log previewBox changes for troubleshooting
  useEffect(() => {
    setPreviewQgisProfile(qgisProfile);
  }, [qgisProfile]);
  useEffect(() => {
    setPreviewQgisProjectPath(toProjectPathArray(qgisProjectPath));
  }, [qgisProjectPath]);
  useEffect(() => {
    setPreviewPathAliases(pathAliases);
  }, [pathAliases]);
  useEffect(() => {
    setPreviewRcloneMounts(rcloneMounts);
  }, [rcloneMounts]);

  function applyPreview() {
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
    setQgisProfile(previewQgisProfile);
    setQgisProjectPath(previewQgisProjectPath);
    setPathAliases(previewPathAliases);
    setRcloneMounts(previewRcloneMounts);
  }

  // Apply loaded settings (from file) to preview and active states in one place
  function applyLoadedSettings(data = {}) {
    if (data.reearth_url) {
      setPreviewReearth(data.reearth_url);
      setReearthUrl(data.reearth_url);
    }
    if (data.box_url) {
      setPreviewBox(data.box_url);
      setBoxUrl(data.box_url);
    }
    if (data.profile) {
      setPreviewQgisProfile(data.profile);
      setQgisProfile(data.profile);
    }
    if (data.project_path !== undefined) {
      const arr = toProjectPathArray(data.project_path);
      setPreviewQgisProjectPath(arr);
      setQgisProjectPath(arr);
    }
    if (data.path_aliases) {
      setPreviewPathAliases(data.path_aliases);
      setPathAliases(data.path_aliases);
    }
    if (data.rclone_mounts !== undefined) {
      setPreviewRcloneMounts(data.rclone_mounts);
      setRcloneMounts(data.rclone_mounts);
    }
    // Persist the whole settings object for full restore
    try {
      if (typeof window !== "undefined") {
        localStorage.setItem('portal:settings', JSON.stringify(data));
      }
    } catch (e) {}
  }

  

  // File upload handler
  function loadSettingsFromFile(file) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        try {
          const data = JSON.parse(e.target.result);
          applyLoadedSettings(data);
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
      const payload = {
        ...(getSavedSettings() || {}),
        profile: qgisProfile,
        project_path: qgisProjectPath,
        reearth_url: reearthUrl,
        box_url: boxUrl,
        path_aliases: pathAliases,
        rclone_mounts: rcloneMounts
      };
      localStorage.setItem('portal:settings', JSON.stringify(payload));
      return true;
    } catch (e) {
      return false;
    }
  }

  // Apply the preview values and persist them immediately.
  async function applyPreviewAndSave() {
    try {
      const payload = {
        ...(getSavedSettings() || {}),
        profile: previewQgisProfile,
        project_path: previewQgisProjectPath,
        reearth_url: previewReearth,
        box_url: previewBox,
        path_aliases: previewPathAliases,
        rclone_mounts: previewRcloneMounts
      };
      localStorage.setItem('portal:settings', JSON.stringify(payload));
    } catch (e) {}
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
    setQgisProfile(previewQgisProfile);
    setQgisProjectPath(previewQgisProjectPath);
    setPathAliases(previewPathAliases);
    setRcloneMounts(previewRcloneMounts);
    // Local launcher integration removed; persisted to localStorage only.
    return true;
  }

  // Save current preview/settings to a local file using File System Access API
  async function saveToFs(filename = "qgis_settings.json") {
    if (typeof window === "undefined") return false;
    const payload = {
      ...(getSavedSettings() || {}),
      profile: previewQgisProfile,
      project_path: previewQgisProjectPath,
      reearth_url: previewReearth,
      box_url: previewBox,
      path_aliases: previewPathAliases,
      rclone_mounts: previewRcloneMounts
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
    setPreviewQgisProjectPath([]);
    setPreviewPathAliases({ BOX: "%USERPROFILE%\\Box" });
    setPreviewRcloneMounts([]);
    setReearthUrl(reearth || "");
    setBoxUrl(box || "");
    setQgisProfile("default");
    setQgisProjectPath([]);
    setPathAliases({ BOX: "%USERPROFILE%\\Box" });
    setRcloneMounts([]);
    try {
      localStorage.removeItem('portal:settings');
    } catch (e) {}
  }

  return (
    <PortalContext.Provider
      value={{
        reearthUrl,
        boxUrl,
        qgisProfile,
        qgisProjectPath,
        previewReearth,
        previewQgisProfile,
        previewQgisProjectPath,
        previewPathAliases,
        previewRcloneMounts,
        setPreviewReearth,
        setPreviewBox,
        setPreviewQgisProfile,
        setPreviewQgisProjectPath,
        setPreviewPathAliases,
        setPreviewRcloneMounts,
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
