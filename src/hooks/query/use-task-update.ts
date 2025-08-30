import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Query } from "@/utils/query.ts";

// import {normalize_response_body_for__delivery_task_list} from "@/api";

const updateTask = async ({ id, status }: { id: string; status: string }) => {
  try {
    const response = await Query(
      `https://192.168.10.107:4000/api/task/${id}?status=${status}`,
      {
        method: "PATCH",
      },
    );
    return await response.json();
  } catch (error) {
    console.error("list task failed:", error);
    // return { success: false, message: '网络请求失败' };
    return [];
  }
};

export const useTaskUpdate = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: updateTask,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["task-list"] });
    },
  });
};
