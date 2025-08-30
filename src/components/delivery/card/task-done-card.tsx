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

interface DeliveryCardProps {
  task: DeliveryTask;
  className?: string;
}

export function TaskDoneCard({ task, className }: DeliveryCardProps) {
  const taskUpdate = useTaskUpdate();

  const onClickForAcceptOrder = (id: string) => {
    taskUpdate.mutate({ id: id, status: "pend" });
  };

  return (
    <Card
      className={cn(
        "w-full rounded-2xl bg-white py-2 shadow-sm border-0",
        className,
      )}
    >
      {/* Header */}
      <CardHeader className="flex h-auto flex-row items-center justify-between px-3 py-2 sm:h-9 sm:py-1">
        <div className="flex-1 text-left text-sm font-bold text-black pl-2">
          今天{new Date().getHours()}:{new Date().getMinutes()}前送达
        </div>
        <div className={"flex "}>{`id: ${task.id} status: ${task.status}`}</div>
        <div className="flex-shrink-0 text-right text-sm font-bold text-black pr-2">
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
          onClick={() => onClickForAcceptOrder(task.id)}
          className="cursor-pointer h-12 w-full flex-shrink-0 rounded-lg bg-orange-400 text-base font-bold text-white"
        >
          {"接单"}
        </Button>
      </CardFooter>
    </Card>
  );
}
