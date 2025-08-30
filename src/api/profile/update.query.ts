import { Query_update_delivery_user_status } from "@/api/profile/query";

type Status = "working" | "resting";

export const UpdateDataForProfile = {
  symbol: Query_update_delivery_user_status,
  path: "/api/profile",
  method: "PATCH",
  body: {
    status: undefined as unknown as Status,
  },
} as const;
