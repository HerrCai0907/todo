import React from "react";
import { Table, TableProps } from "antd";
import { Task } from "./types";

const columns: TableProps<Task>["columns"] = [
  { title: "ID", dataIndex: "id", key: "id" },
  { title: "Task", dataIndex: "task", key: "id" },
  { title: "Create Time", dataIndex: "create_time", key: "id" },
];

type P = {
  tasks: Task[];
};

const TodoList: React.FC<P> = ({ tasks }: P) => {
  return <Table<Task> columns={columns} dataSource={tasks} rowKey={"id"} />;
};

export default TodoList;
