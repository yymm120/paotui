import { MobileSidebar, NotificationIcon } from "@/components/delivery";
import { WorkingStatusDropdown } from "@/components/delivery/app-header/working-status-dropdown.tsx";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button.tsx";
import { MapNavIcon } from "@/components/delivery/icons/map-nav-icon.tsx";
import { type Dispatch, type SetStateAction, useContext } from "react";
import { ChevronDownIcon } from "lucide-react";
import { AppContext } from "@/components/app-context.tsx";

interface DeliveryAppHeaderProps {
  className?: string;
}

export function DeliveryAppHeader({ className }: DeliveryAppHeaderProps) {
  const {
    states: {
      viewMode: [mode, setMode],
    },
  } = useContext(AppContext);
  const onNotificationClick = () => {};
  const onMapNavigationClick = () => {
    setMode((pre) => (pre === 0 ? 1 : 0));
  };

  return (
    <div
      className={cn(
        "z-[1000] w-full flex flex-col items-center bg-black",
        className,
      )}
    >
      <div className="flex h-12 w-full items-center gap-2 bg-black px-3 py-2 sm:h-[46px] sm:px-4">
        <MobileSidebar />

        <WorkingStatusDropdown className={"cursor-pointer"} />

        {mode === 1 && (
          <span
            className={"ml-1 text-white text-xs flex-2 flex items-center gap-1"}
          >
            <span>新任务</span>
            <ChevronDownIcon size={"14"} />
          </span>
        )}

        <div className="flex-1 min-w-0" />
        <Button
          onClick={onMapNavigationClick}
          size={"default"}
          className={
            "has-[>svg]:px-1 cursor-pointer justify-center p-0 h-6 border-[#4b4b4b] border-1 text-white rounded-3xl text-xs font-bold bg-[#1A1A1A] flex gap-2.5 w-20"
          }
        >
          <MapNavIcon />
          <span>导航</span>
        </Button>

        <NotificationIcon
          className={"w-8 h-8 cursor-pointer"}
          onClick={onNotificationClick}
        />
      </div>
    </div>
  );
}
