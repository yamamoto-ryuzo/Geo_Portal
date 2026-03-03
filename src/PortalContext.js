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
  }, []);

  const [previewReearth, setPreviewReearth] = useState(reearthUrl);
  const [previewBox, setPreviewBox] = useState(boxUrl);

  useEffect(() => {
    setPreviewReearth(reearthUrl);
  }, [reearthUrl]);
  useEffect(() => {
    setPreviewBox(boxUrl);
  }, [boxUrl]);

  function applyPreview() {
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
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
  function applyPreviewAndSave() {
    try {
      // Persist preview values first to avoid async state timing issues
      localStorage.setItem(STORAGE_REEARTH, previewReearth || "");
      localStorage.setItem(STORAGE_BOX, previewBox || "");
    } catch (e) {
      return false;
    }
    setReearthUrl(previewReearth);
    setBoxUrl(previewBox);
    return true;
  }

  function resetTo(initialVals = {}) {
    const queryReearth = getReearthFromQuery();
    const { reearth = queryReearth || initialReearth, box = initialBox } = initialVals;
    setPreviewReearth(reearth || "");
    setPreviewBox(box || "");
    setReearthUrl(reearth || "");
    setBoxUrl(box || "");
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
        previewReearth,
        previewBox,
        setPreviewReearth,
        setPreviewBox,
        applyPreview,
        save,
        resetTo,
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
