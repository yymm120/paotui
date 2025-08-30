import { MotorCarIcon } from "@/components/delivery/icons/motor-car-icon.tsx";
import { CarIcon } from "@/components/delivery/icons/car-icon.tsx";
import { useState } from "react";

export function ToggleCarDelivery() {
  const [toggle, setToggle] = useState<"car" | "motor">("motor");

  return (
    <div className="h-10 px-1 bg-gray-100 rounded-lg flex justify-start items-center gap-0.5 overflow-hidden">
      {toggle === "motor" ? (
        <>
          <div className="w-[90px] h-[34px] px-[9px] py-[5px] bg-white rounded-lg flex justify-center items-center gap-1 overflow-hidden">
            <MotorCarIcon />
            <div className="w-[42px] h-4 justify-center text-black text-sm font-semibold font-['Inter']">
              电动车
            </div>
          </div>
          <div
            onClick={() => setToggle("car")}
            className="w-[72px] p-[5px] flex justify-center items-center gap-0.5 overflow-hidden"
          >
            <CarIcon />
            <div className="w-8 h-[17px] justify-center text-zinc-500 text-sm font-semibold font-['Inter']">
              汽车
            </div>
          </div>
        </>
      ) : (
        <>
          <div
            onClick={() => setToggle("motor")}
            className="w-[72px] p-[5px] flex justify-center items-center gap-0.5 overflow-hidden"
          >
            <MotorCarIcon />
            <div className="w-[42px] h-4 justify-center text-black text-sm font-semibold font-['Inter']">
              电动车
            </div>
          </div>
          <div className="w-[90px] h-[34px] px-[9px] py-[5px] bg-white rounded-lg flex justify-center items-center gap-1 overflow-hidden">
            <CarIcon />
            <div className="w-8 h-[17px] justify-center text-zinc-500 text-sm font-semibold font-['Inter']">
              汽车
            </div>
          </div>
        </>
      )}
    </div>
  );
}
