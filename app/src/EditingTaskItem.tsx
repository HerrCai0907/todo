import React, { useState } from "react";
import { PlusCircleOutlined } from "@ant-design/icons";
import { App, Button, Flex, Input } from "antd";
import { ipc } from "./lib/ipc";
import { error, success } from "./lib/notification";
import { Task } from "./lib/types";
const { TextArea } = Input;

type P = {
  record: Task;
  onSubmit: () => void;
};
const EditingTaskItem: React.FC<P> = ({ record, onSubmit }) => {
  const appRef = App.useApp();
  const [inputText, setInputText] = useState<string>(record.task);

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputText(e.target.value);
  };
  const handleButtonClick = async () => {
    const inputTextTrimmed = inputText.trim();
    // do not add empty task to avoid unintended click
    if (inputTextTrimmed.length == 0) return;
    try {
      await ipc<null>("patch_task_task", { id: record.id, task: inputTextTrimmed });
      success(appRef, `edit task successfully`, inputTextTrimmed);
      onSubmit();
    } catch (e) {
      if (e instanceof Error) error(appRef, "failed to edit task", e.message);
    }
  };

  return (
    <Flex vertical={false} align={"center"} gap={"small"}>
      <Button
        onClick={handleButtonClick}
        type="primary"
        size="small"
        style={{ marginLeft: 8 }}
        icon={<PlusCircleOutlined />}
      />
      <TextArea
        size="small"
        placeholder="add new task"
        autoSize
        allowClear
        onChange={handleInputChange}
        value={inputText}
      />
    </Flex>
  );
};

export default EditingTaskItem;
