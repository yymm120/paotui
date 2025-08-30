import { useTaskList } from "@/hooks/query/use-task-list.ts";
import { useQueryClient } from "@tanstack/react-query";
import { PullToRefresh } from "@/components/pull-to-refresh.tsx";
import { cn } from "@/lib/utils.ts";
import { TaskNewCard } from "@/components/delivery/card/task-new-card.tsx";
import type { DeliveryTask } from "@/types";
import { type ReactElement, useContext, useEffect } from "react";
import { TaskPendCard } from "@/components/delivery/card/task-pend-card.tsx";
import { TaskDoneCard } from "@/components/delivery/card/task-done-card.tsx";
import { TaskIdleCard } from "@/components/delivery/card/task-idle-card.tsx";
import { AppContext } from "@/components/app-context.tsx";
import { useTaskUpdate } from "@/hooks/query/use-task-update.ts";

import { TMapApi } from "@/components/map/tencent-map.tsx";

export function TaskListContent({
  orderBy,
  contentType,
}: {
  orderBy: string;
  contentType: string;
}) {
  const { isLoading, data, isRefetching } = useTaskList(contentType, orderBy);
  const {
    states: {
      viewMode: [mode],
      taskAddress: [taskAddress, setTaskAddress],
    },
    features: { map: mF },
  } = useContext(AppContext);
  const queryClient = useQueryClient();

  const taskUpdate = useTaskUpdate();

  const onAcceptTask = async (task: DeliveryTask) => {
    let t: DeliveryTask = await taskUpdate.mutateAsync({
      id: task.id,
      status: "idle",
    });
    setTaskAddress({
      ...taskAddress,
      [t.id]: {
        taskId: t.id,
        address_send: t.address_send,
        address_receive: t.address_receive,
      },
    });
  };

  useEffect(() => {
    if (!mF) {
      return;
    }
    if (!taskAddress || !Object.values(taskAddress).length) {
      return;
    }
    const [_, setMarker] = mF.geometries.marker;
    let markers = [];
    let latlngBounds = new TMapApi.LatLngBounds();

    Object.values(taskAddress).forEach((t) => {
      let marker1 = {
        styleId: "multiMarkerStyle1",
        position: new TMapApi.LatLng(
          t.address_send.latlng_lat,
          t.address_send.latlng_lng,
        ),
      };
      let marker2 = {
        styleId: "multiMarkerStyle2",
        position: new TMapApi.LatLng(
          t.address_receive.latlng_lat,
          t.address_receive.latlng_lng,
        ),
      };
      // console.log(t.address_send)
      // console.log(t.address_receive)
      markers.push(marker1);
      markers.push(marker2);

      latlngBounds.extend(marker1.position);
      latlngBounds.extend(marker2.position);
    });
    console.log(JSON.stringify(latlngBounds));
    mF.refs.mapRef.current.fitBounds(latlngBounds, { padding: 60 });
    // console.log("into", JSON.stringify(markers))
    setMarker((prev) => [...markers]);
  }, [mF, taskAddress]);

  const TaskCard = ({ task }: { task: DeliveryTask }): ReactElement => {
    const variant = mode === 0 ? "default" : mode === 1 ? "mini" : "default";

    let card;
    switch (contentType) {
      case "new": {
        card = (
          <TaskNewCard
            task={task}
            variant={variant}
            onAcceptTask={onAcceptTask}
          />
        );
        break;
      }
      case "idle": {
        card = <TaskIdleCard task={task} />;
        break;
      }
      case "pend": {
        card = <TaskPendCard task={task} />;
        break;
      }
      case "done": {
        card = <TaskDoneCard task={task} />;
        break;
      }
      default:
        card = (
          <TaskNewCard
            task={task}
            variant={variant}
            onAcceptTask={onAcceptTask}
          />
        );
    }
    return card!;
  };

  return (
    <main className="w-full h-full flex-1 overscroll-y-auto overflow-scroll scrollbar-hide">
      {isLoading || isRefetching ? (
        <></>
      ) : (
        <PullToRefresh
          className={"h-full"}
          onRefresh={() => {
            return queryClient.invalidateQueries({ queryKey: ["task-list"] });
          }}
        >
          <div
            className={cn(
              "flex w-full flex-col items-start gap-2 bg-neutral-100 px-2 py-2 sm:gap-2.5 sm:px-3",
            )}
          >
            {data?.map((task) => (
              <TaskCard key={task.id} task={task!} />
            ))}
          </div>
        </PullToRefresh>
      )}
    </main>
  );
}
