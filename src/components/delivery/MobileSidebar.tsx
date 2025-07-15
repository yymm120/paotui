// import { Button } from "@/components/ui/button"

import {
    Sheet,
    // SheetClose,
    SheetContent,
    SheetDescription,
    // SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger,
} from "@/components/ui/sheet"
import {HamburgerIcon} from "@/components/delivery/icons/HamburgerIcon.tsx";
import {cn} from "@/lib/utils.ts";
import {Button} from "@/components/ui/button.tsx";

export function MobileSidebar() {
    return (
        <Sheet>
            <SheetTrigger asChild>
                  <button><HamburgerIcon className="h-[17px] w-[17px] flex-shrink-0" /></button>
                {/*<Button variant="outline">Open</Button>*/}
            </SheetTrigger>
            <SheetContent className={cn("z-[1000] w-86 px-px pt-9 pb-14 bg-white inline-flex flex-col justify-start items-start gap-2.5 overflow-hidden")} side={"left"}>
                <div className="self-stretch px-3 pt-4 bg-white inline-flex flex-col justify-start items-center gap-3 overflow-hidden">
                    <div className="self-stretch h-10 bg-white inline-flex justify-between items-center overflow-hidden">
                        <SheetHeader>
                            <SheetTitle className={"size- py-[3px] flex justify-start items-start gap-2.5 overflow-hidden"}>
                                <span className="justify-center text-neutral-700 text-xl font-semibold font-['Inter']">校内配送</span>
                            </SheetTitle>
                            <SheetDescription />
                        </SheetHeader>
                        <button>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd" clip-rule="evenodd" d="M11.199 2.587C11.4441 2.45093 11.7197 2.37953 12 2.37953C12.2803 2.37953 12.556 2.45093 12.801 2.587L20.001 6.587C20.525 6.878 20.85 7.43 20.85 8.03V15.97C20.85 16.57 20.525 17.122 20.001 17.413L12.801 21.413C12.556 21.5491 12.2803 21.6205 12 21.6205C11.7197 21.6205 11.4441 21.5491 11.199 21.413L3.99902 17.413C3.74165 17.2701 3.52719 17.0609 3.37791 16.8072C3.22862 16.5534 3.14994 16.2644 3.15002 15.97V8.03C3.15002 7.43 3.47502 6.878 3.99902 6.587L11.199 2.587ZM12.073 3.899C12.0507 3.88656 12.0256 3.88004 12 3.88004C11.9745 3.88004 11.9493 3.88656 11.927 3.899L4.72702 7.899C4.70384 7.91192 4.68449 7.93076 4.67096 7.9536C4.65743 7.97644 4.65021 8.00246 4.65002 8.029V15.971C4.65002 16.025 4.68002 16.075 4.72702 16.101L11.927 20.101C11.9493 20.1134 11.9745 20.12 12 20.12C12.0256 20.12 12.0507 20.1134 12.073 20.101L19.273 16.101C19.2962 16.0881 19.3156 16.0692 19.3291 16.0464C19.3426 16.0236 19.3498 15.9975 19.35 15.971V8.03C19.35 8.00329 19.3429 7.97706 19.3293 7.95403C19.3158 7.931 19.2964 7.912 19.273 7.899L12.073 3.899Z" fill="#383838"/>
                                <path fill-rule="evenodd" clip-rule="evenodd" d="M7.25 12C7.25 10.7402 7.75045 9.53204 8.64124 8.64124C9.53204 7.75045 10.7402 7.25 12 7.25C13.2598 7.25 14.468 7.75045 15.3588 8.64124C16.2496 9.53204 16.75 10.7402 16.75 12C16.75 13.2598 16.2496 14.468 15.3588 15.3588C14.468 16.2496 13.2598 16.75 12 16.75C10.7402 16.75 9.53204 16.2496 8.64124 15.3588C7.75045 14.468 7.25 13.2598 7.25 12ZM12 8.75C11.138 8.75 10.3114 9.09241 9.7019 9.7019C9.09241 10.3114 8.75 11.138 8.75 12C8.75 12.862 9.09241 13.6886 9.7019 14.2981C10.3114 14.9076 11.138 15.25 12 15.25C12.862 15.25 13.6886 14.9076 14.2981 14.2981C14.9076 13.6886 15.25 12.862 15.25 12C15.25 11.138 14.9076 10.3114 14.2981 9.7019C13.6886 9.09241 12.862 8.75 12 8.75Z" fill="#383838"/>
                            </svg>
                        </button>
                    </div>
                    <div className="self-stretch h-12 py-2 inline-flex justify-start items-start gap-2.5 overflow-hidden">
                        <div className="self-stretch flex justify-start items-center gap-1 overflow-hidden">
                            <img className="size-8 rounded-[999px]" src="https://placehold.co/32x32" />
                            <div className="w-10 h-6 justify-center text-neutral-700 text-base font-bold font-['Inter']">罗*昊</div>
                            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M6 12L10 8L6 4" stroke="#383838" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M15 18L14.278 14.75M2 8C2.74835 10.0508 4.10913 11.8219 5.8979 13.0733C7.68667 14.3247 9.81695 14.9959 12 14.9959C14.1831 14.9959 16.3133 14.3247 18.1021 13.0733C19.8909 11.8219 21.2516 10.0508 22 8M20 15L18.274 12.95M4 15L5.726 12.95M9 18L9.722 14.75" stroke="#383838" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </div>
                    </div>
                    <div className="self-stretch px-4 py-3 bg-rose-100 rounded-xl shadow-[0px_0px_1px_0px_rgba(0,0,0,0.40)] inline-flex justify-start items-center gap-12 overflow-hidden">
                        <div className="w-32 inline-flex flex-col justify-center items-start gap-3">
                            <div className="self-stretch h-4 justify-center text-neutral-700 text-sm font-bold font-['Inter']">今日收入(元)</div>
                            <div className="self-stretch justify-center text-red-800 text-xl font-black font-['Inter']">0.00</div>
                            <div className="self-stretch inline-flex justify-start items-center gap-1 overflow-hidden">
                                <div className="w-14 justify-center text-rose-950 text-sm font-medium font-['Inter']">我的钱包</div>
                                <div data-svg-wrapper className="relative">
                                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                        <path d="M6 12L10 8L6 4" stroke="#46151B" strokeWidth="1.33333" strokeLinecap="round" strokeLinejoin="round"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <div className="w-32 inline-flex flex-col justify-center items-start gap-3 overflow-hidden">
                            <div className="self-stretch h-4 justify-center text-neutral-700 text-sm font-bold font-['Inter']">今日完单量(单)</div>
                            <div className="self-stretch inline-flex justify-start items-center">
                                <div className="justify-center text-red-800 text-xl font-black font-['Inter']">0.00</div>
                                <div className="w-14 justify-center text-red-800 text-sm font-medium font-['Inter']">(含0趟)</div>
                            </div>
                            <div className="self-stretch inline-flex justify-start items-center gap-1">
                                <div className="w-14 justify-center text-rose-950 text-sm font-medium font-['Inter']">订单统计</div>
                                <div data-svg-wrapper className="relative">
                                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                        <path d="M6 12L10 8L6 4" stroke="#46151B" strokeWidth="1.33333" strokeLinecap="round" strokeLinejoin="round"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="self-stretch px-3 py-1.5 inline-flex justify-center items-center gap-6 overflow-hidden">
                        <div className="flex-1 px-1 flex justify-center items-center gap-1 overflow-hidden">
                            <div data-svg-wrapper className="relative">
                                <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M10.279 8.99225L3.56208 15.7417C3.38397 15.9185 3.2426 16.1288 3.14614 16.3604C3.04967 16.5921 3 16.8406 3 17.0915C3 17.3425 3.04967 17.5909 3.14614 17.8226C3.2426 18.0543 3.38397 18.2645 3.56208 18.4413C3.92178 18.7991 4.40849 19 4.91584 19C5.42318 19 5.90989 18.7991 6.26959 18.4413L13.0084 11.7219M16.3809 13.8082L18.7336 11.4553C18.9042 11.2847 19 11.0533 19 10.812C19 10.5707 18.9042 10.3393 18.7336 10.1687L11.832 3.2664C11.6614 3.09582 11.43 3 11.1888 3C10.9476 3 10.7162 3.09582 10.5456 3.2664L8.19288 5.61934C8.02232 5.78997 7.9265 6.02136 7.9265 6.26262C7.9265 6.50389 8.02232 6.73528 8.19288 6.9059L15.0945 13.8082C15.2651 13.9788 15.4965 14.0746 15.7377 14.0746C15.979 14.0746 16.2103 13.9788 16.3809 13.8082Z" stroke="#383838" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div className="w-14 justify-center text-neutral-700 text-sm font-medium font-['Inter']">违规申诉</div>
                        </div>
                        <div className="flex-1 px-11 flex justify-center items-center gap-1 overflow-hidden">
                            <div data-svg-wrapper className="relative">
                                <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M13.3333 1.66666H6.66663C3.90913 1.66666 1.66663 3.90916 1.66663 6.66666V17.5C1.66663 17.721 1.75442 17.933 1.9107 18.0892C2.06698 18.2455 2.27895 18.3333 2.49996 18.3333H13.3333C16.0908 18.3333 18.3333 16.0908 18.3333 13.3333V6.66666C18.3333 3.90916 16.0908 1.66666 13.3333 1.66666ZM16.6666 13.3333C16.6666 15.1717 15.1716 16.6667 13.3333 16.6667H3.33329V6.66666C3.33329 4.82832 4.82829 3.33332 6.66663 3.33332H13.3333C15.1716 3.33332 16.6666 4.82832 16.6666 6.66666V13.3333Z" fill="#383838"/>
                                    <path d="M5.83337 7.5H14.1667V9.16667H5.83337V7.5ZM5.83337 10.8333H11.6667V12.5H5.83337V10.8333Z" fill="#383838"/>
                                </svg>
                            </div>
                            <div className="w-14 justify-center text-neutral-700 text-sm font-medium font-['Inter']">违规申诉</div>
                        </div>
                    </div>
                </div>

                {/*<div className="grid flex-1 auto-rows-min gap-6 px-4">*/}
                {/*    <div className="grid gap-3">*/}
                {/*    </div>*/}
                {/*    <div className="grid gap-3">*/}

                {/*    </div>*/}
                {/*</div>*/}
                {/*<SheetFooter>*/}
                {/*    <Button type="submit">Save changes</Button>*/}
                {/*    <SheetClose asChild>*/}
                {/*        <Button variant="outline">Close</Button>*/}
                {/*    </SheetClose>*/}
                {/*</SheetFooter>*/}
            </SheetContent>
        </Sheet>
    )
}
