// import { Button } from "@/components/ui/button"

import { HamburgerIcon } from "@/components/delivery/icons/hamburger-icon.tsx";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { cn } from "@/lib/utils.ts";
import {Button} from "@/components/ui/button.tsx";

import {SidebarHeader} from "@/components/delivery/sidebar/sidebar-header.tsx";
import {SidebarContent} from "@/components/delivery/sidebar/sidebar-content.tsx";

export function MobileSidebar() {
  return (
    <Sheet>
      <SheetTrigger asChild className={"w-8 h-8 bg-zinc-900 rounded-2xl outline-neutral-500 "}>
        <Button>
          <HamburgerIcon />
        </Button>
      </SheetTrigger>
      <SheetContent
        className={cn(
          "z-[1000] w-86 pt-0 pb-14 bg-white inline-flex flex-col justify-start items-start gap-2.5",
        )}
        side={"left"}
      >
        <SheetHeader className={"w-full px-4 pb-2"}>
          <SheetTitle>
            <SidebarHeader />
          </SheetTitle>
          {/*<SheetDescription />*/}
        </SheetHeader>
        <SidebarContent />

      </SheetContent>
    </Sheet>
  );
}
