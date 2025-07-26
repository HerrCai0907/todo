import React from "react";
import { App } from "antd";
import { ipc } from "./lib/ipc";
import { error, success } from "./lib/notification";
import { Task } from "./lib/types";
import EditableLine from "./component/EditableLine";

type P = {
  task: Task;
  onFinished: () => void;
};
const EditingTaskItem: React.FC<P> = ({ task: record, onFinished }) => {
  const appRef = App.useApp();

  const handleSubmit = async (text: string) => {
    try {
      await ipc<null>("patch_task_task", { id: record.id, task: text });
      success(appRef, `edit task successfully`, text);
      onFinished();
    } catch (e) {
      if (e instanceof Error) error(appRef, "failed to edit task", e.message);
    }
  };
  const handleCancel = () => {
    onFinished();
  };
  return <EditableLine onSubmit={handleSubmit} onCancel={handleCancel} initText={record.task}></EditableLine>;
};

export default EditingTaskItem;
