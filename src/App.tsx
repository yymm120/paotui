
import { useState, useCallback, useEffect } from "react";
import { DeliveryAppLayout } from "./components/delivery";
import { useDeliveryApp } from "./hooks/useDeliveryApp";
import { useTauriMobile } from "./hooks/useTauriMobile";
import "./App.css";
import {isTauri} from "@tauri-apps/api/core";
import {MobileSidebar} from "@/components/delivery/MobileSidebar.tsx";
import {LoginPage} from "@/components/page/LoginPage.tsx";
// import { useTauriWebSocket } from './hooks/useTauriWebSocket';
// import type { WebSocketMessage } from './types';

// const formatMessage = (msg: WebSocketMessage): string => {
//   if (typeof msg === 'string') return msg;
//   if (msg instanceof ArrayBuffer) return '[ArrayBuffer]';
//   if (msg instanceof Blob) return '[Blob]';
//   return JSON.stringify(msg);
// };


function App() {
  const [showMobileFeatures, setShowMobileFeatures] = useState(false);
  const [isLogin, setLogin] = useState(false);

  // const {
  //   isConnected,
  //   lastMessage,
  //   sendMessage
  // } = useTauriWebSocket('ws://192.168.10.107:8080', {
  //   onMessage: (msg) => console.log('Received:', msg),
  //   onOpen: () => console.log('WebSocket connected'),
  //   onClose: () => console.log('WebSocket disconnected'),
  //   onError: (error) => console.error('WebSocket error:', error),
  // });

  // const handleSendMessage = useCallback(async () => {
  //   try {
  //     await sendMessage({
  //       type: 'greeting',
  //       content: 'Hello from React!',
  //       timestamp: new Date().toISOString()
  //     });
  //   } catch (error) {
  //     console.error('Failed to send message:', error);
  //   }
  // }, [sendMessage]);
  // 检查是否在Tauri环境
  const isMobileApp = isTauri()

  const deliveryApp = useDeliveryApp();



  const mobileApp = useTauriMobile();

  // Use web app data as primary source (working solution)
  const {orders, isWorking, handleAcceptOrder, handleToggleWork} = deliveryApp;

  // Mobile-specific features (optional)
  const currentLocation = mobileApp?.currentLocation || null;
  // const sendMobileNotification = mobileApp?.sendMobileNotification || null;

  // Mobile-specific handlers
  const handleMenuClick = useCallback(() => {
    setShowMobileFeatures(!showMobileFeatures);
    console.log("Menu clicked - toggling mobile features panel");
  }, [showMobileFeatures]);



  const handleToggleWorkWrapper = useCallback(async () => {
    // Use the working web app function
    handleToggleWork();

    if (isMobileApp) {
      try {
        const newStatus = !isWorking;
        // await sendMobileNotification(
        //   newStatus ? "开始工作" : "结束工作",
        //   newStatus ? "您已开始接收新订单" : "您已停止接收新订单",
        // );
      } catch (error) {
        console.error("Failed to send mobile notification:", error);
      }
    }
  }, [handleToggleWork, isWorking, isMobileApp]);

  const handleAcceptOrderWrapper = useCallback(async (order_id: string) => {
    handleAcceptOrder(order_id);
  }, [])


  // Show order counts in console for debugging
  useEffect(() => {
    console.log("📱 Platform:", isMobileApp ? "Tauri Mobile" : "Web Browser");
    console.log("👷 Working Status:", isWorking ? "Working" : "Not Working");
    console.log("📦 Orders in current tab:", orders.length);
    if (currentLocation) {
      console.log(
        "📍 Current Location:",
        currentLocation.latitude,
        currentLocation.longitude,
      );
    }
  }, [isMobileApp, isWorking, orders.length, currentLocation]);

  // Welcome message
  useEffect(() => {
    console.log("🚚 Welcome to the Paotui Delivery Mobile App!");
    console.log("💡 Platform:", isMobileApp ? "Tauri Mobile" : "Web Browser");
    if (isMobileApp) {
      console.log("📱 Mobile Features Available:");
      console.log("  • Real-time GPS location tracking");
      console.log("  • Camera for delivery photos");
      console.log("  • Push notifications");
      console.log("  • Offline data storage");
    }
  }, [isMobileApp]);

  return (
    <div className="min-h-screen bg-gray-100">
      {!isLogin ? (
          <LoginPage setLogin={setLogin} />
      ): (
          <DeliveryAppLayout
              // State props - use working web app data
              activeTab={deliveryApp.activeTab}
              isWorking={deliveryApp.isWorking}
              status={deliveryApp.isWorking ? "开工" : "已收工"}
              filterLabel={deliveryApp.currentFilterLabel}
              orders={deliveryApp.orders}
              // Event handlers - use working web app handlers
              onMenuClick={handleMenuClick}
              onStatusClick={deliveryApp.handleStatusClick}
              // onNotificationClick={handleNotificationClick}
              onTabChange={deliveryApp.handleTabChange}
              onFilterClick={deliveryApp.handleFilterChange}
              onAcceptOrder={handleAcceptOrderWrapper}
              onToggleWork={handleToggleWorkWrapper}
              // onSettingsClick={handleSettingsClick}
          />
      )}

      {/*<div className="websocket-container">*/}
      {/*  <h2>WebSocket Status:*/}
      {/*    <span className={isConnected ? 'connected' : 'disconnected'}>*/}
      {/*    {isConnected ? 'Connected' : 'Disconnected'}*/}
      {/*  </span>*/}
      {/*  </h2>*/}

      {/*  <button*/}
      {/*      onClick={handleSendMessage}*/}
      {/*      disabled={!isConnected}*/}
      {/*      aria-label="Send WebSocket message"*/}
      {/*  >*/}
      {/*    Send Message*/}
      {/*  </button>*/}

      {/*  <div className="message-display">*/}
      {/*    <h3>Last Message:</h3>*/}
      {/*    <pre>{lastMessage ? formatMessage(lastMessage) : 'No messages received'}</pre>*/}
      {/*  </div>*/}
      {/*</div>*/}

      {/* Mobile Features Panel */}
      {isMobileApp && showMobileFeatures && (
          <MobileSidebar />
      )}

      {/* Settings Panel */}
      {/*{showSettings && (*/}
      {/*  <div className="fixed inset-0 z-[9999] bg-black/50 backdrop-blur-sm">*/}
      {/*    <div className="absolute right-0 top-0 h-full w-80 bg-white shadow-xl overflow-y-auto">*/}
      {/*      <div className="p-4">*/}
      {/*        <SettingsComponent onClose={() => setShowSettings(false)} />*/}
      {/*      </div>*/}
      {/*    </div>*/}
      {/*  </div>*/}
      {/*)}*/}
    </div>
  );
}

export default App;
