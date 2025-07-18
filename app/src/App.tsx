import React, { useEffect } from "react";
import { ConfigProvider, theme } from "antd";
import { Task } from "./types";
import TodoList from "./TodoList";
import { ipc } from "./ipc";
import TaskAdder from "./TaskAdder";

const App: React.FC = () => {
  const [todoList, setTodoList] = React.useState<Task[] | undefined>(undefined);
  const fn = async () => {
    try {
      setTodoList(await ipc<Task[]>("get_tasks"));
    } catch (_) {}
  };

  useEffect(() => {
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
      <TaskAdder onPost={fn}></TaskAdder>
      <TodoList tasks={todoList} onPost={fn} />
    </ConfigProvider>
  );
};

export default App;
