import { Button } from "@/components/ui/button";
import { ArrowUp, Settings } from "lucide-react";

export const FooterSection = () => {
    return (
        <footer className="flex w-full items-center justify-between px-5 py-4 bg-transparent">
            <div className="flex flex-col items-center justify-center gap-1 w-12 h-12">
                <Settings className="w-6 h-6 text-black" />
                <div className="text-xs text-black text-center [font-family:'Inter-Light',Helvetica] font-light tracking-[0] leading-[normal]">
                    接单设置
                </div>
            </div>

            <Button className="flex items-center gap-2 px-[120px] py-2.5 h-[46px] bg-[#fe6603] hover:bg-[#e55a02] rounded-lg">
                <ArrowUp className="w-6 h-6 text-white" />
                <span className="text-base text-white [font-family:'Inter-Light',Helvetica] font-light tracking-[0] leading-[normal]">
          开工
        </span>
            </Button>
        </footer>
    );
};