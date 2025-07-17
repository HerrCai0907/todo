import React from "react";
import { Table, TableProps } from "antd";
import { Task } from "./types";

const columns: TableProps<Task>["columns"] = [
  { title: "ID", dataIndex: "id", key: "id" },
  { title: "Task", dataIndex: "task", key: "task" },
  { title: "Create Time", dataIndex: "create_time", key: "create_time" },
];

type P = {
  tasks: Task[];
};

const TodoList: React.FC<P> = ({ tasks }: P) => {
  return <Table columns={columns} dataSource={tasks} />;
};

export default TodoList;
