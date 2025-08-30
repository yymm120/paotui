import { DeliveryAppHeader } from "@/components/delivery";
import { cn } from "@/lib/utils.ts";

export interface LayoutProps {
  className?: string;
  children: React.ReactNode;
}

export function Layout({ className, children }: LayoutProps) {
  return (
    <div className={cn("flex w-full flex-col", className)}>
      {/* Fixed Header */}
      <DeliveryAppHeader
        // onMenuClick={() => {}}
        // onStatusClick={() => {}}
        // onNotificationClick={() => {}}
        status={"已收工"}
      />

      {/* Navigation Tabs - Part of Header */}
      <div className="fixed top-12 left-0 z-[999] w-full sm:top-[46px]">
        {/*<NavigationTabs activeTab={activeTab} onTabChange={onTabChange} />*/}
      </div>

      {/* Filter Section - Part of Header */}
      <div className="fixed top-24 left-0 z-[999] w-full sm:top-[94px]">
        {/*<FilterSection*/}
        {/*  filterLabel={filterLabel}*/}
        {/*  onFilterClick={onFilterClick}*/}
        {/*/>*/}
      </div>
      {/* Main Content Area */}
      <main className="mt-36 mb-20 min-h-[calc(100vh-224px)] w-full sm:mt-[130px] sm:min-h-[calc(100vh-210px)]">
        {children}
        {/*<DeliveryCardList orders={orders} onAcceptOrder={onAcceptOrder} />*/}
      </main>
    </div>
  );
}
