import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

interface BottomFooterProps {
  className?: string;
  isWorking?: boolean;
  onToggleWork?: () => void;
  onSettingsClick?: () => void;
}

export function BottomFooter({
  className,
  isWorking = false,
  onToggleWork,
  onSettingsClick,
}: BottomFooterProps) {
  return (
    <div
      className={cn(
        "fixed bottom-0 left-0 flex h-20 w-full items-center gap-3 bg-white px-3 border-t border-gray-200 sm:gap-4 sm:px-4",
        className,
      )}
    >
      {/* Settings Button */}
      <button
        onClick={onSettingsClick}
        className="min-h-[44px] px-2 py-2 text-sm font-normal text-black active:bg-gray-100 rounded whitespace-nowrap"
      >
        接单设置
      </button>

      {/* Work Status Button */}
      <Button
        onClick={onToggleWork}
        className={cn(
          "relative flex h-12 flex-1 items-center justify-center rounded-lg text-base font-bold text-white active:scale-95 transition-transform",
          isWorking
            ? "bg-red-600 active:bg-red-700"
            : "bg-orange-600 active:bg-orange-700",
        )}
      >
        {isWorking ? "收工" : "开工"}
      </Button>
    </div>
  );
}
