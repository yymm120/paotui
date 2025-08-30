import { useMutation } from "@tanstack/react-query";
import { Query } from "@/utils/query.ts";

const sendVerificationCode = async (phone: string): Promise<any> => {
  try {
    const response = await Query("https://localhost:4000/api/code", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ phone_number: phone }),
    });
    return await response.json();
  } catch (error) {
    console.error("发送验证码失败:", error);
    return { success: false, message: "网络请求失败" };
  }
};

export const useCode = () => {
  return useMutation({
    mutationFn: sendVerificationCode,
    onSuccess: (data) => {
      console.log(data); // TODO: 不需code, 应该在手机短信上
    },
  });
};
