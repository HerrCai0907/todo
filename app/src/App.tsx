import React, { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export default () => {
  const [todoList, setTodoList] = React.useState("");
  useEffect(() => {
    (async () => {
      let res = await invoke<string>("get_todo_list");
      console.log(res);
      setTodoList(res);
    })();
  }, []);
  return (
    <div>
      <h1>TODO</h1>
      {todoList}
    </div>
  );
};
