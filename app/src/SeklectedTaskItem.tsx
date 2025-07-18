import React from "react";
import { ipc } from "./ipc";
import { Task } from "./types";
import { error, success } from "./notification";
import { App, Checkbox } from "antd";

type P = {
  record: Task;
  onNotifyServer: () => void;
};

const SelectedTaskItem: React.FC<P> = ({ record, onNotifyServer }) => {
  const appRef = App.useApp();
  return (
    <div>
      {record.task}&nbsp;&nbsp;&nbsp;
      <Checkbox
        onClick={() => {
          (async () => {
            try {
              await ipc<null>("post_task_done", { id: record.id });
              success(appRef, `finished`, record.task);
              onNotifyServer();
            } catch (e) {
              if (e instanceof Error) error(appRef, "An error occurred while completing the task", e.message);
            }
          })();
        }}
      ></Checkbox>
    </div>
  );
};

export default SelectedTaskItem;
