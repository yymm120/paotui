
import { cn } from "@/lib/utils";
import { DeliveryCard } from "./DeliveryCard";
import { type DeliveryOrder } from "@/types";

interface DeliveryCardListProps {
  orders: DeliveryOrder[];
  className?: string;
  onAcceptOrder?: (orderId: string) => void;
}

export function DeliveryCardList({
  orders,
  className,
  onAcceptOrder,
}: DeliveryCardListProps) {
  return (
    <div
      className={cn(
        "flex w-full flex-col items-start gap-2 bg-neutral-100 px-2 py-2 sm:gap-2.5 sm:px-3",
        className,
      )}
    >
      {orders.map((order) => (
        <DeliveryCard
          key={order.id}
          order={order}
          onAcceptOrder={onAcceptOrder}
        />
      ))}
    </div>
  );
}
