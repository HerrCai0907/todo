import React, { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ConfigProvider, message, theme } from "antd";
import { Task } from "./types";
import TodoList from "./TodoList";

interface SuccessResponse {
  data: Task[];
}

interface ErrorResponse {
  err: string;
}

const App: React.FC = () => {
  const [todoList, setTodoList] = React.useState<Task[] | undefined>(undefined);

  useEffect(() => {
    const fn = async () => {
      try {
        let response = await invoke<string>("get_todo_list");
        console.log(response);
        let res: SuccessResponse | ErrorResponse = JSON.parse(response);
        if ("err" in res) throw new Error(res.err);
        setTodoList(res.data);
      } catch (e) {
        if (e instanceof Error) message.error(`Error fetching todo list\n${e.message}`);
      }
    };
    fn();
    let handler = setInterval(fn, 5 * 1000);
    return () => clearInterval(handler);
  }, []);
  if (todoList === undefined) {
    return <div>Loading...</div>;
  }
  return (
    <ConfigProvider
      theme={{
        algorithm: [theme.darkAlgorithm, theme.compactAlgorithm],
      }}
    >
      <TodoList tasks={todoList} />
    </ConfigProvider>
  );
};

export default App;
