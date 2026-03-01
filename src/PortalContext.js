import React, { createContext, useContext, useEffect, useState } from "react";

const PortalContext = createContext(null);

export function PortalProvider({ children, initialReearth, initialBox }) {
  const STORAGE_REEARTH = "portal:reearthUrl";
  const STORAGE_BOX = "portal:boxUrl";

  const [reearthUrl, setReearthUrl] = useState(() => {
    try {
      return localStorage.getItem(STORAGE_REEARTH) || initialReearth || "";
    } catch (e) {
      return initialReearth || "";
    }
  });
  const [boxUrl, setBoxUrl] = useState(() => {
    try {
      return localStorage.getItem(STORAGE_BOX) || initialBox || "";
    } catch (e) {
      return initialBox || "";
    }
  });

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

  function resetTo(initialVals = {}) {
    const { reearth = initialReearth, box = initialBox } = initialVals;
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
