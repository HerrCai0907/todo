import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { TrayIcon } from "@tauri-apps/api/tray";
import { defaultWindowIcon } from "@tauri-apps/api/app";

// const tray = await TrayIcon.new({
//   icon: (await defaultWindowIcon()) ?? undefined,
// });
// console.log(tray);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
