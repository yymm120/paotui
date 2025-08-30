import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { cn } from "@/lib/utils";
import type { DeliveryTask } from "@/types";
import { useTaskUpdate } from "@/hooks/query/use-task-update.ts";
import { DistanceIndicator } from "@/components/delivery";
import { CardMenuDropdown } from "@/components/delivery/card/card-menu-dropdown.tsx";
import { useCallback, useContext, useRef, useState } from "react";
import { ViewStatusContext } from "@/components/delivery/DeliveryAppLayout.tsx";
import { Channel, isTauri } from "@tauri-apps/api/core";
import {
  fragment,
  startTxMapWithChannel,
  type Task,
} from "tauri-plugin-txmap-api";
import { addTaskPreview } from "tauri-plugin-txmap-api";
import { AppContext } from "@/components/app-context.tsx";

interface DeliveryCardProps {
  // key: string,
  task: DeliveryTask;
  className?: string;
  viewMode?: number;
  variant: "default" | "mini";
  onAcceptTask: (task: DeliveryTask) => void;
}

export function TaskNewCard({
  task,
  className,
  variant = "default",
  onAcceptTask,
}: DeliveryCardProps) {
  const taskUpdate = useTaskUpdate();

  const appContext = useContext(AppContext);
  const {
    states: {
      taskAddress: [taskAddress, setTaskAddress],
    },
  } = appContext;
  // const onClickAccept = async (task: DeliveryTask) => {
  //   let t = await taskUpdate.mutateAsync({id: task.id, status: "idle"});
  //
  //   onAcceptTask(task);
  //   // if (t.status === "idle") {
  //   //   if (mode === 0) {
  //   //     setMode(1);
  //   //     let map = new Map();
  //   //     map.set(t.id, commandArgs(t))
  //   //     await startTxMapWithChannel({tasks: map}, (res) => {
  //   //       console.log(res)
  //   //     })
  //   //   } else {
  //   //     await addTaskPreview(commandArgs(t))
  //   //
  //   //   }
  //   // }
  // }

  // function commandArgs(task: DeliveryTask): Task {
  //   const args: Task = {
  //     id: task.id,
  //     status: "idle",
  //     start_position: {
  //       lat: task.address_send.latlng_lat,
  //       lng: task.address_send.latlng_lng
  //     },
  //     end_position: {
  //       lat: task.address_receive.latlng_lat,
  //       lng: task.address_receive.latlng_lng
  //     },
  //     send_info: {
  //       avatar: undefined
  //     },
  //     receive_info: {
  //       avatar: undefined
  //     }
  //   }
  //   return args;
  // }

  return (
    <>
      <NormalCard
        hidden={variant !== "default"}
        onClickAccept={onAcceptTask}
        className={className}
        task={task}
      />{" "}
      <MiniCard
        hidden={variant !== "mini"}
        onClickAccept={onAcceptTask}
        className={className}
        task={task}
      />
    </>
  );
}

type CardProps = {
  task: DeliveryTask;
  className?: string;
  onClickAccept: (task: DeliveryTask) => void;
  hidden: boolean;
};

const NormalCard = ({ task, className, onClickAccept, hidden }: CardProps) => {
  return (
    <Card
      hidden={hidden}
      className={cn(
        "w-full rounded-2xl bg-white py-2 shadow-sm border-0",
        className,
      )}
    >
      {/* Header */}
      <CardHeader className="flex h-auto flex-row items-center justify-between px-3 py-2 sm:h-9 sm:py-1">
        <div className="flex-1 text-left text-base font-bold text-black pl-2">
          今天{new Date().getHours()}:{new Date().getMinutes()}前送达
        </div>
        <div className={"flex "}>{`id: ${task.id} status: ${task.status}`}</div>
        <div className="flex-shrink-0 text-right text-base font-bold text-black">
          ￥{task?.estimated_in_come}
        </div>
        <CardMenuDropdown id={task.id} />
      </CardHeader>
      {/* Content */}
      <CardContent className="flex w-full items-center justify-center px-3 py-0">
        <div className="flex flex-1 flex-col items-start gap-2">
          {/* Address Section */}
          <div className="flex w-full items-start gap-3">
            <DistanceIndicator fromDistance={"1"} toDistance={"2"} />

            <div className="flex flex-1 flex-col justify-center items-start gap-3 py-1">
              {/* From Section */}
              <div className="flex w-full flex-shrink-0 flex-col justify-center items-start gap-2">
                <div className="w-full flex-shrink-0  text-lg font-bold text-black leading-tight">
                  {task?.address_send?.poiname}
                </div>
                <div className="w-full flex-shrink-0 text-xs font-normal text-neutral-800 sm:h-[10px] sm:text-sm">
                  {task?.address_receive?.poiaddress}
                </div>
              </div>

              {/* To Section */}
              <div className="w-full flex-shrink-0 text-lg font-bold text-black leading-tight sm:h-6 sm:py-1 sm:text-xl">
                {task?.address_receive?.poiname}
              </div>
            </div>
          </div>

          {/* Details Section */}
          <div className="flex w-full items-start gap-3">
            {/* Empty space for alignment */}
            <div className="flex w-9 items-center gap-2.5 px-0.5 sm:h-[100px]" />

            <div className="flex flex-1 flex-col items-start justify-center gap-2 sm:gap-1.5">
              {/* Tag */}
              {task?.tag && (
                <div className="rounded border border-neutral-800 px-3 py-1 text-xs font-normal text-neutral-800">
                  {task.tag}
                </div>
              )}

              {/* Items */}
              {task?.items && (
                <div className="w-full rounded px-3 py-1 bg-neutral-100 text-sm font-normal text-neutral-700">
                  {task?.items}
                </div>
              )}

              {/* Notes */}
              {task?.notes && (
                <div className="w-full rounded px-3 py-1 bg-orange-50 p-3 text-sm font-normal text-amber-900">
                  {task.notes}
                </div>
              )}
            </div>
          </div>
        </div>
      </CardContent>
      {/* Footer */}
      <CardFooter className="flex rounded-2xl h-16 w-full flex-col items-center justify-center gap-2.5 bg-white px-3 py-1">
        <Button
          onClick={() => onClickAccept(task)}
          className="cursor-pointer h-12 w-full flex-shrink-0 rounded-lg bg-orange-400 text-base font-bold text-white"
        >
          {"接单"}
        </Button>
      </CardFooter>
    </Card>
  );
};

const MiniCard = ({ task, className, onClickAccept, hidden }: CardProps) => {
  // 1. 计算路程

  return (
    <div
      hidden={hidden}
      className={cn(
        className,
        "w-full h-[120px] py-1 bg-white rounded-2xl shadow-[0px_0px_1px_1px_rgba(244,244,244,0.10)] inline-flex flex-col",
      )}
    >
      <div className="flex-1 px-3 py-1 flex justify-between items-center">
        {/* 左侧店铺信息 */}
        <div className="w-full h-full relative flex gap-3">
          {/* 店铺图标和距离 */}
          <DistanceIndicator
            font={"mini"}
            className={"h-full"}
            line={"h-2"}
            fromDistance={100}
            toDistance={200}
          />
          {/* 店铺详情 */}
          <div className="py-1 w-max-3/5 flex flex-col justify-center gap-3">
            <h3 className="text-sm font-semibold tracking-wider">
              蜜雪冰城-德润城店
            </h3>
            <p className="text-sm font-semibold tracking-wider">
              重庆华府酒店(财富广场店)8楼802
            </p>
            <div className="w-fit bg-orange-50 rounded-sm flex items-center px-3">
              <span className="text-yellow-950 text-sm">备注：other-#其他</span>
            </div>
          </div>
        </div>

        {/* 右侧操作面板 */}
        <div className="w-24 h-full flex flex-col items-end justify-center gap-3">
          <div className="w-fit mr-2 transform text-red-700 text-base font-semibold">
            ￥4.8
          </div>
          <button
            onClick={() => onClickAccept(task)}
            className="w-full h-10  top-8 text-white bg-orange-400 rounded-lg flex items-center justify-center text-base font-semibold"
          >
            接单
          </button>
          <div className="w-fit mr-2 transform text-neutral-400 text-xs">
            路程: 3km
          </div>
        </div>
      </div>
    </div>
  );
};
