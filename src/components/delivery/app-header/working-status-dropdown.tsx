import { useState } from "react";
import { ChevronDownIcon, StatusIcon } from "@/components/delivery";
import { Button } from "@/components/ui/button.tsx";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu.tsx";
import { cn } from "@/lib/utils.ts";
// import {type Profile, ProfileContext} from "@/components/profile-context-provider.tsx";

const workingStatus = ["工作", "休息"];

export function WorkingStatusDropdown({ className }: { className: string }) {
  const [position, setPosition] = useState("休息");
  // let { userWorkingStatus } = useContext<Profile>(ProfileContext);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant={"link"}
          className={cn(
            className,
            "flex h-[30px] min-w-fit items-center justify-center gap-1 rounded-full border-[0.5px] border-neutral-400 px-2 py-0 hover:bg-neutral-800 sm:h-[30px] sm:px-2 sm:py-1",
          )}
        >
          <StatusIcon className="h-4 w-4 flex-shrink-0" />
          <span className="text-sm font-bold text-white whitespace-nowrap">
            {position}
          </span>
          <ChevronDownIcon className="h-3 w-3 flex-shrink-0" stroke="white" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align={"start"} className="w-3 z-1000">
        <DropdownMenuRadioGroup value={position} onValueChange={setPosition}>
          {workingStatus.map(
            (status) =>
              status !== position && (
                <DropdownMenuRadioItem key={status} value={status}>
                  {status}
                </DropdownMenuRadioItem>
              ),
          )}
        </DropdownMenuRadioGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
