import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import { ConfigProvider, App, Button, Layout, theme as antdTheme } from "antd";
import "./index.css";
import { defaultConfig, loadGlobalConfig, storeGlobalConfig, Theme } from "./lib/global_config";
import { SunOutlined } from "@ant-design/icons";

type ConfigSettingProps = {
  onSwitchTheme: () => void;
};
const ConfigSetting: React.FC<ConfigSettingProps> = ({ onSwitchTheme }) => {
  return (
    <>
      <Button onClick={onSwitchTheme}>
        <SunOutlined />
      </Button>
    </>
  );
};

const Root: React.FC = () => {
  const config = useRef(defaultConfig);
  const [theme, setTheme] = useState<Theme>(defaultConfig.theme);
  useEffect(() => {
    (async () => {
      const config = await loadGlobalConfig();
      setTheme(config.theme);
    })();
  }, []);
  const handleSwitchTheme = async () => {
    const newTheme = theme === Theme.Dark ? Theme.Light : Theme.Dark;
    setTheme(newTheme);
    config.current.theme = newTheme;
    await storeGlobalConfig(config.current);
  };

  return (
    <React.StrictMode>
      <ConfigProvider
        theme={{
          algorithm: theme === Theme.Dark ? [antdTheme.darkAlgorithm] : [],
        }}
      >
        <App>
          <Layout style={{ height: "100vh" }}>
            <ConfigSetting onSwitchTheme={handleSwitchTheme}></ConfigSetting>
          </Layout>
        </App>
      </ConfigProvider>
    </React.StrictMode>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<Root />);
