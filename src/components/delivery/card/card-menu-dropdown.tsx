import { Button } from "@/components/ui/button.tsx";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu.tsx";
import { cn } from "@/lib/utils.ts";
import { ChefHatIcon } from "lucide-react";
import { useTaskUpdate } from "@/hooks/query/use-task-update.ts";

const menuOptions = ["联系卖家", "联系买家", "违约取消"];

export function CardMenuDropdown({
  className,
  id,
}: {
  className?: string;
  id: string;
}) {
  const taskUpdate = useTaskUpdate();

  const handleMenuOption = (option: string) => {
    switch (option) {
      case "联系卖家": {
        console.log("联系卖家");
        break;
      }
      case "联系买家": {
        console.log("联系买家");
        break;
      }
      case "违约取消": {
        console.log(id);
        taskUpdate.mutate({
          id,
          status: "new",
        });
        console.log("违约取消");
        break;
      }
    }
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant={"link"}
          className={cn(className, "absolute, w-0 py-0 h-0")}
        >
          {/*<span className="text-sm font-bold whitespace-nowrap">*/}
          {/* ...*/}
          {/*</span>*/}
          <span className={"border-1 rounded-xl border-gray-200"}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
            >
              <path
                fill="F90705"
                d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0-4 0m0-6a2 2 0 1 0 4 0a2 2 0 0 0-4 0m0 12a2 2 0 1 0 4 0a2 2 0 0 0-4 0"
              />
            </svg>
            {/*<svg className={"fill-gray-700"} xmlns="http://www.w3.org/2000/svg" width="4em" height="4em" viewBox="0 0 24 24"><path fill="F90705" d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0-4 0m0-6a2 2 0 1 0 4 0a2 2 0 0 0-4 0m0 12a2 2 0 1 0 4 0a2 2 0 0 0-4 0"/></svg>*/}
          </span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align={"end"} className="w-3 z-1000 mt-2 mr-6">
        {menuOptions.map((op) => (
          <DropdownMenuItem key={op} onClick={() => handleMenuOption(op)}>
            <ChefHatIcon />
            {op}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
