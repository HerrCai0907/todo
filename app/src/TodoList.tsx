import React, { useState } from "react";
import { Checkbox, ConfigProvider, message, Table, TableProps } from "antd";
import { Task } from "./types";
import { ipc } from "./ipc";

type P = {
  tasks: Task[];
  onPost: () => void;
};

const TodoList: React.FC<P> = ({ tasks, onPost }: P) => {
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
          return (
            <div>
              {text}&nbsp;&nbsp;&nbsp;
              <Checkbox
                onClick={() => {
                  (async () => {
                    try {
                      await ipc<null>("post_task_done", { id: record.id });
                      message.success(`finished task '${record.task}' successfully`);
                      onPost();
                    } catch (_) {}
                  })();
                }}
              ></Checkbox>
            </div>
          );
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

export default TodoList;
