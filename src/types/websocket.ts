// types/query-socket.ts
export type WebSocketMessage = string | Uint8Array | Blob;

import type WebSocket from "@tauri-apps/plugin-websocket";

export interface WebSocketCallbacks {
  onMessage?: (msg: WebSocketMessage) => void;
  onOpen?: () => void;
  onClose?: (event?: CloseEvent) => void;
  onError?: (error: Event) => void;
}

export interface UseTauriWebSocketReturn {
  socket: WebSocket | null;
  isConnected: boolean;
  lastMessage: WebSocketMessage | null;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
  sendMessage: (message: string | object) => Promise<void>;
  queueMessage: (message: string | object) => void;
}
