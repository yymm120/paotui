"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
// import { toast } from "sonner"
import { z } from "zod"

import { Button } from "@/components/ui/button"
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import {type Dispatch, type SetStateAction, useState} from "react";
// import {auth_login} from "@/api/auth_login.ts";
import {auth_code, type AuthCodeResult} from "@/api/auth_code.ts";
import {invoke} from "@tauri-apps/api/core";

const FormSchema = z.object({
    phone_number: z.string().min(2, {
        message: "Username must be at least 2 characters.",
    }),
    code: z.string().optional(),
})



/**
 * @component LoginPage
 * @description 登录页面
 * @param setLogin
 * @constructor
 */
export function LoginPage({ setLogin }: { setLogin: Dispatch<SetStateAction<boolean>>}) {
    // useRef("code")
    const [actionType, setActionType] = useState("code");
    const form = useForm<z.infer<typeof FormSchema>>({
        resolver: zodResolver(FormSchema),
        defaultValues: {
            phone_number: "",
        },
    })

    async function onSubmit(data: z.infer<typeof FormSchema>) {
        console.log("login", data)
        if (data.code) {

            invoke('auth_login', { user_phone: data.phone_number, code: data.code }).then((message: unknown) => {
                console.log("login", message);
            });
            console.log("login", data)
            // await auth_login(data)

            setLogin(true)
        } else {
            // 获取验证码
            try {
                // 如果请请求失败，提示"当前服务器不可用，请点击此处通知后台"

                const {code}: AuthCodeResult = await auth_code();
                console.log(code)
                setActionType("login")
            } catch (error) {
                console.log("error:")
                console.log(error)
            }
        }

        // toast("You submitted the following values", {
        //     description: (
        //         <pre className="mt-2 w-[320px] rounded-md bg-neutral-950 p-4">
        //   <code className="text-white">{JSON.stringify(data, null, 2)}</code>
        // </pre>
        //     ),
        // })
    }
    return (

        <div className="w-full min-h-screen px-4 bg-white inline-flex flex-col justify-start items-center overflow-hidden">
            <div className="self-stretch h-56 px-16 py-12 flex flex-col justify-center items-center gap-2.5 overflow-hidden">
                <div className="w-52 text-center justify-start text-black text-base font-bold font-['Inter'] leading-normal">涉外校园送 - 兼职跑腿接单<br/>平台</div>
            </div>
            <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)} className={"self-stretch px-1 pt-5 flex flex-col justify-start items-start gap-8 overflow-hidden"}>
                    <FormField
                        control={form.control}
                        name="phone_number"
                        render={({ field }) => (
                            <FormItem className={"w-full size- flex flex-col justify-start items-start gap-1.5"}>
                                <FormLabel className={"justify-start text-tokens-foreground text-sm font-bold font-['Inter'] leading-tight"}>请输入手机号</FormLabel>
                                <div className="w-full inline-flex justify-start items-start gap-2">
                                    <div className="flex-1 inline-flex flex-col justify-start items-start gap-1.5">
                                        <FormControl>
                                            <Input className="justify-start text-slate-600 text-base font-normal font-['Inter'] leading-normal" placeholder="手机号" {...field} />
                                        </FormControl>
                                    </div>
                                </div>
                                <FormDescription className="justify-start text-slate-500 text-sm font-normal font-['Inter'] leading-tight">
                                    未注册的手机号，验证后自动注册
                                </FormDescription>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    {actionType === "login" && (
                        <>
                        <FormField
                            control={form.control}
                            name="code"
                            render={({ field }) => (
                                <FormItem className={"w-full size- flex flex-col justify-start items-start gap-1.5"}>
                                    {/*<FormLabel className={"justify-start text-tokens-foreground text-sm font-bold font-['Inter'] leading-tight"}>请输入手机号</FormLabel>*/}
                                    <FormControl>
                                        <Input className="justify-start text-slate-400 text-base font-normal font-['Inter'] leading-normal" placeholder="验证码" {...field} />
                                    </FormControl>
                                    {/*<div className="w-full inline-flex justify-start items-start gap-2">*/}
                                    {/*    <div className="flex-1 inline-flex flex-col justify-start items-start gap-1.5">*/}
                                    {/*        <FormControl>*/}
                                    {/*            <Input className="justify-start text-slate-400 text-base font-normal font-['Inter'] leading-normal" placeholder="手机号" {...field} />*/}
                                    {/*        </FormControl>*/}
                                    {/*    </div>*/}
                                    {/*</div>*/}
                                    {/*<FormDescription className="justify-start text-slate-500 text-sm font-normal font-['Inter'] leading-tight">*/}
                                    {/*    未注册的手机号，验证后自动注册*/}
                                    {/*</FormDescription>*/}
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                            <Button type="submit" className={"w-full"}>登录</Button>
                        </>
                    )}
                    {actionType === "code" && (
                        <Button type="submit" className={"w-full cursor-pointer"} >获取验证码</Button>
                    )}
                </form>
            </Form>

    <div className="self-stretch flex-1 flex flex-col justify-end items-center overflow-hidden">
        <div className="self-stretch py-2.5 flex flex-col justify-center items-center gap-2.5 overflow-hidden">
            <div data-caption="true" data-disabled="no" data-filled="false" data-label="true" data-required="false" data-style="New York" data-type="With Label" className="size- inline-flex justify-start items-center gap-2">
            <div data-svg-wrapper className="relative">
                <svg width="17" height="16" viewBox="0 0 17 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M8.5 0.5C12.6421 0.5 16 3.85786 16 8C16 12.1421 12.6421 15.5 8.5 15.5C4.35786 15.5 1 12.1421 1 8C1 3.85786 4.35786 0.5 8.5 0.5Z" stroke="#64748B"/>
                </svg>
            </div>
            <div className="size- flex justify-start items-start gap-0.5">
                <div className="justify-center text-slate-500 text-sm font-normal font-['Inter'] leading-tight">已阅读并同意平台用户服务协议和涉外校园送App隐私政策</div>
            </div>
        </div>
    </div>
</div>
</div>
    )
}
