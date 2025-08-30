import { type ReactElement, useContext, useEffect, useState } from "react";
import { FilterSection } from "@/components/delivery/FilterSection.tsx";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs.tsx";
import { cn } from "@/lib/utils";
import { ChevronDownIcon } from "./icons/chevron-down-icon.tsx";
import { useQueryClient } from "@tanstack/react-query";
import { TaskListContent } from "@/components/delivery/app-content/task-list-content.tsx";
import { AppContext, ViewMode } from "@/components/app-context.tsx";
import { Button } from "@/components/ui/button.tsx";
import {
  TencentMap,
  TencentMapContainer,
  TencentMapTrigger,
} from "@/components/map/tencent-map.tsx";
import { isTauri } from "@tauri-apps/api/core";

interface NavigationTabsProps {
  className?: string;
  activeTab?: string;
  onTabChange?: (tab: string) => void;
  children?: ReactElement;
}

const filterLabels = ["综合排序", "距离优先", "收益优先"];
export function MainForDeliveryTask({ className }: NavigationTabsProps) {
  const [tab, setTab] = useState<string>("new");
  const [filter, setFilter] = useState("综合排序");
  let {
    states: {
      viewMode: [mode, setMode],
      map: [_, setMap],
    },
    features: {
      location: {
        location: [currentLocation, _2, locationRef],
      },
      map: mF,
    },
  } = useContext(AppContext);

  if (isTauri()) {
    // if (!channels.current["location"]) {
    // console.log("Watching location...")
    // watchPosition(
    //     {
    //         enableHighAccuracy: true,
    //         timeout: 30000,
    //         maximumAge: 30000,
    //     },
    //     (location) => {
    //         // console.log("Watching location, Location:", location)
    //         currentLocation.current = {
    //             lat: location?.coords.latitude!,
    //             lng: location?.coords.longitude!,
    //         }
    //     }
    // ).then((id) => {
    //     channels.current["location"] = id
    // })
    // }
  }
  const queryClient = useQueryClient();

  useEffect(() => {
    queryClient.invalidateQueries({ queryKey: ["task-list"] });
  }, [filter]);

  const onMapInitialed = (feature: unknown) => {
    setMap(feature);
    console.log(mF);
  };
  return (
    <Tabs defaultValue={tab} className={cn(className, "w-full")}>
      <div
        hidden={mode === 0}
        className={
          "p-0 fixed h-[40%] w-full z-[1] bg-black rounded-none justify-start"
        }
      ></div>
      <TencentMapContainer
        display={mode === 1}
        className={cn(
          "fixed w-full top-12 z-2",
          mode === 1 ? "h-[40%]" : "h-0",
        )}
      >
        <TencentMapTrigger
          hidden={mode !== 1}
          className={"z-2 absolute text-white"}
          onClick={() => setMode((pre) => (pre === 0 ? 1 : 0))}
        >
          唤起/关闭
        </TencentMapTrigger>
        <TencentMap
          className={"z-[1]"}
          mapOptions={{
            center: {
              lat: locationRef.current?.coords?.latitude ?? 30.015,
              lng: locationRef.current?.coords?.longitude ?? 106.23,
            },
            zoom: 16,
            viewMode: "3D",
            // pitch: 80
          }}
          onMapInitialed={onMapInitialed}
        />
      </TencentMapContainer>
      <TabsList
        hidden={mode !== 0}
        className={
          "p-0 fixed h-12 w-full z-[1] bg-black rounded-none justify-start"
        }
      >
        <TabsTrigger
          value={"new"}
          className={cn(
            "min-w-24 min-h-12 gap-1 px-2  rounded-none  ",
            "data-[state=active]:bg-neutral-800 data-[state=active]:text-white cursor-pointer",
            "text-neutral-400   text-center text-sm font-bold whitespace-nowrap ",
          )}
          onClick={() => setTab("new")}
        >
          新任务
          <ChevronDownIcon
            className="h-3 w-3 flex-shrink-0 "
            stroke={tab === "new" ? "white" : "#A7A7A7"}
          />
        </TabsTrigger>
        <TabsTrigger
          value={"idle"}
          className={cn(
            "min-w-24 min-h-12 gap-1 px-2  rounded-none",
            "data-[state=active]:bg-neutral-800 data-[state=active]:text-white cursor-pointer",
            "text-neutral-400   text-center text-sm font-bold whitespace-nowrap ",
          )}
          onClick={() => setTab("idle")}
        >
          待取货
          <ChevronDownIcon
            className="h-3 w-3 flex-shrink-0 "
            stroke={tab === "idle" ? "white" : "#A7A7A7"}
          />
        </TabsTrigger>
        <TabsTrigger
          value="pend"
          className={cn(
            "min-w-24 min-h-12 gap-1 px-2  rounded-none  ",
            "data-[state=active]:bg-neutral-800 data-[state=active]:text-white cursor-pointer",
            "text-neutral-400   text-center text-sm font-bold whitespace-nowrap ",
          )}
          onClick={() => setTab("pend")}
        >
          配送中
          <ChevronDownIcon
            className="h-2 w-2 flex-shrink-0 "
            stroke={tab === "pend" ? "white" : "#A7A7A7"}
          />
        </TabsTrigger>
        <TabsTrigger
          value="done"
          className={cn(
            "min-w-24 min-h-12 gap-1 px-2  rounded-none  ",
            "data-[state=active]:bg-neutral-800 data-[state=active]:text-white cursor-pointer",
            "text-neutral-400   text-center text-sm font-bold whitespace-nowrap ",
          )}
          onClick={() => setTab("done")}
        >
          已完成
          <ChevronDownIcon
            className="h-2 w-2 flex-shrink-0 "
            stroke={tab === "done" ? "white" : "#A7A7A7"}
          />
        </TabsTrigger>
      </TabsList>

      <FilterSection
        filterLabels={filterLabels}
        filter={filter}
        setFilter={setFilter}
        className={cn(
          "fixed h-10 left-0 z-[1] ",
          mode === 0 ? "top-24" : "top-[calc(40%+var(--spacing)*12)]",
        )}
      />

      {/*{children}*/}
      <TabsContent
        value="new"
        className={cn(
          "w-full fixed top-34 bottom-20",
          mode === 0 ? "top-34" : "top-[calc(40%+var(--spacing)*22)]",
        )}
      >
        <TaskListContent orderBy={filter} contentType={"new"} />
      </TabsContent>
      <TabsContent
        value="idle"
        className={cn(
          "w-full fixed top-34 bottom-20",
          mode === 0 ? "top-34" : "top-[calc(40%+var(--spacing)*22)]",
        )}
      >
        <TaskListContent orderBy={filter} contentType={"idle"} />
      </TabsContent>
      <TabsContent
        value="pend"
        className={cn(
          "w-full fixed top-34 bottom-20",
          mode === 0 ? "top-34" : "top-[calc(40%+var(--spacing)*22)]",
        )}
      >
        <TaskListContent orderBy={filter} contentType={"pend"} />
      </TabsContent>
      <TabsContent
        value="done"
        className={cn(
          "w-full fixed top-34 bottom-20",
          mode === 0 ? "top-34" : "top-[calc(40%+var(--spacing)*22)]",
        )}
      >
        <TaskListContent orderBy={filter} contentType={"done"} />
      </TabsContent>
    </Tabs>
  );
}
