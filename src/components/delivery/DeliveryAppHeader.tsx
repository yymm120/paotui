import { cn } from "@/lib/utils";
import { HamburgerIcon } from "./icons/HamburgerIcon";
import { StatusIcon } from "./icons/StatusIcon";
import { NotificationIcon } from "./icons/NotificationIcon";
import { ChevronDownIcon } from "./icons/ChevronDownIcon";

interface DeliveryAppHeaderProps {
  className?: string;
  onMenuClick?: () => void;
  onStatusClick?: () => void;
  onNotificationClick?: () => void;
  status?: string;
}

export function DeliveryAppHeader({
  className,
  onMenuClick,
  onStatusClick,
  onNotificationClick,
  status = "已收工",
}: DeliveryAppHeaderProps) {
  return (
    <div
      className={cn(
        "fixed top-0 left-0 z-[1000] w-full flex flex-col items-center bg-black",
        className,
      )}
    >
      {/* Menu Bar */}
      <div className="flex h-12 w-full items-center gap-2 bg-black px-3 py-2 sm:h-[46px] sm:px-4">
        {/* Hamburger Menu Button */}
        <button
          onClick={onMenuClick}
          className="flex h-10 w-10 items-center justify-center rounded-2xl border-[0.5px] border-neutral-600 bg-neutral-800 p-1 active:bg-neutral-700 sm:h-[30px] sm:w-[30px]"
        >
          <HamburgerIcon className="h-[17px] w-[17px] flex-shrink-0" />
        </button>

        {/* Status Indicator */}
        <button
          onClick={onStatusClick}
          className="flex h-10 min-w-fit items-center justify-center gap-1 rounded-2xl border-[0.5px] border-neutral-400 px-3 py-2 active:bg-neutral-800 sm:h-[30px] sm:px-2 sm:py-1"
        >
          <StatusIcon className="h-4 w-4 flex-shrink-0" />
          <span className="text-sm font-bold text-white whitespace-nowrap">
            {status}
          </span>
          <ChevronDownIcon className="h-3 w-3 flex-shrink-0" stroke="white" />
        </button>

        {/* Spacer */}
        <div className="flex-1 min-w-0" />

        {/* Notification Button */}
        <button
          onClick={onNotificationClick}
          className="flex h-10 w-10 items-center justify-center rounded-full active:bg-neutral-800 sm:h-[30px] sm:w-[30px]"
        >
          <NotificationIcon className="h-7 w-7 sm:h-[30px] sm:w-[30px]" />
        </button>
      </div>
    </div>
  );
}
