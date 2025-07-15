
export interface DeliveryOrder {
    id: string;
    // 跑腿任务的下发时间
    time_order: Date | undefined;
    // 跑腿的送达时间
    time_arrived: Date | undefined;
    // 跑腿的剩余时间
    time_remaining: number;
    // 跑腿的接单时间
    time_pickup?: Date,

    // 店铺地址
    address_store: string;
    // 当前地址
    address_current: string;
    // 顾客地址
    address_customer: string;

    // 当前地址到店铺的距离
    distance_current_to_store: string;
    // 店铺到顾客的距离
    distance_store_to_customer: string;
    // 当前地址到顾客的距离
    distance_current_to_customer: number;

    // 标记
    tag?: string;
    // 货品清单
    items: string;
    // 备注
    notes?: string;
    // 运送状态
    status: "new" | "pickup" | "delivery" | "completed";
    // 运送优先级
    priority: "high" | "medium" | "low";
    // 预计收入
    estimatedEarnings: number;
}
