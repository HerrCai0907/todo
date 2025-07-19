import React, { useState } from "react";
import { ConfigProvider, Table, TableProps } from "antd";
import { Task } from "./lib/types";
import SelectedTaskItem from "./SelectedTaskItem";
import EditingTaskItem from "./EditingTaskItem";

type P = {
  tasks: Task[];
  onNotifyServer: () => void;
};

const TaskShower: React.FC<P> = ({ tasks, onNotifyServer }: P) => {
  const [currentSelectedId, setCurrentSelectedId] = useState<number | undefined>(undefined);
  const [currentEditingId, setCurrentEditingId] = useState<number | undefined>(undefined);
  const [pendingUpdateItem, setPendingUpdateItem] = useState<number[]>([]);

  const columns: TableProps<Task>["columns"] = [
    {
      dataIndex: "task",
      key: "id",
      render: (text: Task["task"], record, _) => {
        const id = record.id;
        const onEditing = () => {
          setCurrentEditingId(id);
        };
        if (currentEditingId == record.id) {
          const onSubmit = () => {
            setCurrentEditingId(undefined);
            setPendingUpdateItem([record.id]);
            onNotifyServer();
          };
          return <EditingTaskItem record={record} onSubmit={onSubmit}></EditingTaskItem>;
        } else if (currentSelectedId == record.id) {
          return (
            <SelectedTaskItem record={record} onEditing={onEditing} onNotifyServer={onNotifyServer}></SelectedTaskItem>
          );
        } else {
          return <div>{text}</div>;
        }
      },
      shouldCellUpdate(record) {
        return pendingUpdateItem.includes(record.id);
      },
    },
  ];

  let props: TableProps<Task> = {
    columns,
    dataSource: tasks,
    rowKey: "id",
    size: "small",
    bordered: false,
    pagination: {
      position: ["topCenter", "bottomCenter"],
      hideOnSinglePage: true,
      showSizeChanger: false,
      size: "small",
    },
    onRow: (record) => {
      return {
        onMouseEnter: () => {
          setCurrentSelectedId(record.id);
          if (currentSelectedId) setPendingUpdateItem([currentSelectedId, record.id]);
          else setPendingUpdateItem([record.id]);
        },
        onMouseLeave: () => {
          setCurrentSelectedId(undefined);
          if (currentSelectedId) setPendingUpdateItem([currentSelectedId]);
        },
      };
    },
  };

  return (
    <ConfigProvider
      theme={{
        components: {
          Table: {
            rowHoverBg: "#441d12",
          },
        },
      }}
    >
      <Table<Task> {...props} />
    </ConfigProvider>
  );
};

export default TaskShower;
