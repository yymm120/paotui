import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Bell, ChevronDown, MapPin, Menu } from "lucide-react";

const HeaderSection = () => {
    const navigationTabs = [
        { id: "new-tasks", label: "新任务", active: true },
        { id: "pickup", label: "待取货", active: false },
        { id: "delivery", label: "配送中", active: false },
    ];

    return (
        <header className="flex flex-col w-full items-center relative bg-transparent">
            <div className="flex h-[46px] items-center gap-2 px-4 py-2 relative w-full bg-black">
                <Button
                    variant="outline"
                    size="sm"
                    className="w-[30px] h-[30px] p-0 bg-[#1a1a1a] rounded-2xl border-[0.5px] border-[#727272] hover:bg-[#1a1a1a]"
                >
                    <Menu className="w-[17px] h-[21px] text-white" />
                </Button>

                <Badge
                    variant="outline"
                    className="inline-flex h-[30px] items-center justify-center gap-1 px-2 py-1 rounded-[15px] border-[0.5px] border-[#ababab] bg-transparent hover:bg-transparent"
                >
                    <MapPin className="w-4 h-4 text-white" />
                    <span className="[font-family:'Inter-SemiBold',Helvetica] font-semibold text-white text-sm whitespace-nowrap">
            已收工
          </span>
                    <ChevronDown className="w-3 h-3 text-white" />
                </Badge>

                <div className="flex-1" />

                <Button
                    variant="ghost"
                    size="sm"
                    className="w-[30px] h-[30px] p-0 hover:bg-transparent"
                >
                    <Bell className="w-[30px] h-[30px] text-white" />
                </Button>
            </div>

            <div className="flex h-12 items-center relative w-full bg-black">
                {navigationTabs.map((tab) => (
                    <div
                        key={tab.id}
                        className="flex flex-col w-24 items-start justify-center gap-2.5 px-4 py-2.5 relative"
                    >
                        <Button
                            variant="ghost"
                            className="inline-flex items-center justify-center px-0.5 py-1 h-auto hover:bg-transparent"
                        >
              <span
                  className={`[font-family:'Inter-SemiBold',Helvetica] font-semibold text-base whitespace-nowrap ${
                      tab.active ? "text-white" : "text-[#a7a7a7]"
                  }`}
              >
                {tab.label}
              </span>
                            <ChevronDown className="w-3 h-3 ml-1 text-current" />
                        </Button>
                    </div>
                ))}
            </div>

            <div className="flex items-center gap-2.5 px-4 py-0 relative flex-1 w-full bg-white">
                <Select>
                    <SelectTrigger className="inline-flex items-center gap-7 pl-4 pr-1 py-0 border-none shadow-none bg-transparent hover:bg-transparent focus:ring-0">
                        <SelectValue>
              <span className="[font-family:'Inter-SemiBold',Helvetica] font-semibold text-[#353535] text-base whitespace-nowrap">
                综合排序
              </span>
                        </SelectValue>
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="comprehensive">综合排序</SelectItem>
                        <SelectItem value="time">时间排序</SelectItem>
                        <SelectItem value="distance">距离排序</SelectItem>
                    </SelectContent>
                </Select>
            </div>
        </header>
    );
};

export default HeaderSection;
