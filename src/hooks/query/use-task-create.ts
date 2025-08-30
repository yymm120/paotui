import { useMutation } from "@tanstack/react-query";
import { Query } from "@/utils/query.ts";
import { z } from "zod";
import { FormSchemaForCreateDeliveryTask } from "@/api";

const createTask = async (
  data: z.infer<typeof FormSchemaForCreateDeliveryTask>,
): Promise<any> => {
  // console.log(JSON.stringify(data));

  try {
    const response = await Query("https://192.168.10.107:4000/api/task", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    return await response.json();
  } catch (error) {
    console.error("发送验证码失败:", error);
    return { success: false, message: "网络请求失败" };
  }
};

export const useTaskCreate = () => {
  return useMutation({
    mutationFn: createTask,
    onSuccess: (data) => {
      console.log(data);
    },
  });
};
