// hooks/useTauriWebSocket.ts
import { useState, useEffect, useCallback } from 'react';
import WebSocket, {type Message} from '@tauri-apps/plugin-websocket';
import type {
    WebSocketMessage,
    WebSocketCallbacks,
    UseTauriWebSocketReturn
} from '@/types';

export function useTauriWebSocket(
    url: string,
    callbacks?: WebSocketCallbacks
): UseTauriWebSocketReturn {
    const [socket, setSocket] = useState<WebSocket | null>(null);
    const [isConnected, setIsConnected] = useState(false);
    const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null);
    const [messageQueue, setMessageQueue] = useState<Array<string | object>>([]);

    const handleMessage = useCallback((msg: Message) => {
        // 将 Tauri 的 Message 类型转换为我们的 WebSocketMessage 类型
        let convertedMessage: WebSocketMessage;

        if (msg.type === 'Text') {
            convertedMessage = msg.data;
        } else if (msg.type === 'Binary') {
            convertedMessage = new Uint8Array(msg.data);
        } else {
            console.warn('Unknown message type:', msg.type);
            return;
        }

        setLastMessage(convertedMessage);
        callbacks?.onMessage?.(convertedMessage);
    }, [callbacks]);

    const handleError = useCallback((error: Event) => {
        console.error('WebSocket error:', error);
        callbacks?.onError?.(error);
    }, [callbacks]);

    const connect = useCallback(async () => {
        if (isConnected) return;

        try {
            const ws = await WebSocket.connect(url);

            ws.addListener(handleMessage);
            // Tauri WebSocket 插件可能需要通过事件监听错误
            // 根据实际API调整错误处理方式

            setIsConnected(true);
            setSocket(ws);
            callbacks?.onOpen?.();
        } catch (error) {
            handleError(error as Event);
            throw error;
        }
    }, [isConnected, url, handleMessage, callbacks, handleError]);

    const disconnect = useCallback(async () => {
        if (!socket) return;

        try {
            await socket.disconnect();
            setIsConnected(false);
            callbacks?.onClose?.();
        } catch (error) {
            handleError(error as Event);
        } finally {
            setSocket(null);
        }
    }, [socket, handleError, callbacks]);

    const sendMessage = useCallback(async (message: string | object) => {
        if (!socket || !isConnected) {
            throw new Error('WebSocket is not connected');
        }

        try {
            const payload = typeof message === 'object'
                ? JSON.stringify(message)
                : message;
            await socket.send(payload);
        } catch (error) {
            handleError(error as Event);
            throw error;
        }
    }, [socket, isConnected, handleError]);

    const queueMessage = useCallback((message: string | object) => {
        setMessageQueue(prev => [...prev, message]);
    }, []);

    // 自动处理消息队列
    useEffect(() => {
        if (isConnected && messageQueue.length > 0) {
            const processQueue = async () => {
                const failedMessages: Array<string | object> = [];

                for (const message of messageQueue) {
                    try {
                        await sendMessage(message);
                    } catch {
                        failedMessages.push(message);
                    }
                }

                setMessageQueue(failedMessages);
            };

            processQueue();
        }
    }, [isConnected, messageQueue, sendMessage]);

    // 自动连接和清理
    useEffect(() => {
        connect();

        return () => {
            disconnect();
        };
    }, [connect, disconnect]);

    return {
        socket,
        isConnected,
        lastMessage,
        connect,
        disconnect,
        sendMessage,
        queueMessage,
    };
}