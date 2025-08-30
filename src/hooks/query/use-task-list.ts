import { useQuery } from "@tanstack/react-query";
import { Query } from "@/utils/query.ts";
import { normalize_response_body_for__delivery_task_list } from "@/api";

const listTask = async (status: string, orderBy: string) => {
  try {
    const response = await Query(
      `https://192.168.10.107:4000/api/task?status=${status}`,
      {
        method: "GET",
      },
    );
    let data = await response.json();

    let res_data = normalize_response_body_for__delivery_task_list(data);
    if (orderBy === "收益优先") {
      res_data = res_data.sort((a, b) => {
        if (a.estimated_in_come > b.estimated_in_come) return -1; // 收入高在前
        if (a.estimated_in_come < b.estimated_in_come) return 1; // 收入低在后
        return 0; // 收入相同不变
      });
    } else if (orderBy === "距离优先") {
      res_data = res_data.sort((a, b) => {
        if (a.distance_store_to_customer > b.distance_store_to_customer)
          return 1; // 距离远的在后
        if (a.distance_store_to_customer < b.distance_store_to_customer)
          return -1; // 距离近的在前
        return 0;
      });
    }

    return res_data;
  } catch (error) {
    console.error("list task failed:", error); // TODO: error Toast
    return [];
  }
};

export const useTaskList = (status: string, orderBy: string = "综合排序") => {
  const { isLoading, data, isRefetching } = useQuery({
    queryKey: ["task-list"],
    queryFn: () => listTask(status, orderBy),
  });

  return { isLoading, data, isRefetching };
};

