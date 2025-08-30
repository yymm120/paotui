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
import { useState } from "react";
import { CardMenuDropdown } from "@/components/delivery/card/card-menu-dropdown.tsx";
import { YesIcon } from "@/components/delivery/icons/yes-icon.tsx";
import { isTauri } from "@tauri-apps/api/core";
import { fragment } from "tauri-plugin-txmap-api";

interface DeliveryCardProps {
  task: DeliveryTask;
  className?: string;
}

export function TaskIdleCard({ task, className }: DeliveryCardProps) {
  const [arrived, setArrived] = useState<boolean>(false);
  const taskUpdate = useTaskUpdate();

  const confirmArrived = (id: string) => {
    console.log(id);
    setArrived(true);
  };

  const confirmPickup = (id: string) => {
    taskUpdate.mutate({ id: id, status: "pend" });
  };
  const inTauri = isTauri();

  const handleGPS = async () => {
    if (inTauri) {
      await fragment();
    }
  };

  return (
    <Card
      className={cn(
        "w-full rounded-2xl bg-white py-2 shadow-sm border-0",
        className,
      )}
    >
      {/* Header */}
      <CardHeader className="flex h-auto flex-row items-center justify-between px-3 py-2 relative">
        <div className="flex-1 text-left font-bold text-base text-black pl-2">
          今天{new Date().getHours()}:{new Date().getMinutes()}前送达
        </div>
        {/*<div className={"flex "}>*/}
        {/*  { `id: ${task.id} status: ${task.status}` }*/}
        {/*</div>*/}
        <div className="flex-shrink-0 text-right font-bold text-base text-black">
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
                <div className="w-full flex-shrink-0 flex justify-between text-lg font-bold text-black leading-tight">
                  {task?.address_send?.poiname}
                  <Button
                    onClick={handleGPS}
                    className={"mr-2 py-0 h-8 cursor-pointer w-24"}
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="24"
                      height="24"
                      viewBox="-1.5 -1 24 24"
                    >
                      <path
                        fill="#fff"
                        d="M18.913 2.9L2.632 9.226l4.829 2.006a5.77 5.77 0 0 1 3.118 3.119l2.006 4.828zm1.847.682l-6.328 16.281c-.4 1.03-1.551 1.557-2.571 1.18a1.92 1.92 0 0 1-1.11-1.067l-2.007-4.83a3.85 3.85 0 0 0-2.079-2.078l-4.828-2.006C.833 10.645.375 9.486.814 8.472A2.05 2.05 0 0 1 1.949 7.38L18.23 1.052a1.945 1.945 0 0 1 2.53 2.53"
                      />
                    </svg>
                    导航
                  </Button>
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
      <CardFooter className="flex rounded-2xl h-16 w-full items-center justify-center gap-4 bg-white px-3 py-1">
        {!arrived ? (
          <Button
            onClick={() => confirmArrived(task.id)}
            className="flex-1 cursor-pointer h-12 bg-blue-400 text-base font-bold text-white"
          >
            {"抵 达"}
          </Button>
        ) : (
          <span className={"flex gap-1"}>
            <YesIcon color={"#3bf214"} />
            已到达取货点
          </span>
        )}

        <Button
          disabled={!arrived}
          onClick={() => confirmPickup(task.id)}
          className="flex-1 h-12 cursor-pointer font-bold text-base text-white"
        >
          {"取 货"}
        </Button>
      </CardFooter>
    </Card>
  );
}
