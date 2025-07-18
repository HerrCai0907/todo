import React, { useState } from "react";
import { PlusCircleOutlined } from "@ant-design/icons";
import { App, Button, Flex, Input } from "antd";
import { ipc } from "./ipc";
import { error, success } from "./notification";
const { TextArea } = Input;

type P = {
  onNotifyServer: () => void;
};

const TaskAdder: React.FC<P> = ({ onNotifyServer }) => {
  const appRef = App.useApp();
  const [inputText, setInputText] = useState("");

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputText(e.target.value);
  };
  const handleButtonClick = async () => {
    const inputTextTrimmed = inputText.trim();
    try {
      await ipc<null>("post_task", { task: inputTextTrimmed });
      success(appRef, `create new task successfully`, inputTextTrimmed);
      onNotifyServer();
    } catch (e) {
      if (e instanceof Error) error(appRef, "Error fetching todo list", e.message);
    }
  };

  return (
    <Flex vertical={false}>
      <Button
        onClick={handleButtonClick}
        type="primary"
        size="small"
        style={{ marginLeft: 8 }}
        icon={<PlusCircleOutlined />}
      />
      <TextArea size="small" placeholder="add new task" autoSize allowClear onChange={handleInputChange} />
    </Flex>
  );
};

export default TaskAdder;
