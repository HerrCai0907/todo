import React, { useRef, useState } from "react";
import { PlusCircleOutlined } from "@ant-design/icons";
import { Button, Flex, Input } from "antd";
const { TextArea } = Input;

type P = {
  initText?: string;
  placeholder?: string;
  onSubmit: (text: string) => void;
};
// wasm perf 适配 perfetto

const EditableLine: React.FC<P> = ({ initText, placeholder, onSubmit }) => {
  const [inputText, setInputText] = useState<string>(initText ?? "");
  const lastEnterTime = useRef<number>(0);

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputText(e.target.value);
  };
  const handleSubmit = async () => {
    const inputTextTrimmed = inputText.trim();
    // do not add empty task to avoid unintended click
    if (inputTextTrimmed.length == 0) return;
    onSubmit(inputTextTrimmed);
    setInputText("");
  };
  const handlePressEnter = () => {
    // when user click enter quickly, we want to submit the input
    if (Date.now() - lastEnterTime.current <= 200) {
      handleSubmit();
    }
    lastEnterTime.current = Date.now();
  };

  return (
    <Flex vertical={false} align={"center"} gap={"small"}>
      <Button
        onClick={handleSubmit}
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
        onPressEnter={handlePressEnter}
      />
    </Flex>
  );
};

export default EditableLine;
