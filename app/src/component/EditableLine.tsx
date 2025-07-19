import React, { useState } from "react";
import { PlusCircleOutlined } from "@ant-design/icons";
import { Button, Flex, Input } from "antd";
const { TextArea } = Input;

type P = {
  initText?: string;
  placeholder?: string;
  onSubmit: (text: string) => void;
};

const EditableLine: React.FC<P> = ({ initText, placeholder, onSubmit }) => {
  const [inputText, setInputText] = useState<string>(initText ?? "");

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputText(e.target.value);
  };
  const handleButtonClick = async () => {
    const inputTextTrimmed = inputText.trim();
    // do not add empty task to avoid unintended click
    if (inputTextTrimmed.length == 0) return;
    onSubmit(inputTextTrimmed);
    setInputText("");
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
        placeholder={placeholder}
        autoSize
        allowClear
        onChange={handleInputChange}
        value={inputText}
      />
    </Flex>
  );
};

export default EditableLine;
