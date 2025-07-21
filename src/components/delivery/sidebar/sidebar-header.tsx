import { useState } from "react";
import { SettingIcon } from "@/components/delivery/icons/setting-icon.tsx";
import { SidebarSwitchIcon } from "@/components/delivery/icons/sidebar-switch-icon.tsx";

export function SidebarHeader() {
  const [method] = useState<string>("校内配送");
  return (
    <div className={"flex justify-between w-full"}>
      <div className="flex gap-1 justify-center items-center text-neutral-700 text-xl font-semibold font-['Inter']">
        <span>{method}</span>
        <SidebarSwitchIcon />
      </div>
      <SettingIcon />
    </div>
  );
}
