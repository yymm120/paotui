import {SidebarSwitchIcon} from "@/components/delivery/icons/sidebar-switch-icon.tsx";
import {SettingIcon} from "@/components/delivery/icons/setting-icon.tsx";

export function SidebarHeader () {
  return (
    <div className={"flex justify-between w-full"}>
      <div className="flex gap-1 justify-center items-center text-neutral-700 text-xl font-semibold font-['Inter']">
        <span>校内配送</span>
        <SidebarSwitchIcon />
      </div>
      <SettingIcon />
    </div>
  )
}