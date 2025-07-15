import { fetch } from '@tauri-apps/plugin-http';
// import { FetchOptions, Response, ResponseType,Body } from "@tauri-apps/api/http";

interface UserForLogin {
    phone_number: string,
    code: string
}

export const auth_login = async (user: UserForLogin) => await fetch('http://127.0.0.1:3000', {
    method: 'POST',
    headers: {
        'Authorization': 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzYWNjZWEyNi03YzVlLTRhNDktYWFmZC0xYTc5MWI4ZWJjZjkiLCJleHAiOjE3Njc0NDkzMzcsImlhdCI6MTc1MTg5NzMzNywicGhvbmUiOiIifQ.TQg5DtbA9gf4LuxnI0QG2IHgFDZSpPDOIL1jOqxDy3E',
        'Content-Type': 'application/json'
    },
    body: JSON.stringify(user)
});