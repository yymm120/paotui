import { cn } from "@/lib/utils";
import { ChevronDownIcon } from "./icons/chevron-down-icon.tsx";
import { Button } from "@/components/ui/button.tsx";
import { type Dispatch, type SetStateAction } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu.tsx";

interface FilterSectionProps {
  className?: string;
  filterLabels: string[];
  filter: string;
  setFilter: Dispatch<SetStateAction<string>>;
}

export function FilterSection({
  className,
  filterLabels,
  filter,
  setFilter,
}: FilterSectionProps) {
  return (
    <div className={cn("w-full", className)}>
      <div
        className={cn("flex w-full items-center gap-2.5 bg-white px-3 h-full")}
      >
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant={"link"}
              type={"button"}
              className={cn(
                "cursor-pointer flex py-1 h-full items-center gap-4 px-3 rounded focus-visible:border-none",
              )}
            >
              <span className="text-center text-sm font-bold text-neutral-700">
                {filter}
              </span>
              <ChevronDownIcon
                className="h-2 w-2 "
                strokeWidth={"1.2"}
                stroke="#303030"
              />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align={"start"} className="w-3 z-1000">
            <DropdownMenuRadioGroup value={filter} onValueChange={setFilter}>
              {filterLabels.map((f) => (
                <DropdownMenuRadioItem key={f} value={f}>
                  {f}
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
}
