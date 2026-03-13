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

  // Initialize with provided defaults; read persisted values on client mount.
  const [reearthUrl, setReearthUrl] = useState(initialReearth || "");
  const [boxUrl, setBoxUrl] = useState(initialBox || "");
  const [qgisProfile, setQgisProfile] = useState("default");
  const [qgisProjectPath, setQgisProjectPath] = useState("");

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
          setPreviewQgisProfile(data.profile);
        }
        if (data.project_path !== undefined) {
          setQgisProjectPath(data.project_path);
          setPreviewQgisProjectPath(data.project_path);
        }
        if (data.reearth_url) {
          setReearthUrl(data.reearth_url);
          setPreviewReearth(data.reearth_url);
        }
        if (data.box_url) {
          setBoxUrl(data.box_url);
          setPreviewBox(data.box_url);
        }
      })
      .catch(err => {
        console.warn("Could not load QGIS settings from local launcher", err);
      });
  }, []);

  const [previewReearth, setPreviewReearth] = useState(reearthUrl);
  const [previewBox, setPreviewBox] = useState(boxUrl);
  const [previewQgisProfile, setPreviewQgisProfile] = useState(qgisProfile);
  const [previewQgisProjectPath, setPreviewQgisProjectPath] = useState(qgisProjectPath);

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
    setPreviewQgisProjectPath(qgisProjectPath);
  }, [qgisProjectPath]);

  function applyPreview() {
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
    setQgisProfile(previewQgisProfile);
    setQgisProjectPath(previewQgisProjectPath);
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

    try {
      await fetch("http://127.0.0.1:12345/settings", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          profile: previewQgisProfile,
          project_path: previewQgisProjectPath,
          reearth_url: previewReearth,
          box_url: previewBox
        })
      });
      return true;
    } catch (e) {
      console.warn("Could not save QGIS settings to local launcher", e);
      return false;
    }
  }

  function resetTo(initialVals = {}) {
    const queryReearth = getReearthFromQuery();
    const { reearth = queryReearth || initialReearth, box = initialBox } = initialVals;
    setPreviewReearth(reearth || "");
    setPreviewBox(box || "");
    setPreviewQgisProfile("default");
    setPreviewQgisProjectPath("");
    setReearthUrl(reearth || "");
    setBoxUrl(box || "");
    setQgisProfile("default");
    setQgisProjectPath("");
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
        previewReearth,
        previewBox,
        previewQgisProfile,
        previewQgisProjectPath,
        setPreviewReearth,
        setPreviewBox,
        setPreviewQgisProfile,
        setPreviewQgisProjectPath,
        applyPreview,
        save,
        resetTo,
        applyPreviewAndSave
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
