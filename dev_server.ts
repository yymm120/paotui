import { serve } from "bun";
// import homepage from "./index.html";   // Preview
import homepage from "./src/index.html"; // App
import {mockDeliveryOrders} from "./src/data/mockDeliveryOrders";

const server = serve({
  fetch(req, server) {

    if (req.url.endsWith("ws")) {
      const success = server.upgrade(req);
      if (success) {
        return undefined;
      }
      return new Response("Upgrade Websocket Failed!")
    }
  },
  websocket: {
    // QuerySocket 事件处理器
    open(ws) {
      console.log(`Client connected!`);
      ws.subscribe("task-updates"); // 加入频道
    },

    message(ws, message) {
      console.log(`Received: ${message}`);
      // 1. 每次task状态改变, 向所有客户端发送
      //    客户端根据id, 删除相应的task项
      // 2. 当有新的task时, 向所有客户端发送
      //    客户端根据id, 增加相应的task项
      // 3. 凡是连接了websocket的用户都有一个计时器, 根据频率向客户端发送最新数据
      //    客户端可以手动向服务端发起刷新
      // 4. 客户端打开工作中状态, 则意味着启动websocket连接
      // 5. 客户端关闭工作中状态, 则意味着关闭websocket连接
      //    连接期间, 主要由服务端向客户端推送消息, 因为客户端不知道哪个数据何时失效

      // 示例：广播消息给所有客户端
      server.publish("task-updates", `Echo: ${message}`);

      // 或单独回复
      // ws.send(`You said: ${message}`);
    },

    close(ws) {
      console.log("Client disconnected");
      ws.unsubscribe("task-updates");
    },

    drain(ws) {
      console.log("Backpressure released");
    }
  },
  routes: {
    "/": homepage,
    "/api/task": {
      async GET(req) {
        return Response.json(mockDeliveryOrders);
      },
    }
  },

  hostname: "192.168.10.107",
  port: 5173,
  development: {
    hmr: true,
    console: true,
  },

});
console.log(`Listening on ${server.url}`);
