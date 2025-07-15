import { cn } from "@/lib/utils";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DistanceIndicator } from "./DistanceIndicator";
import { type DeliveryOrder } from "@/types";

interface DeliveryCardProps {
  order: DeliveryOrder;
  className?: string;
  onAcceptOrder?: (orderId: string) => void;
}

export function DeliveryCard({
  order,
  className,
  onAcceptOrder,
}: DeliveryCardProps) {
  return (
    <Card
      className={cn(
        "w-full rounded-2xl bg-white p-0 shadow-sm border-0",
        className,
      )}
    >
      {/* Header */}
      <CardHeader className="flex h-auto flex-row items-center justify-between px-3 py-2 sm:h-9 sm:py-1">
        <div className="flex-1 text-left text-sm font-bold text-black sm:h-[26px] sm:w-[186px] sm:text-center sm:text-base">
          {order.time_arrived?.getTime()}
        </div>
        <div className="flex-shrink-0 text-right text-sm font-bold text-black sm:h-[21px] sm:w-[51px] sm:text-center sm:text-base">
          {order.estimatedEarnings}
        </div>
      </CardHeader>

      {/* Content */}
      <CardContent className="flex w-full items-center justify-center px-3 py-0">
        <div className="flex flex-1 flex-col items-start gap-3">
          {/* Address Section */}
          <div className="flex w-full items-start gap-3">
            <DistanceIndicator
              fromDistance={order.distance_current_to_store}
              toDistance={order.distance_store_to_customer}
            />

            <div className="flex min-h-[100px] flex-1 flex-col justify-center items-start gap-3 py-1 sm:h-[100px] sm:gap-4">
              {/* From Section */}
              <div className="flex w-full flex-shrink-0 flex-col justify-center items-start gap-2 sm:h-12 sm:gap-3 sm:py-1">
                <div className="w-full flex-shrink-0  text-lg font-bold text-black leading-tight sm:h-4 sm:text-xl">
                  {order.address_store}
                </div>
                <div className="w-full flex-shrink-0 text-xs font-normal text-neutral-800 sm:h-[10px] sm:text-sm">
                  {order.address_current}
                </div>
              </div>

              {/* To Section */}
              <div className="w-full flex-shrink-0 text-lg font-bold text-black leading-tight sm:h-6 sm:py-1 sm:text-xl">
                {order.address_customer}
              </div>
            </div>
          </div>

          {/* Details Section */}
          <div className="flex w-full items-start gap-3">
            {/* Empty space for alignment */}
            <div className="flex w-9 items-center gap-2.5 px-0.5 sm:h-[100px]" />

            <div className="flex flex-1 flex-col items-start justify-center gap-2 sm:gap-1.5">
              {/* Tag */}
              {order.tag && (
                <div className="rounded border border-neutral-800 px-2 py-1 text-xs font-normal text-neutral-800 sm:px-1 sm:py-1.5">
                  {order.tag}
                </div>
              )}

              {/* Items */}
              <div className="w-full rounded bg-neutral-100 p-3 text-sm font-normal text-neutral-700 sm:relative sm:left-3 sm:top-3 sm:h-9 sm:text-base">
                {order.items}
              </div>

              {/* Notes */}
              {order.notes && (
                <div className="w-full rounded bg-orange-50 p-3 text-sm font-normal text-amber-900 sm:relative sm:left-3 sm:top-3 sm:h-9 sm:text-base">
                  {order.notes}
                </div>
              )}
            </div>
          </div>
        </div>
      </CardContent>

      {/* Footer */}
      <CardFooter className="flex rounded-2xl h-auto w-full flex-col items-center justify-center gap-2.5 bg-white px-3 py-3 sm:h-12 sm:py-2.5">
        <Button
          onClick={() => onAcceptOrder?.(order.id)}
          className="h-12 w-full flex-shrink-0 rounded-lg bg-orange-200 px-4 py-3 text-base font-bold text-white active:bg-orange-300 sm:relative sm:left-1/2 sm:top-4 sm:-translate-x-1/2 sm:transform sm:px-0 sm:py-4"
        >
          {"接单"}
        </Button>
      </CardFooter>
    </Card>
  );
}
