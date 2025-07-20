import React, { useState } from "react";
import { Task } from "./lib/types";
import SelectedTaskItem from "./SelectedTaskItem";
import EditingTaskItem from "./EditingTaskItem";
import * as dndCore from "@dnd-kit/core";
import * as dndSort from "@dnd-kit/sortable";
import * as dndCss from "@dnd-kit/utilities";
import * as dndMod from "@dnd-kit/modifiers";

type P = {
  tasks: Task[];
  onNotifyServer: () => void;
};

type RowProps = {
  onNotifyServer: () => void;
  task: Task;
  isDragging: boolean;
};

const Row: React.FC<RowProps> = ({ isDragging, task, onNotifyServer }: RowProps) => {
  const [focus, setFocus] = useState<boolean>(isDragging); // dragging force focus
  const [hover, setHover] = useState<boolean>(false);
  const [editing, setEditing] = useState<boolean>(false);
  const { attributes, listeners, setNodeRef, transform, transition, setActivatorNodeRef } = dndSort.useSortable({
    id: task.id,
  });
  const style = {
    transform: dndCss.CSS.Transform.toString(transform),
    transition,
  };

  if (editing) {
    const handleEditingSubmit = () => {
      setEditing(false);
      onNotifyServer();
    };
    return (
      <div ref={setNodeRef} style={style} {...attributes}>
        <EditingTaskItem task={task} onSubmit={handleEditingSubmit} />
      </div>
    );
  }
  if (focus || hover) {
    const handleMouseLeave = () => {
      setHover(false);
    };
    const handleEditing = () => {
      setEditing(true);
    };
    const handleDropDownStatusChanged = (open: boolean) => {
      setFocus(open);
      if (!open) {
        // work around since when drop down is closed, the mouse is already leaved
        setHover(false);
      }
    };
    const keyNode = (
      <div style={{ backgroundColor: "#441d12" }}>
        <SelectedTaskItem
          onDropDownStatusChanged={handleDropDownStatusChanged}
          task={task}
          onNotifyServer={onNotifyServer}
          onEditing={handleEditing}
          dragProps={{ listeners, setActivatorNodeRef }}
        ></SelectedTaskItem>
      </div>
    );
    // in focus mode, we do not want to render mouse leave event
    if (focus) {
      return (
        <div ref={setNodeRef} style={style} {...attributes}>
          {keyNode}
        </div>
      );
    } else {
      return (
        <div onMouseLeave={handleMouseLeave} ref={setNodeRef} style={style} {...attributes}>
          {keyNode}
        </div>
      );
    }
  }
  const handleMouseEnter = () => {
    setHover(true);
  };
  return (
    <div ref={setNodeRef} onMouseEnter={handleMouseEnter} style={style} {...attributes}>
      {task.task}
    </div>
  );
};

const TaskShower: React.FC<P> = ({ tasks, onNotifyServer }: P) => {
  const [sequences, setSequences] = useState(tasks.map((task) => task.id));
  const [activeId, setActiveId] = useState<number | null>(null);

  const handleDragStart = ({ active }: dndCore.DragEndEvent) => {
    setActiveId(active.id as number);
  };
  const handleDragEnd = ({ active, over }: dndCore.DragEndEvent) => {
    setActiveId(null);
    if (over && active.id !== over.id) {
      setSequences((seq) => {
        const oldIndex = seq.indexOf(active.id as number);
        const newIndex = seq.indexOf(over.id as number);
        return dndSort.arrayMove(seq, oldIndex, newIndex);
      });
    }
  };
  const mapIdToIndex = tasks.reduce<Record<number, number>>((prev, curr, index) => ((prev[curr.id] = index), prev), {});
  return (
    <div>
      <dndCore.DndContext
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
        modifiers={[dndMod.restrictToVerticalAxis]}
      >
        <dndSort.SortableContext items={sequences} strategy={dndSort.verticalListSortingStrategy}>
          {sequences.map((id) => (
            <Row
              isDragging={activeId == id}
              key={id}
              task={tasks[mapIdToIndex[id]]}
              onNotifyServer={onNotifyServer}
            ></Row>
          ))}
        </dndSort.SortableContext>
        <dndCore.DragOverlay>
          {activeId ? (
            <Row
              isDragging={true}
              key={activeId}
              task={tasks[mapIdToIndex[activeId]]}
              onNotifyServer={onNotifyServer}
            ></Row>
          ) : null}
        </dndCore.DragOverlay>
      </dndCore.DndContext>
    </div>
  );
};

export default TaskShower;
