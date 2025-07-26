import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import MainPage from "./MainPage";
import { ConfigProvider, theme as antdTheme, App } from "antd";
import "./index.css";
import { defaultConfig, registerOnGlobalConfigChanged, Theme } from "./lib/global_config";
import { GlobalConfigContext } from "./component/ConfigContext";

const Root = () => {
  const [theme, setTheme] = useState<Theme>(defaultConfig.theme);
  useEffect(() => {
    registerOnGlobalConfigChanged(({ theme }) => {
      setTheme(theme);
    });
  }, []);

  return (
    <React.StrictMode>
      <GlobalConfigContext.Provider value={{ theme }}>
        <ConfigProvider
          theme={{
            algorithm: theme === Theme.Dark ? [antdTheme.darkAlgorithm] : [],
          }}
        >
          <App>
            <MainPage />
          </App>
        </ConfigProvider>
      </GlobalConfigContext.Provider>
    </React.StrictMode>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<Root />);
