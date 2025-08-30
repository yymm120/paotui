"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input.tsx";
import { useLogin } from "@/hooks/query/use-login.ts";
import { useCode } from "@/hooks/query/use-code.ts";

// 表单验证规则
const FormSchema = z.object({
  phone: z
    .string()
    .min(11, "手机号必须是11位数字")
    .max(11, "手机号必须是11位数字"),
  code: z.string().min(4, "验证码必须是4位数字").optional(),
});

export function LoginPage({
  setLogin,
}: {
  setLogin: (loggedIn: boolean) => void;
}) {
  const [step, setStep] = useState<"requestCode" | "verifyCode">("requestCode");
  const [countdown, setCountdown] = useState(0);

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      phone: "",
      code: "",
    },
  });

  const code = useCode();
  const login = useLogin();

  // 发送验证码函数
  const sendCode = async () => {
    const phone = form.getValues("phone");
    if (!phone || phone.length !== 11) {
      form.setError("phone", { message: "请输入有效的手机号" });
      return;
    }

    try {
      code.mutate(phone);
      console.log("发送验证码到:", phone);
      await new Promise((resolve) => setTimeout(resolve, 1000)); // 模拟网络请求

      // 开始倒计时
      setCountdown(0);
      const timer = setInterval(() => {
        setCountdown((prev) => {
          if (prev <= 1) clearInterval(timer);
          return prev - 1;
        });
      }, 1000);

      setStep("verifyCode");
    } catch (error) {
      console.error("发送验证码失败:", error);
    }
  };

  // 登录函数
  const handleLogin = async (data: z.infer<typeof FormSchema>) => {
    console.log("into login");
    if (!data.code || data.code.length !== 4) {
      form.setError("code", { message: "请输入有效的验证码" });
      return;
    }
    try {
      login.mutate({ phone: data.phone, code: data.code! });
      console.log("登录请求:", { phone: data.phone, code: data.code });
      await new Promise((resolve) => setTimeout(resolve, 1000)); // 模拟网络请求
      setLogin(true);
    } catch (error) {
      console.error("登录失败:", error);
    }
  };

  return (
    <div className="w-full max-w-md mx-auto p-4 bg-white rounded-lg shadow">
      <div className="text-center py-8">
        <h1 className="text-2xl font-bold text-gray-800">
          涉外校园送 - 兼职跑腿接单平台
        </h1>
      </div>

      <Form {...form}>
        <form onSubmit={form.handleSubmit(handleLogin)} className="space-y-4">
          {/* 手机号输入 */}
          <FormField
            control={form.control}
            name="phone"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input placeholder="请输入手机号" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* 验证码输入 - 仅在验证阶段显示 */}
          {step === "verifyCode" && (
            <div className="flex gap-2">
              <FormField
                control={form.control}
                name="code"
                render={({ field }) => (
                  <FormItem className="flex-1">
                    <FormControl>
                      <Input
                        placeholder="请输入验证码"
                        {...field}
                        // disabled={isSubmitting}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <Button
                type="button"
                onClick={sendCode}
                disabled={countdown > 0}
                className="w-32"
              >
                {countdown > 0 ? `${countdown}秒后重试` : "重新发送"}
              </Button>
            </div>
          )}

          <Button
            type={step === "requestCode" ? "button" : "submit"}
            onClick={step === "requestCode" ? sendCode : undefined}
            className="w-full"
          >
            {step === "requestCode" ? "获取验证码" : "登录"}
          </Button>
        </form>
      </Form>
    </div>
  );
}
