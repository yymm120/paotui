import { cn } from "@/lib/utils";
import { ChevronDownIcon } from "./icons/ChevronDownIcon";

interface NavigationTabsProps {
  className?: string;
  activeTab?: string;
  onTabChange?: (tab: string) => void;
}

const tabs = [
  { id: "new-tasks", label: "新任务" },
  { id: "pickup", label: "待取货" },
  { id: "delivery", label: "配送中" },
];

export function NavigationTabs({
  className,
  activeTab = "new-tasks",
  onTabChange,
}: NavigationTabsProps) {
  return (
    <div
      className={cn(
        "flex h-12 w-full items-center bg-black sm:h-12",
        className,
      )}
    >
      {tabs.map((tab) => (
          <button key={tab.id}
            onClick={() => onTabChange?.(tab.id)}
            className="flex min-w-24 min-h-12 items-center justify-center gap-1 px-2 py-2 active:bg-neutral-800 rounded sm:min-h-fit sm:px-0.5 sm:py-1"
          >
            <span
              className={cn(
                "text-center text-sm font-bold whitespace-nowrap sm:text-base",
                activeTab === tab.id ? "text-white" : "text-neutral-400",
              )}
            >
              {tab.label}
            </span>
            <ChevronDownIcon
              className="h-3 w-3 flex-shrink-0"
              stroke={activeTab === tab.id ? "white" : "#A7A7A7"}
            />
          </button>
      ))}
    </div>
  );
}
