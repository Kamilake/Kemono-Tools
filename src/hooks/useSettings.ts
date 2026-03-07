import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "../types";

const defaultSettings: Settings = {
  server: "https://kemono.cr/api",
  service: "fantia",
  session: "",
  download_path: "./downloads",
  username: "",
  password: "",
  downloads: {},
};

export function useSettings() {
  const [settings, setSettings] = useState<Settings>(defaultSettings);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    invoke<Settings>("get_settings")
      .then((s) => {
        setSettings(s);
        setLoaded(true);
      })
      .catch(() => {
        setLoaded(true);
      });
  }, []);

  const updateSettings = useCallback(async (partial: Partial<Settings>) => {
    const updated = { ...settings, ...partial };
    setSettings(updated);
    await invoke("save_settings", { settings: updated });
  }, [settings]);

  return { settings, updateSettings, loaded };
}
