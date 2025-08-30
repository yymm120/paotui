import { Query_update_delivery_task } from "@/api/task/query";

type Status = "new" | "pending" | "done";

export const UpdateDataForTask = {
  symbol: Query_update_delivery_task,
  path: "/api/task/{id}",
  method: "PATCH",
  pathParams: {
    id: undefined as unknown as string,
  },
  body: {
    status: undefined as unknown as Status,
  },
} as const;
