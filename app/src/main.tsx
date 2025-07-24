import React from "react";
import ReactDOM from "react-dom/client";
import MainPage from "./MainPage";
import { ConfigProvider, theme, App } from "antd";
import "./index.css";

const Root = () => (
  <React.StrictMode>
    <ConfigProvider
      theme={{
        algorithm: [theme.darkAlgorithm],
      }}
    >
      <App>
        <MainPage />
      </App>
    </ConfigProvider>
  </React.StrictMode>
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<Root />);
