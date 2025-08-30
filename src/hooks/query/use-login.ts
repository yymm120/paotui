import { Query } from "@/utils/query.ts";
import { useMutation } from "@tanstack/react-query";

const loginWithCode = async ({
  phone,
  code,
}: {
  phone: string;
  code: string;
}): Promise<any> => {
  try {
    const response = await Query("https://192.168.10.107:4000/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ user_phone: phone, code }),
    });

    const token =
      response.headers.get("Authorization") ||
      response.headers.get("authorization");
    const data = await response.json();

    return {
      ...data,
      token,
    };
  } catch (error) {
    console.error("登录失败:", error);
    return { success: false, message: "网络请求失败" };
  }
};

export const useLogin = () => {
  return useMutation({
    mutationFn: loginWithCode,
    onSuccess: (data) => {
      saveToken(data.token);
      console.log(data);
    },
  });
};

const saveToken = (token: string) => {
  // 可以同时存入 localStorage/sessionStorage
  localStorage.setItem("token", token);
};
