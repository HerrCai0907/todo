import { ipc } from "./ipc";

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

export async function storeGlobalConfig(config: GlobalConfig): Promise<void> {
  await ipc<string | undefined>("set_storage", { key, value: JSON.stringify(config) });
}
