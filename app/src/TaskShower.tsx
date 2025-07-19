import React, { useState } from "react";
import { Task } from "./lib/types";
import SelectedTaskItem from "./SelectedTaskItem";
import EditingTaskItem from "./EditingTaskItem";

type P = {
  tasks: Task[];
  onNotifyServer: () => void;
};

type RowProps = {
  onNotifyServer: () => void;
  task: Task;
};

const Row: React.FC<RowProps> = ({ task, onNotifyServer }: RowProps) => {
  const [hover, setHover] = useState<boolean>(false);
  const [editing, setEditing] = useState<boolean>(false);
  const handleMouseEnter = () => {
    setHover(true);
  };
  const handleMouseLeave = () => {
    setHover(false);
  };
  const handleEditing = () => {
    setEditing(true);
  };
  const handleEditingSubmit = () => {
    setEditing(false);
    onNotifyServer();
  };

  if (editing) {
    return <EditingTaskItem task={task} onSubmit={handleEditingSubmit} />;
  }
  if (hover) {
    return (
      <div onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} style={{ backgroundColor: "#441d12" }}>
        <SelectedTaskItem task={task} onEditing={handleEditing} onNotifyServer={onNotifyServer}></SelectedTaskItem>
      </div>
    );
  }
  return (
    <div onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} style={{}}>
      {task.task}
    </div>
  );
};

const TaskShower: React.FC<P> = ({ tasks, onNotifyServer }: P) => {
  return (
    <>
      {tasks.map((task) => (
        <Row key={task.id} task={task} onNotifyServer={onNotifyServer}></Row>
      ))}
    </>
  );
};

export default TaskShower;
