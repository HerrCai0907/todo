import React from "react";
import { Table, TableProps } from "antd";
import { Task } from "./types";

const columns: TableProps<Task>["columns"] = [
  { title: "ID", dataIndex: "id", key: "id" },
  { title: "Task", dataIndex: "task", key: "id" },
  { title: "Create Time", dataIndex: "create_time", key: "id" },
];

type P = {
  tasks: Task[] | null;
};

const TodoList: React.FC<P> = ({ tasks }: P) => {
  if (tasks == null) {
    return <Table<Task> columns={columns} dataSource={[]} rowKey={"id"} loading={true} />;
  } else {
    return <Table<Task> columns={columns} dataSource={tasks} rowKey={"id"} />;
  }
};

export default TodoList;
