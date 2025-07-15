import { fetch } from '@tauri-apps/plugin-http';

export interface AuthCodeResult {
    code:  string;
}

export async function auth_code(): Promise<Response>{
    console.log("into auth_code")
    return await fetch('http://127.0.0.1:3000/api/code', {
        method: 'GET',
    });
    // console.log("end auth_code")
    // console.log(response.json());
    // return response.json()
}