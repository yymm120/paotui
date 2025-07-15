import { cn } from "@/lib/utils";
import { ChevronDownIcon } from "./icons/ChevronDownIcon";

interface FilterSectionProps {
  className?: string;
  filterLabel?: string;
  onFilterClick?: () => void;
}

export function FilterSection({
  className,
  filterLabel = "综合排序",
  onFilterClick,
}: FilterSectionProps) {
  return (
    <div
      className={cn(
        "flex w-full items-center gap-2.5 bg-white px-3 sm:px-4",
        className,
      )}
    >
      <button
        onClick={onFilterClick}
        className="flex min-h-[44px] items-center gap-4 px-3 py-2 rounded active:bg-gray-100 sm:h-9 sm:gap-7 sm:px-4 sm:pr-1"
      >
        <span className="text-center text-sm font-bold text-neutral-700 sm:text-base">
          {filterLabel}
        </span>
        <ChevronDownIcon className="h-3 w-3 flex-shrink-0" stroke="#303030" />
      </button>
    </div>
  );
}
