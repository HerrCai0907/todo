import React, { useEffect, useState } from "react";
import { Task } from "./lib/types";
import SelectedTaskItem from "./SelectedTaskItem";
import EditingTaskItem from "./EditingTaskItem";
import * as dndCore from "@dnd-kit/core";
import * as dndSort from "@dnd-kit/sortable";
import * as dndCss from "@dnd-kit/utilities";
import * as dndMod from "@dnd-kit/modifiers";
import { trace } from "@tauri-apps/plugin-log";

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

function sortTasksWithUserOrder(userOrder: number[], tasks: Task[]): number[] {
  const unknownOrderTask = tasks.filter((task) => !userOrder.includes(task.id));
  const knownOrderTask = userOrder
    .map((id) => tasks.find((task) => task.id === id))
    .filter((task) => task != undefined);
  const sortedTask = unknownOrderTask.concat(knownOrderTask);
  return sortedTask.map((task) => task.id);
}

const TaskShower: React.FC<P> = ({ tasks, onNotifyServer }: P) => {
  const [sequences, setSequences] = useState<number[] | null>(null);
  const [activeId, setActiveId] = useState<number | null>(null);
  useEffect(() => {
    const newSequences = sortTasksWithUserOrder(sequences ?? [], tasks);
    trace(`change sequences when init: ${sequences}`);
    setSequences(newSequences);
  }, [tasks]);

  if (sequences == null) {
    return <div>Loading...</div>;
  }

  const handleDragStart = ({ active }: dndCore.DragEndEvent) => {
    setActiveId(active.id as number);
  };
  const handleDragEnd = ({ active, over }: dndCore.DragEndEvent) => {
    setActiveId(null);
    if (over && active.id !== over.id) {
      setSequences((prev) => {
        const oldIndex = prev!.indexOf(active.id as number);
        const newIndex = prev!.indexOf(over.id as number);
        const newSequences = dndSort.arrayMove(prev!, oldIndex, newIndex);
        trace(`change sequences when drag: ${newSequences}`);
        return newSequences;
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
          {sequences.map((id) => {
            const index = mapIdToIndex[id];
            if (index === undefined) {
              return null;
            }
            return <Row isDragging={activeId == id} key={id} task={tasks[index]} onNotifyServer={onNotifyServer}></Row>;
          })}
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
