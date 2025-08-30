import { cn } from "@/lib/utils.ts";

interface DistanceIndicatorProps {
  line?: "h-3" | "h-2" | "h-1";
  font?: "normal" | "mini";
  className?: string;
  fromDistance: string | number;
  toDistance: string | number;
}

export function DistanceIndicator({
  line = "h-3",
  font = "normal",
  className,
  fromDistance,
  toDistance,
}: DistanceIndicatorProps) {
  return (
    <div className={cn("flex w-10 items-center gap-2.5 px-0.5", className)}>
      <div className="flex flex-1 flex-col items-center justify-center gap-1.5 rounded-xl bg-neutral-100 px-1 py-2 sm:h-[100px] sm:rounded-2xl">
        {/* From Distance */}
        <div className="flex w-full flex-shrink-0 flex-col items-center">
          <span
            className={cn(
              font === "normal" ? "text-sm/4" : "text-xs/3",
              "w-full text-center text-sm/4 font-bold text-black",
            )}
          >
            {fromDistance}
            {/*900*/}
          </span>
          <span className="w-full text-center text-xs/3 font-normal text-neutral-500">
            km
          </span>
        </div>

        {/* Divider Line */}
        <div className={cn(line, "w-0.5 flex-shrink-0 bg-neutral-400")} />

        {/* To Distance */}
        <div className="flex w-full flex-shrink-0 flex-col items-center">
          <div className="w-full text-center text-sm/4 font-semibold text-black">
            {toDistance}
          </div>
          <div className="w-full text-center text-xs/3 font-normal text-neutral-500">
            km
          </div>
        </div>
      </div>
    </div>
  );
}
