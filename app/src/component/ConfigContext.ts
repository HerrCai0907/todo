import React from "react";
import { defaultConfig, GlobalConfig } from "../lib/global_config";

export const GlobalConfigContext = React.createContext<GlobalConfig>(defaultConfig);
