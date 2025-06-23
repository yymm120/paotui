import * as React from "react";
import { cn } from "@/lib/utils";
import { DeliveryAppHeader } from "./DeliveryAppHeader";
import { NavigationTabs } from "./NavigationTabs";
import { FilterSection } from "./FilterSection";
import { DeliveryCardList } from "./DeliveryCardList";
import { BottomFooter } from "./BottomFooter";
import { type DeliveryOrder } from "../../types/delivery";

interface DeliveryAppLayoutProps {
  className?: string;
  orders?: DeliveryOrder[];
  activeTab?: string;
  isWorking?: boolean;
  status?: string;
  filterLabel?: string;
  onMenuClick?: () => void;
  onStatusClick?: () => void;
  onNotificationClick?: () => void;
  onTabChange?: (tab: string) => void;
  onFilterClick?: () => void;
  onAcceptOrder?: (orderId: string) => void;
  onToggleWork?: () => void;
  onSettingsClick?: () => void;
}

const mockOrders: DeliveryOrder[] = [
  {
    id: "1",
    deliveryTime: "30分钟内(19:45前)送达",
    rating: "4.8",
    fromStore: "蜜雪冰城-德润城店",
    fromAddress: "义务大道337号",
    toAddress: "重庆华府酒店(财富广场店)八楼802",
    fromDistance: "300",
    toDistance: "300",
    tag: "新人体验单",
    items: "货品：食品小吃·2公斤·2件",
    notes: "备注：other-#其他",
    buttonText: "接单",
  },
  {
    id: "2",
    deliveryTime: "30分钟内(19:45前)送达",
    rating: "4.8",
    fromStore: "蜜雪冰城-德润城店",
    fromAddress: "义务大道337号",
    toAddress: "重庆华府酒店(财富广场店)八楼802",
    fromDistance: "300",
    toDistance: "300",
    tag: "新人体验单",
    items: "货品：食品小吃·2公斤·2件",
    notes: "备注：other-#其他",
    buttonText: "接单",
  },
];

export function DeliveryAppLayout({
  className,
  orders = mockOrders,
  activeTab = "new-tasks",
  isWorking = false,
  status = "已收工",
  filterLabel = "综合排序",
  onMenuClick,
  onStatusClick,
  onNotificationClick,
  onTabChange,
  onFilterClick,
  onAcceptOrder,
  onToggleWork,
  onSettingsClick,
}: DeliveryAppLayoutProps) {
  return (
    <div className={cn("flex h-screen w-full flex-col", className)}>
      {/* Fixed Header */}
      <DeliveryAppHeader
        onMenuClick={onMenuClick}
        onStatusClick={onStatusClick}
        onNotificationClick={onNotificationClick}
        status={status}
      />

      {/* Navigation Tabs - Part of Header */}
      <div className="fixed top-12 left-0 z-[999] w-full sm:top-[46px]">
        <NavigationTabs activeTab={activeTab} onTabChange={onTabChange} />
      </div>

      {/* Filter Section - Part of Header */}
      <div className="fixed top-24 left-0 z-[999] w-full sm:top-[94px]">
        <FilterSection
          filterLabel={filterLabel}
          onFilterClick={onFilterClick}
        />
      </div>

      {/* Main Content Area */}
      <main className="mt-36 mb-20 min-h-[calc(100vh-224px)] w-full sm:mt-[130px] sm:min-h-[calc(100vh-210px)]">
        <DeliveryCardList orders={orders} onAcceptOrder={onAcceptOrder} />
      </main>

      {/* Fixed Bottom Footer */}
      <BottomFooter
        isWorking={isWorking}
        onToggleWork={onToggleWork}
        onSettingsClick={onSettingsClick}
      />
    </div>
  );
}
