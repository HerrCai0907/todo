import React, { useEffect, useState } from "react";
import { ipc } from "./lib/ipc";
import { Task } from "./lib/types";
import { error, success } from "./lib/notification";
import { App, Button, Checkbox, Dropdown, MenuProps } from "antd";
import { DownOutlined, HolderOutlined } from "@ant-design/icons";
import { SyntheticListenerMap } from "@dnd-kit/core/dist/hooks/utilities";

type DragProps = {
  setActivatorNodeRef: (element: HTMLElement | null) => void;
  listeners: SyntheticListenerMap | undefined;
};

type P = {
  task: Task;
  onEditing: () => void;
  onNotifyServer: () => void;
  dragProps: DragProps;
  onDropDownStatusChanged: (open: boolean) => void;
};

const SelectedTaskItem: React.FC<P> = ({ task, dragProps, onDropDownStatusChanged, onEditing, onNotifyServer }) => {
  const appRef = App.useApp();
  const [isFullRendered, setIsFullRendered] = useState(false);

  useEffect(() => {
    setTimeout(() => {
      setIsFullRendered(true);
    }, 100);
  }, []);

  if (!isFullRendered) {
    return <>{task.task}</>;
  }

  const menuItems: MenuProps["items"] = [
    {
      label: "edit",
      key: "edit",
      onClick: () => {
        onEditing();
        onDropDownStatusChanged(false);
      },
    },
  ];
  return (
    <div style={{ display: "flex", alignItems: "center" }}>
      <div>{task.task}</div>
      <div style={{ marginLeft: "auto", display: "flex", gap: "4px", padding: "0 4px" }}>
        <DragHandle setActivatorNodeRef={dragProps.setActivatorNodeRef} listeners={dragProps.listeners}></DragHandle>
        <Dropdown menu={{ items: menuItems }} onOpenChange={onDropDownStatusChanged} trigger={["click"]}>
          <DownOutlined />
        </Dropdown>
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
      </div>
    </div>
  );
};

export default SelectedTaskItem;

const DragHandle: React.FC<DragProps> = ({ setActivatorNodeRef, listeners }) => {
  return (
    <Button type="text" size="small" style={{ cursor: "move" }} ref={setActivatorNodeRef} {...listeners}>
      <HolderOutlined />
    </Button>
  );
};
