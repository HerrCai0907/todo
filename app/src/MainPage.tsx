import React, { useEffect } from "react";
import { Task } from "./lib/types";
import TaskShower from "./TaskShower";
import { ipc } from "./lib/ipc";
import TaskAdder from "./TaskAdder";
import { error } from "./lib/notification";
import { App, Layout } from "antd";
import { info } from "@tauri-apps/plugin-log";

const MainPage: React.FC = () => {
  const appRef = App.useApp();
  const [tasks, setTasks] = React.useState<Task[] | undefined>(undefined);
  const handleNotifyServer = async () => {
    try {
      const newTasks = (await ipc<Task[]>("get_tasks")).reverse();
      setTasks((prevTasks) => {
        if (JSON.stringify(prevTasks) != JSON.stringify(newTasks)) {
          info(`tasks changed`);
          return newTasks;
        } else {
          return prevTasks;
        }
      });
    } catch (e) {
      if (e instanceof Error) error(appRef, "Error fetching todo list", e.message);
    }
  };

  useEffect(() => {
    handleNotifyServer();
    let handler = setInterval(handleNotifyServer, 1000);
    return () => clearInterval(handler);
  }, []);
  if (tasks === undefined) {
    return <div>Loading...</div>;
  }
  return (
    <Layout>
      <Layout.Content style={{ padding: "0 4px" }}>
        <TaskAdder onNotifyServer={handleNotifyServer}></TaskAdder>
        <TaskShower tasks={tasks} onNotifyServer={handleNotifyServer} />
      </Layout.Content>
    </Layout>
  );
};

export default MainPage;
