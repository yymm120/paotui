/// 在应用启动时, 立即发送一个初始化请求
/// 1. 如果token已经存在, 就携带token访问
///     1. 如果token有效, 立即进入主页面
///     2. 如果token无效, 立即进入登录页面

import {fetch} from "@tauri-apps/plugin-http";
import { load } from '@tauri-apps/plugin-store';


export async function auth_init() {
    // 从android本地读取文件
    const store = await load('store.json');
    const {token} = await store.get<{ token: string }>('token') || {token: ""};
    const response = await fetch('http://127.0.0.1:3000/api/_', {
        method: 'GET',
        headers: {
            'Authorization': token,
        }
    });
    // 获得新的token
    console.log(response);
    await store.save();
    return response.json()
}