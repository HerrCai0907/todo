import React from "react";
import { ipc } from "./lib/ipc";
import { Task } from "./lib/types";
import { error, success } from "./lib/notification";
import { App, Checkbox, Row, Dropdown, MenuProps } from "antd";

type P = {
  task: Task;
  onEditing: () => void;
  onNotifyServer: () => void;
};

const SelectedTaskItem: React.FC<P> = ({ task, onEditing, onNotifyServer }) => {
  const appRef = App.useApp();

  const menuItems: MenuProps["items"] = [
    {
      label: "edit",
      key: "edit",
      onClick: onEditing,
    },
  ];
  return (
    <Dropdown menu={{ items: menuItems }} trigger={["contextMenu"]}>
      <Row justify="space-between" style={{ width: "100%" }}>
        {task.task}
        <Checkbox
          onClick={() => {
            (async () => {
              try {
                await ipc<null>("patch_task_status_done", { id: task.id });
                success(appRef, `finished`, task.task);
                onNotifyServer();
              } catch (e) {
                if (e instanceof Error) error(appRef, "An error occurred while completing the task", e.message);
              }
            })();
          }}
        ></Checkbox>
      </Row>
    </Dropdown>
  );
};

export default SelectedTaskItem;
