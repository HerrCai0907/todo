import React, { useState } from "react";
import { ConfigProvider, Table, TableProps } from "antd";
import { Task } from "./types";
import SelectedTaskItem from "./SeklectedTaskItem";

type P = {
  tasks: Task[];
  onNotifyServer: () => void;
};

const TaskShower: React.FC<P> = ({ tasks, onNotifyServer }: P) => {
  const [selectedState, setSelectedState] = useState<{
    currentSelectedId: number | undefined;
    lastSelectedId: number | undefined;
  }>({ currentSelectedId: undefined, lastSelectedId: undefined });

  const columns: TableProps<Task>["columns"] = [
    {
      dataIndex: "task",
      key: "id",
      render: (text: Task["task"], record, _) => {
        if (selectedState.currentSelectedId == record.id) {
          return <SelectedTaskItem record={record} onNotifyServer={onNotifyServer}></SelectedTaskItem>;
        } else {
          return <div>{text}</div>;
        }
      },
      shouldCellUpdate(record, prevRecord) {
        return record.id === selectedState.currentSelectedId || prevRecord.id === selectedState.lastSelectedId;
      },
    },
  ];

  let props: TableProps<Task> = {
    columns,
    dataSource: tasks,
    rowKey: "id",
    size: "small",
    bordered: false,
    pagination: { position: ["topCenter", "bottomCenter"], hideOnSinglePage: true, showSizeChanger: false },
    onRow: (record) => {
      return {
        onMouseEnter: () => {
          setSelectedState({ currentSelectedId: record.id, lastSelectedId: selectedState.currentSelectedId });
        },
        onMouseLeave: () => {
          setSelectedState({ currentSelectedId: undefined, lastSelectedId: selectedState.currentSelectedId });
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
