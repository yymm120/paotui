import { cn } from "@/lib/utils";

interface DistanceIndicatorProps {
  className?: string;
  fromDistance: string;
  toDistance: string;
}

export function DistanceIndicator({
  className,
  fromDistance,
  toDistance,
}: DistanceIndicatorProps) {
  return (
    <div
      className={cn("flex w-8 items-center gap-2.5 px-0.5 sm:w-9", className)}
    >
      <div className="flex min-h-[80px] flex-1 flex-col items-center justify-center gap-1 rounded-xl bg-neutral-100 px-1 py-2 sm:h-[100px] sm:rounded-2xl">
        {/* From Distance */}
        <div className="flex h-5 w-full flex-shrink-0 flex-col items-center sm:h-6">
          <div className="w-full text-center text-[10px] font-bold text-black sm:text-xs">
            {fromDistance}
          </div>
          <div className="w-full text-center text-[7px] font-normal text-neutral-500 sm:text-[8px]">
            km
          </div>
        </div>

        {/* Divider Line */}
        <div className="h-4 w-0.5 flex-shrink-0 bg-neutral-400 sm:h-6" />

        {/* To Distance */}
        <div className="flex h-5 w-full flex-shrink-0 flex-col items-center sm:h-6">
          <div className="w-full text-center text-[10px] font-bold text-black sm:text-xs">
            {toDistance}
          </div>
          <div className="w-full text-center text-[7px] font-normal text-neutral-500 sm:text-[8px]">
            km
          </div>
        </div>
      </div>
    </div>
  );
}
