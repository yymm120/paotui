import { useEffect, useRef, useState, useCallback, useContext } from "react";
import { ProfileContext } from "@/components/profile-context-provider.tsx";

// type MessageHandler = (event: MessageEvent) => void;
// type WebSocketStatus = 'connecting' | 'connected' | 'disconnected' | 'error';

export function useWebSocket(url: string, protocols: string[] = []) {
  const [send] = useState();
  const { userWorkingStatus } = useContext(ProfileContext);
  // const [status, setStatus] = useState<WebSocketStatus>('connecting');
  // const [lastMessage, setLastMessage] = useState<string | null>(null);
  // const wsRef = useRef<WebSocket | null>(null);
  // const handlersRef = useRef<Set<MessageHandler>>(new Set());
  // const reconnectAttempts = useRef(0);

  // // 稳定的消息发送函数
  // const sendMessage = useCallback((message: string) => {
  //   if (wsRef.current?.readyState === WebSocket.OPEN) {
  //     try {
  //       wsRef.current.send(message);
  //     } catch (err) {
  //       console.error('QuerySocket send error:', err);
  //     }
  //   }
  // }, []);
  //
  // // 消息处理器管理
  // const addMessageHandler = useCallback((handler: MessageHandler) => {
  //   handlersRef.current.add(handler);
  //   return () => handlersRef.current.delete(handler);
  // }, []);

  // const initializeWebSocket = useCallback(() => {
  //   const ws = new WebSocket(url, protocols || []);
  //
  //   ws.onopen = () => {
  //     reconnectAttempts.current = 0;
  //     setStatus('connected');
  //     wsRef.current = ws;
  //   };
  //
  //   ws.onmessage = (event) => {
  //     setLastMessage(event.data);
  //     handlersRef.current.forEach(handler => handler(event));
  //   };
  //
  //   ws.onclose = () => {
  //     setStatus('disconnected');
  //     wsRef.current = null;
  //     reconnect();
  //   };
  //
  //   ws.onerror = () => {
  //     console.log("into error");
  //     setStatus('error');
  //     reconnect();
  //   };
  // }, [url, protocols]);

  // const url = "wss://192.168.10.107:4000/ws"
  // const protocol = "task"

  const websocketRef = useRef<WebSocket | undefined>(undefined);
  // const user_working_status = useState<"working" | "rest">("rest")
  const websocketRetryTimer = useRef<Timer | undefined>(undefined);

  const initializeWebSocket = (url: string, protocols: string[]) => {
    if (userWorkingStatus === "working") {
      const websocket = new WebSocket(url, [...protocols]);

      websocket.onerror = (err) => {
        console.error("websocket error:", err);
        // if (user_working_status === "working") {
        //   try_reconnection({frequency: 1000})
        // }
      };

      websocket.onclose = () => {
        console.error("websocket意外关闭!");
        if (userWorkingStatus === "working") {
          websocketRef.current = undefined;
          try_reconnection({ frequency: 3000 });
        }
      };

      websocketRef.current = websocket;
    }
    if (userWorkingStatus === "rest") {
      if (websocketRef.current) {
        websocketRef.current.close();
      }
      if (websocketRetryTimer.current) {
        clearTimeout(websocketRetryTimer.current);
      }
    }
  };

  const try_reconnection = useCallback(
    ({ frequency }: { frequency: number }) => {
      const retryHandle = setTimeout(() => {
        initializeWebSocket(url, [...protocols]);
      }, frequency);
      websocketRetryTimer.current = retryHandle;
      return () => clearTimeout(retryHandle);
    },
    [],
  );
  useEffect(() => {
    initializeWebSocket(url, [...protocols]);
  }, [userWorkingStatus]);

  return {
    send,
    // status,
    // isConnected: status === 'connected',
    // lastMessage,
    // sendMessage,
    // addMessageHandler
  };
}
