import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";

const orderData = [
    {
        id: 1,
        deliveryTime: "30分钟内",
        deliveryDeadline: "(19:45前)送达",
        rating: "4.8",
        distance1: "300",
        distance2: "300",
        storeName: "蜜雪冰城-德润城店",
        storeAddress: "义务大道337号",
        destination: "重庆华府酒店(财富广场店)八楼802",
        badgeText: "新人体验单",
        itemDescription: "货品：食品小吃·2公斤·2件",
        notes: "备注：other-#其他",
    },
    {
        id: 2,
        deliveryTime: "30分钟内",
        deliveryDeadline: "(19:45前)送达",
        rating: "4.8",
        distance1: "300",
        distance2: "300",
        storeName: "蜜雪冰城-德润城店",
        storeAddress: "义务大道337号",
        destination: "重庆华府酒店(财富广场店)八楼802",
        badgeText: "新人体验单",
        itemDescription: "货品：食品小吃·2公斤·2件",
        notes: "备注：other-#其他",
    },
];

export default function MainContentSection() {
    return (
        <section className="flex flex-col w-full items-start gap-2.5 p-3 bg-[#f2f2f2] overflow-hidden">
            {orderData.map((order) => (
                <Card
                    key={order.id}
                    className="flex flex-col items-start gap-3 px-0 py-3 w-full bg-white rounded-2xl overflow-hidden shadow-[0px_0px_1px_1px_#f3f3f31a]"
                >
                    <CardContent className="w-full p-0">
                        <div className="h-9 justify-between px-3 py-1 flex items-center w-full">
                            <div className="w-[186px] h-[26px] [font-family:'Inter-Medium',Helvetica] font-medium text-base text-center tracking-[0] leading-[normal]">
                                <span className="text-[#df711e]">{order.deliveryTime}</span>
                                <span className="text-black">{order.deliveryDeadline}</span>
                            </div>
                            <div className="w-[51px] h-[21px] [font-family:'Inter-SemiBold',Helvetica] font-semibold text-black text-base text-center tracking-[0] leading-[normal]">
                                {order.rating}
                            </div>
                        </div>

                        <div className="justify-center px-3 py-0 flex items-center w-full">
                            <div className="flex flex-col items-start gap-3 flex-1 grow">
                                <div className="flex items-start gap-2.5 w-full">
                                    <div className="flex w-9 items-center gap-2.5 px-0.5 py-0 bg-white">
                                        <div className="flex flex-col h-[100px] items-center justify-center gap-[3px] px-1 py-[7px] flex-1 grow bg-[#f8f8f8] rounded-2xl overflow-hidden">
                                            <div className="flex flex-col h-6 items-center w-full">
                                                <div className="mt-[-1.00px] [font-family:'Inter-SemiBold',Helvetica] font-semibold text-black text-xs text-center tracking-[0] leading-[normal]">
                                                    {order.distance1}
                                                </div>
                                                <div className="[font-family:'Inter-Regular',Helvetica] font-normal text-[#969696] text-[8px] text-center tracking-[0] leading-[normal]">
                                                    km
                                                </div>
                                            </div>
                                            <Separator
                                                className="w-px h-6 bg-[#adadad]"
                                                orientation="vertical"
                                            />
                                            <div className="flex flex-col h-6 items-center w-full">
                                                <div className="mt-[-1.00px] [font-family:'Inter-SemiBold',Helvetica] font-semibold text-black text-xs text-center tracking-[0] leading-[normal]">
                                                    {order.distance2}
                                                </div>
                                                <div className="[font-family:'Inter-ExtraLight',Helvetica] font-extralight text-[#969696] text-[8px] text-center tracking-[0] leading-[normal]">
                                                    km
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    <div className="flex flex-col h-[100px] items-start justify-center gap-4 px-0 py-1 flex-1 grow">
                                        <div className="flex flex-col h-12 items-start justify-center gap-3 px-0 py-[3px] w-full">
                                            <div className="h-4 [font-family:'Inter-SemiBold',Helvetica] font-semibold text-black text-xl whitespace-nowrap tracking-[0] leading-[normal]">
                                                {order.storeName}
                                            </div>
                                            <div className="h-2.5 [font-family:'Inter-Light',Helvetica] font-light text-[#292929] text-sm whitespace-nowrap tracking-[0] leading-[normal]">
                                                {order.storeAddress}
                                            </div>
                                        </div>
                                        <div className="flex flex-col h-6 items-start justify-center gap-3.5 px-0 py-[3px] w-full">
                                            <div className="[font-family:'Inter-SemiBold',Helvetica] font-semibold text-black text-xl tracking-[0] leading-[normal]">
                                                {order.destination}
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div className="flex items-start gap-2.5 w-full">
                                    <div className="w-9 h-[100px] bg-white" />
                                    <div className="flex flex-col items-start justify-center gap-1.5 flex-1 grow">
                                        <Badge
                                            variant="outline"
                                            className="inline-flex items-center justify-center gap-2.5 px-1 py-1.5 rounded overflow-hidden border border-solid border-[#323232]"
                                        >
                                            <div className="w-[60px] h-2 mt-[-1.00px] [font-family:'Inter-Regular',Helvetica] font-normal text-[#323232] text-xs whitespace-nowrap tracking-[0] leading-[normal]">
                                                {order.badgeText}
                                            </div>
                                        </Badge>
                                        <div className="w-full h-9 bg-[#f8f8f8] rounded overflow-hidden relative">
                                            <div className="absolute w-[322px] h-3 top-[11px] left-3 [font-family:'Inter-Regular',Helvetica] font-normal text-[#363636] text-base tracking-[0] leading-[normal]">
                                                {order.itemDescription}
                                            </div>
                                        </div>
                                        <div className="w-full h-9 bg-[#fff3e5] rounded relative">
                                            <div className="absolute w-[322px] h-3 top-[11px] left-3 [font-family:'Inter-Regular',Helvetica] font-normal text-[#4a361b] text-base tracking-[0] leading-[normal]">
                                                {order.notes}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="flex flex-col h-12 items-center justify-center gap-2.5 px-3 py-2.5 w-full bg-white">
                            <Button className="w-full h-12 mt-[-10.00px] mb-[-10.00px] bg-[#ffd0b2] rounded-lg hover:bg-[#ffd0b2]/90 h-auto">
                                <div className="w-[84px] h-[17px] [font-family:'Inter-SemiBold',Helvetica] font-semibold text-white text-base whitespace-nowrap text-center tracking-[0] leading-[normal]">
                                    接单
                                </div>
                            </Button>
                        </div>
                    </CardContent>
                </Card>
            ))}
        </section>
    );
}
