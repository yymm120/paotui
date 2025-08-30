import { useQuery } from "@tanstack/react-query";
import { Query } from "@/utils/query.ts";
import { useEffect, useState } from "react";

const initialQuery = async (): Promise<any> => {
  try {
    const response = await Query("https://192.168.10.107:4000/api/_", {
      method: "GET",
    });
    return await response.json();
  } catch (error) {
    console.error("初始化失败:", error);
    return { status: false, message: "网络请求失败" };
  }
};

const saveProfile = (data: any) => {
  localStorage.setItem("profile", JSON.stringify(data));
};

export const useInitial = () => {
  const [isLogin, setIsLogin] = useState(false);

  const { isLoading, data } = useQuery({
    queryKey: ["query"],
    queryFn: initialQuery,
  });

  useEffect(() => {
    if (!!data && !!data.status && data.user.user_type === "VerifiedUser") {
      setIsLogin(true);
      saveProfile(data.user);
    }
  }, [data]);
  return { isLoading, data, isLogin, setIsLogin };
};
