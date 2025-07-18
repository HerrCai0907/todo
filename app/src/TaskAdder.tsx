import React, { useState } from "react";
import { PlusCircleOutlined } from "@ant-design/icons";
import { Button, Flex, Input, message } from "antd";
import { ipc } from "./ipc";
const { TextArea } = Input;

type P = {
  onPost: () => void;
};

const TaskAdder: React.FC<P> = ({ onPost }) => {
  const [inputText, setInputText] = useState("");

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputText(e.target.value);
  };
  const handleButtonClick = async () => {
    try {
      await ipc<null>("post_task", { task: inputText.trim() });
      message.success(`finished task '${inputText}' successfully`);
      onPost();
    } catch (e) {}
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
