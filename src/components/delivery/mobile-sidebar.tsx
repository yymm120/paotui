import { HamburgerIcon } from "@/components/delivery/icons/hamburger-icon.tsx";
import { SidebarContent } from "@/components/delivery/sidebar/sidebar-content.tsx";
import { SidebarHeader } from "@/components/delivery/sidebar/sidebar-header.tsx";
import { Button } from "@/components/ui/button.tsx";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { cn } from "@/lib/utils.ts";
import { type ReactNode, useState } from "react";
import { ArrowLeft, ChevronRight, Power, Space } from "lucide-react";

export function MobileSidebar() {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [settingOpen, setSettingOpen] = useState(false);

  const handleSettingIconClick = () => {
    setSidebarOpen(false);
    setSettingOpen(true);
  };

  return (
    <>
      <Sheet open={sidebarOpen} onOpenChange={setSidebarOpen}>
        <SheetTrigger
          asChild
          className={"w-8 h-8 bg-zinc-900 rounded-2xl outline-neutral-500 "}
        >
          <Button>
            <HamburgerIcon />
          </Button>
        </SheetTrigger>
        <SheetContent
          aria-description={undefined}
          className={cn(
            "z-[1000] w-86 pt-0 pb-14 bg-white inline-flex flex-col justify-start items-start gap-2.5",
          )}
          side={"left"}
        >
          <SheetHeader className={"w-full px-4 pb-2"}>
            <SheetTitle>
              <SidebarHeader onSettingIconClick={handleSettingIconClick} />
            </SheetTitle>
            {/*<SheetDescription />*/}
          </SheetHeader>
          <SidebarContent />
        </SheetContent>
      </Sheet>

      <Sheet open={settingOpen} onOpenChange={setSettingOpen}>
        {/*<SheetTrigger asChild>*/}
        {/*<Button variant="ghost" size="icon">*/}
        {/*  <SettingsIcon className="h-6 w-6" />*/}
        {/*</Button>*/}
        {/*</SheetTrigger>*/}
        <SheetContent
          side="right"
          className="w-full flex-col gap-2.5 p-0 z-[1000] bg-[#F2F2F2]"
        >
          <SheetHeader className="border-b p-4 bg-white">
            <SheetTitle className="flex items-center justify-between gap-2">
              <ArrowLeft
                className="h-5 w-5"
                onClick={() => setSettingOpen(false)}
              />
              <span>设置</span>
              <span className={"w-5"}></span>
            </SheetTitle>
          </SheetHeader>

          <div className="w-full flex flex-col gap-3.5 overscroll-y-auto overflow-scroll scrollbar-hide">
            {/*<div className="flex items-center justify-between rounded-lg px-4 hover:bg-muted">*/}
            {/*  <span>开启悬浮窗</span>*/}
            {/*  /!*<Switch />*!/*/}
            {/*</div>*/}
            <Section title="帮助与反馈">
              <Item label="开启悬浮窗" />
            </Section>

            <Section title="帮助与反馈">
              <Item label="意见反馈" />
              <Item label="投诉举报" />
            </Section>

            <Section title="隐私">
              <Item label="隐私政策简要版" />
              <Item label="个人信息收集清单" />
              <Item label="第三方共享个人信息清单" />
              <Item label="应用权限说明" />
              <Item label="隐私设置" />
            </Section>

            <Section title="关于">
              <Item label="故障检测" />
              <Item label="检查更新" />
              <Item label="关于App" />
              <Item label="公众号" />
            </Section>

            <div className="rounded-lg  bg-white hover:bg-gray-50">
              <Item label="账号注销" description="注销后不可恢复，谨慎操作！" />
            </div>

            <div className="rounded-lg bg-white hover:bg-gray-50">
              <Button
                variant={"ghost"}
                className="flex h-12 w-full items-center hover:bg-gray-50 p-4 text-destructive"
              >
                <Power className="mr-2 h-5 w-5" />
                <span>退出</span>
              </Button>
            </div>
            <div className={"mb-72"}></div>
          </div>
        </SheetContent>
      </Sheet>
    </>
  );
}

function Section({ children }: { title?: string; children: ReactNode }) {
  return (
    <div className="w-full bg-white ">
      <div className="">{children}</div>
    </div>
  );
}

function Item({ label, description }: { label: string; description?: string }) {
  return (
    <div className="flex items-center justify-between p-4 w-full hover:bg-gray-50">
      <div>
        <p>{label}</p>
        {description && (
          <p className="text-sm text-muted-foreground">{description}</p>
        )}
      </div>
      <ChevronRight className="h-5 w-5 text-muted-foreground" />
    </div>
  );
}
