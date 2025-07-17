import React, { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { message } from "antd";
import { Task } from "./types";
import TodoList from "./TodoList";
// import { TrayIcon } from "@tauri-apps/api/tray";
// import { defaultWindowIcon } from "@tauri-apps/api/app";

interface SuccessResponse {
  data: Task[];
}

interface ErrorResponse {
  err: string;
}

const App: React.FC = () => {
  const [todoList, setTodoList] = React.useState<Task[] | null>(null);

  // useEffect(() => {
  //   (async () => {
  //     await TrayIcon.new({
  //       icon: (await defaultWindowIcon()) ?? undefined,
  //     });
  //   })();
  // }, []);

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
    setInterval(fn, 1000);
  }, []);
  return (
    <div>
      <h1>TODO</h1>
      <TodoList tasks={todoList} />
    </div>
  );
};

export default App;
