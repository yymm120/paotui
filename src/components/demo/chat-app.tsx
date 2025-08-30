import { useWebSocket } from "@/hooks/use-websocket";
import { useState } from "react";
import { Button } from "@/components/ui/button.tsx";
import { Input } from "@/components/ui/input.tsx";

export function ChatApp() {
  const [messages] = useState<string[]>([]);
  const [inputValue, setInputValue] = useState("");

  const {} = useWebSocket("wss://192.168.10.107:4000/ws", ["task"]);

  // 添加消息处理器
  // useEffect(() => {
  //   const cleanup = addMessageHandler((event) => {
  //     setMessages(prev => [...prev, event.data]);
  //   });
  //   return cleanup as () => void;
  // }, [addMessageHandler]);

  // const handleSend = () => {
  //   if (inputValue.trim()) {
  //     sendMessage(inputValue);
  //     setInputValue('');
  //   }
  // };

  return (
    <div>
      <div>Status: {"Connected"}</div>
      <div className="message-list">
        {messages.map((msg, i) => (
          <div key={i}>{msg}</div>
        ))}
      </div>
      <Input
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
      />
      <Button type={"button"} className={"cursor-pointer"}>
        Send
      </Button>
    </div>
  );
}
