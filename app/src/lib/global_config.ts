import { ipc } from "./ipc";
import { listen } from "@tauri-apps/api/event";

const key = "user_config";

export enum Theme {
  Dark,
  Light,
}

export interface GlobalConfig {
  theme: Theme;
}

export const defaultConfig: GlobalConfig = {
  theme: Theme.Light,
};

export async function loadGlobalConfig(): Promise<GlobalConfig> {
  const configStr = await ipc<string | undefined>("get_storage", { key });
  if (configStr === undefined) {
    return defaultConfig;
  }
  const config = JSON.parse(configStr);
  return { ...defaultConfig, ...config };
}

export async function registerOnGlobalConfigChanged(callback: (config: GlobalConfig) => void) {
  callback(await loadGlobalConfig());
  await ipc<void>("register_event_on_storage_change", { key });
  listen<string>("kv_changed", async ({ payload }) => {
    if (payload === key) {
      callback(await loadGlobalConfig());
    }
  });
}

export async function storeGlobalConfig(config: GlobalConfig): Promise<void> {
  await ipc<string | undefined>("set_storage", { key, value: JSON.stringify(config) });
}
