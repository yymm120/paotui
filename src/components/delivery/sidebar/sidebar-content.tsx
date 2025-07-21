import {RightIcon} from "@/components/delivery/icons/right-icon.tsx";
import {useState} from "react";
import {Avatar, AvatarFallback, AvatarImage} from "@/components/ui/avatar.tsx";
import {useWorkInfoToday} from "@/hooks/use-income-today.ts";
import {SidebarEyes} from "@/components/delivery/sidebar/sidebar-eyes.tsx";
import {Button} from "@/components/ui/button.tsx";




export function SidebarContent() {

  let {incomeToday, orderCountTody} = useWorkInfoToday();
  let [displayInfo, setDisplayInfo] = useState<boolean>(false);
  let [userName, ] = useState<string>("罗*昊");


  return (
    <div className="self-stretch px-3 pt-4 bg-white inline-flex flex-col justify-start items-center gap-3 overflow-hidden">
          <div className="self-stretch h-12 py-2 inline-flex justify-start items-start gap-2.5 overflow-hidden">
            <div className="self-stretch flex justify-start items-center gap-2 overflow-hidden">

              <Avatar className={"cursor-pointer "} >
                <AvatarImage src="https://placehold.co/32x32" alt="@shadcn" />
                <AvatarFallback>CN</AvatarFallback>
              </Avatar>

              <span className="cursor-pointer h-6 flex items-center gap-1 justify-center text-neutral-700 text-base font-bold font-['Inter']">
                {userName}
                <RightIcon color={"black"} />
              </span>
              <SidebarEyes isOpen={displayInfo} onClick={() => setDisplayInfo(!displayInfo)}/>
            </div>
          </div>

          <div className="self-stretch px-4 py-3 bg-rose-100 rounded-xl shadow-[0px_0px_1px_0px_rgba(0,0,0,0.40)] inline-flex justify-start items-center gap-12 overflow-hidden">
            <div className="w-32 inline-flex flex-col justify-center items-start gap-3">
              <div className="self-stretch h-4 justify-center text-neutral-700 text-sm font-bold font-['Inter']">
                今日收入(元)
              </div>
              <div className="self-stretch justify-center text-red-800 text-xl font-black font-['Inter']">
                {displayInfo? incomeToday : "***"}
              </div>
              <Button variant={"link"} className="cursor-pointer pl-0 self-stretch inline-flex justify-start items-center gap-1 overflow-hidden">
                <div className="w-14 justify-center text-rose-950 text-sm font-medium font-['Inter']">
                  我的钱包
                </div>
                <div data-svg-wrapper className="relative">
                  <RightIcon color={"#46151B"} />
                </div>
              </Button>
            </div>
            <div className="w-32 inline-flex flex-col justify-center items-start gap-3 overflow-hidden">
              <div className="self-stretch h-4 justify-center text-neutral-700 text-sm font-bold font-['Inter']">
                今日完单量(单)
              </div>
              <div className="self-stretch inline-flex justify-start items-center">
                <div className="justify-center text-red-800 text-xl font-black font-['Inter']">
                  {displayInfo? orderCountTody : "***"}
                </div>
                <div className="w-14 justify-center text-red-800 text-sm font-medium font-['Inter']">
                  {/*(含0趟)*/}
                </div>
              </div>
              <Button variant={"link"} className="cursor-pointer pl-0 self-stretch inline-flex justify-start items-center gap-1">
                <div className="w-14 justify-center text-rose-950 text-sm font-medium font-['Inter']">
                  订单统计
                </div>
                <div data-svg-wrapper className="relative">
                  <RightIcon color={"#46151B"} />
                </div>
              </Button>
            </div>
          </div>
          {/*<div className="self-stretch px-3 py-1.5 inline-flex justify-center items-center gap-6 overflow-hidden">*/}
          {/*  <div className="flex-1 px-1 flex justify-center items-center gap-1 overflow-hidden">*/}
          {/*    <div data-svg-wrapper className="relative">*/}
          {/*      <svg*/}
          {/*        width="20"*/}
          {/*        height="20"*/}
          {/*        viewBox="0 0 20 20"*/}
          {/*        fill="none"*/}
          {/*        xmlns="http://www.w3.org/2000/svg"*/}
          {/*      >*/}
          {/*        <path*/}
          {/*          d="M10.279 8.99225L3.56208 15.7417C3.38397 15.9185 3.2426 16.1288 3.14614 16.3604C3.04967 16.5921 3 16.8406 3 17.0915C3 17.3425 3.04967 17.5909 3.14614 17.8226C3.2426 18.0543 3.38397 18.2645 3.56208 18.4413C3.92178 18.7991 4.40849 19 4.91584 19C5.42318 19 5.90989 18.7991 6.26959 18.4413L13.0084 11.7219M16.3809 13.8082L18.7336 11.4553C18.9042 11.2847 19 11.0533 19 10.812C19 10.5707 18.9042 10.3393 18.7336 10.1687L11.832 3.2664C11.6614 3.09582 11.43 3 11.1888 3C10.9476 3 10.7162 3.09582 10.5456 3.2664L8.19288 5.61934C8.02232 5.78997 7.9265 6.02136 7.9265 6.26262C7.9265 6.50389 8.02232 6.73528 8.19288 6.9059L15.0945 13.8082C15.2651 13.9788 15.4965 14.0746 15.7377 14.0746C15.979 14.0746 16.2103 13.9788 16.3809 13.8082Z"*/}
          {/*          stroke="#383838"*/}
          {/*          stroke-width="1.6"*/}
          {/*          stroke-linecap="round"*/}
          {/*          stroke-linejoin="round"*/}
          {/*        />*/}
          {/*      </svg>*/}
          {/*    </div>*/}
          {/*    <div className="w-14 justify-center text-neutral-700 text-sm font-medium font-['Inter']">*/}
          {/*      违规申诉*/}
          {/*    </div>*/}
          {/*  </div>*/}
          {/*  <div className="flex-1 px-11 flex justify-center items-center gap-1 overflow-hidden">*/}
          {/*    <div data-svg-wrapper className="relative">*/}
          {/*      <svg*/}
          {/*        width="20"*/}
          {/*        height="20"*/}
          {/*        viewBox="0 0 20 20"*/}
          {/*        fill="none"*/}
          {/*        xmlns="http://www.w3.org/2000/svg"*/}
          {/*      >*/}
          {/*        <path*/}
          {/*          d="M13.3333 1.66666H6.66663C3.90913 1.66666 1.66663 3.90916 1.66663 6.66666V17.5C1.66663 17.721 1.75442 17.933 1.9107 18.0892C2.06698 18.2455 2.27895 18.3333 2.49996 18.3333H13.3333C16.0908 18.3333 18.3333 16.0908 18.3333 13.3333V6.66666C18.3333 3.90916 16.0908 1.66666 13.3333 1.66666ZM16.6666 13.3333C16.6666 15.1717 15.1716 16.6667 13.3333 16.6667H3.33329V6.66666C3.33329 4.82832 4.82829 3.33332 6.66663 3.33332H13.3333C15.1716 3.33332 16.6666 4.82832 16.6666 6.66666V13.3333Z"*/}
          {/*          fill="#383838"*/}
          {/*        />*/}
          {/*        <path*/}
          {/*          d="M5.83337 7.5H14.1667V9.16667H5.83337V7.5ZM5.83337 10.8333H11.6667V12.5H5.83337V10.8333Z"*/}
          {/*          fill="#383838"*/}
          {/*        />*/}
          {/*      </svg>*/}
          {/*    </div>*/}
          {/*    <div className="w-14 justify-center text-neutral-700 text-sm font-medium font-['Inter']">*/}
          {/*      违规申诉*/}
          {/*    </div>*/}
          {/*  </div>*/}
          {/*</div>*/}
        </div>
  )
}