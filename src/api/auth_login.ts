import { fetch } from '@tauri-apps/plugin-http';


const getResponse = await fetch('https://api.tauri.app/data', {
    method: 'GET',
    headers: {
        'Authorization': 'Bearer token123'
    }
});