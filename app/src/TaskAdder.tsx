import React from "react";
import { App } from "antd";
import { ipc } from "./lib/ipc";
import { error, success } from "./lib/notification";
import EditableLine from "./component/EditableLine";

type P = {
  onNotifyServer: () => void;
};

const TaskAdder: React.FC<P> = ({ onNotifyServer }) => {
  const appRef = App.useApp();

  const handleSubmit = async (text: string) => {
    try {
      await ipc<null>("put_task", { task: text });
      success(appRef, `create new task successfully`, text);
      onNotifyServer();
    } catch (e) {
      if (e instanceof Error) error(appRef, "Error fetching todo list", e.message);
    }
  };

  return <EditableLine onSubmit={handleSubmit} onCancel={() => {}} placeholder={"enter new task"}></EditableLine>;
};

export default TaskAdder;
