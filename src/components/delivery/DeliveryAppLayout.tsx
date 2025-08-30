import {
  BottomFooter,
  DeliveryAppHeader,
  MainForDeliveryTask,
} from "@/components/delivery";
import { cn } from "@/lib/utils";
// import { DeliveryAppHeader } from "./DeliveryAppHeader";
// import { NavigationTabs } from "./NavigationTabs";
// import { FilterSection } from "./FilterSection";
// import { DeliveryCardList } from "./DeliveryCardList";
// import { BottomFooter } from "./BottomFooter";
import type { DeliveryTask } from "@/types";
import { Button } from "@/components/ui/button.tsx";

// import {mockDeliveryOrders} from "@/data/mockDeliveryOrders.ts";

export interface DeliveryAppLayoutProps {
  className?: string;
  orders?: DeliveryTask[];
  activeTab?: string;
  isWorking?: boolean;
  status?: string;
  filterLabel?: string;
}

export interface DeliveryAppLayoutEvents {
  onMenuClick?: () => void;
  onStatusClick?: () => void;
  onNotificationClick?: () => void;
  onTabChange?: (tab: string) => void;
  onFilterClick?: () => void;
  onAcceptOrder?: (orderId: string) => void;
  onToggleWork?: () => void;
  onSettingsClick?: () => void;
}

// const mockDeliveryOrders: DeliveryOrder[] = [
//   {
//     id: "1",
//     deliveryTime: "30分钟内(19:45前)送达",
//     estimatedEarnings: "4.8",
//     fromStore: "蜜雪冰城-德润城店",
//     fromAddress: "义务大道337号",
//     toAddress: "重庆华府酒店(财富广场店)八楼802",
//     fromDistance: "300",
//     toDistance: "300",
//     tag: "新人体验单",
//     items: "货品：食品小吃·2公斤·2件",
//     notes: "备注：other-#其他",
//     buttonText: "接单",
//     status: "new",
//     priority: "medium",
//     estimatedEarnings: 8.5,
//     orderTime: new Date().toISOString(),
//   },
//   {
//     id: "2",
//     deliveryTime: "30分钟内(19:45前)送达",
//     estimatedEarnings: "4.8",
//     fromStore: "蜜雪冰城-德润城店",
//     fromAddress: "义务大道337号",
//     toAddress: "重庆华府酒店(财富广场店)八楼802",
//     fromDistance: "300",
//     toDistance: "300",
//     tag: "新人体验单",
//     items: "货品：食品小吃·2公斤·2件",
//     notes: "备注：other-#其他",
//     buttonText: "接单",
//     status: "new",
//     priority: "medium",
//     estimatedEarnings: 8.5,
//     orderTime: new Date().toISOString(),
//   },
// ];

import {
  createContext,
  type Dispatch,
  type SetStateAction,
  useContext,
} from "react";
import { AppContext } from "@/components/app-context.tsx";

export const ViewStatusContext = createContext<{
  viewStatus: number;
  setViewStatus: Dispatch<SetStateAction<number>>;
} | null>(null);
export function DeliveryAppLayout({ className }: DeliveryAppLayoutProps) {
  const appContext = useContext(AppContext);
  const {
    states: {
      taskAddress: [taskAddress],
    },
    features: {
      location: {
        location: [currentLocation],
        watchTriggerOfLocation: [_, setWatchLocation],
      },
      map,
    },
  } = appContext;
  const startWatch = async () => {
    setWatchLocation(true);
  };
  const clearWatch = async () => {
    setWatchLocation(false);
  };

  return (
    <div className={cn("flex w-full flex-col", className)}>
      {/* Fixed Header */}
      <DeliveryAppHeader className={"fixed top-0 left-0 h-12"} />
      <div id={"debug"} className={"fixed z-10000 top-1/7 left-0"}>
        <div className={"flex flex-col gap-1"}>
          <Button onClick={startWatch}>监听GPS</Button>
          <Button onClick={clearWatch}>取消监听</Button>
          <Button onClick={clearWatch}>画线</Button>
        </div>
      </div>
      <div id={"debug-info"} className={" fixed z-10000 top-1/8 left-1/5"}>
        <div>
          location: {currentLocation.coords.latitude}
          {currentLocation.coords.longitude}
        </div>
        {/*taskAddress:{JSON.stringify(taskAddress)}*/}
        map:{JSON.stringify(map?.toString())}
      </div>

      {/*<Button>watchPosition</Button>*/}
      <MainForDeliveryTask
        className={"fixed top-12 h-[calc(100vh-var(--spacing)*32)]"}
      />

      <BottomFooter className={"fixed bottom-0 left-0 h-20"} />
    </div>
  );
}
