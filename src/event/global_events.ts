import { listen } from '@tauri-apps/api/event';
import type {AppStartedResult} from "@/event/event_model.ts";


listen<AppStartedResult>('download-started', (event) => {
    console.log(
        `app already started! user_type: ${event.payload.user.user_type}`
    );
});
