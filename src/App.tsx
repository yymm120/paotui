import React from "react";
import { DeliveryAppLayout } from "./components/delivery";
import { DemoControls } from "./components/DemoControls";
import { CameraComponent } from "./components/mobile/CameraComponent";
import { LocationComponent } from "./components/mobile/LocationComponent";
import { NotificationComponent } from "./components/mobile/NotificationComponent";
import { SettingsComponent } from "./components/mobile/SettingsComponent";
import { useDeliveryApp } from "./hooks/useDeliveryApp";
import { useTauriMobile } from "./hooks/useTauriMobile";
import { mockOrders } from "./data/mockOrders";
import "./App.css";

function App() {
  const [showDemoControls, setShowDemoControls] = React.useState(false);
  const [showMobileFeatures, setShowMobileFeatures] = React.useState(false);
  const [showSettings, setShowSettings] = React.useState(false);
  const [selectedOrderForPhoto, setSelectedOrderForPhoto] = React.useState<
    string | null
  >(null);

  // Check if running in mobile environment (Tauri)
  const isMobileApp =
    typeof window !== "undefined" && (window as any).__TAURI__ !== undefined;

  // Use web hooks as primary (they work and are tested)
  const webApp = useDeliveryApp();

  // Use mobile hooks only if in Tauri environment
  const mobileApp = isMobileApp ? useTauriMobile() : null;

  // Use web app data as primary source (working solution)
  const orders = webApp.orders;
  const isWorking = webApp.isWorking;
  const acceptOrder = webApp.handleAcceptOrder;
  const toggleWork = webApp.handleToggleWork;

  // Mobile-specific features (optional)
  const currentLocation = mobileApp?.currentLocation || null;
  const sendMobileNotification = mobileApp?.sendMobileNotification || null;

  // Mobile-specific handlers
  const handleMenuClick = React.useCallback(() => {
    setShowMobileFeatures(!showMobileFeatures);
    console.log("Menu clicked - toggling mobile features panel");
  }, [showMobileFeatures]);

  const handleSettingsClick = React.useCallback(() => {
    setShowSettings(!showSettings);
    console.log("Settings clicked");
  }, [showSettings]);

  const handleNotificationClick = React.useCallback(async () => {
    if (isMobileApp && sendMobileNotification) {
      try {
        await sendMobileNotification("通知测试", "这是一个测试通知");
      } catch (error) {
        console.error("Failed to send mobile notification:", error);
      }
    }
    // Use the existing web app handler
    webApp.handleNotificationClick();
  }, [isMobileApp, sendMobileNotification, webApp.handleNotificationClick]);

  const handleAcceptOrderWrapper = React.useCallback(
    async (orderId: string) => {
      // Use the working web app function
      acceptOrder(orderId);

      if (isMobileApp && sendMobileNotification) {
        try {
          await sendMobileNotification(
            "订单已接受",
            `订单 ${orderId.slice(-8)} 已成功接受`,
          );
        } catch (error) {
          console.error("Failed to send mobile notification:", error);
        }
      }

      console.log("✅ Order accepted:", orderId);
    },
    [acceptOrder, isMobileApp, sendMobileNotification],
  );

  const handleToggleWorkWrapper = React.useCallback(async () => {
    // Use the working web app function
    toggleWork();

    if (isMobileApp && sendMobileNotification) {
      try {
        const newStatus = !isWorking;
        await sendMobileNotification(
          newStatus ? "开始工作" : "结束工作",
          newStatus ? "您已开始接收新订单" : "您已停止接收新订单",
        );
      } catch (error) {
        console.error("Failed to send mobile notification:", error);
      }
    }
  }, [toggleWork, isWorking, isMobileApp, sendMobileNotification]);

  // Demo control handlers (for web version)
  const handleAddRandomOrder = React.useCallback(() => {
    const randomOrder =
      mockOrders[Math.floor(Math.random() * mockOrders.length)];
    const newOrder = {
      ...randomOrder,
      orderTime: new Date().toLocaleTimeString("zh-CN", {
        hour: "2-digit",
        minute: "2-digit",
      }),
      estimatedEarnings: Math.floor(Math.random() * 30) + 8,
    };
    webApp.addNewOrder(newOrder);
    console.log("✅ Added random order:", newOrder.fromStore);
  }, [webApp]);

  const handleSimulateNotification = React.useCallback(async () => {
    if (isMobileApp && sendMobileNotification) {
      try {
        await sendMobileNotification("测试通知", "这是一个移��端测试通知");
      } catch (error) {
        console.error("Failed to send mobile notification:", error);
      }
    } else {
      console.log("🔔 Simulated notification (web mode)");
    }
  }, [isMobileApp, sendMobileNotification]);

  const handleResetData = React.useCallback(() => {
    console.log("🔄 Data reset - refresh page to see original data");
    window.location.reload();
  }, []);

  // Show order counts in console for debugging
  React.useEffect(() => {
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
  React.useEffect(() => {
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
      <DeliveryAppLayout
        // State props - use working web app data
        activeTab={webApp.activeTab}
        isWorking={webApp.isWorking}
        status={webApp.userStatus}
        filterLabel={webApp.currentFilterLabel}
        orders={webApp.orders}
        // Event handlers - use working web app handlers
        onMenuClick={handleMenuClick}
        onStatusClick={webApp.handleStatusClick}
        onNotificationClick={handleNotificationClick}
        onTabChange={webApp.handleTabChange}
        onFilterClick={webApp.handleFilterChange}
        onAcceptOrder={handleAcceptOrderWrapper}
        onToggleWork={handleToggleWorkWrapper}
        onSettingsClick={handleSettingsClick}
      />

      {/* Mobile Features Panel */}
      {isMobileApp && showMobileFeatures && (
        <div className="fixed inset-0 z-[9999] bg-black/50 backdrop-blur-sm">
          <div className="absolute right-0 top-0 h-full w-80 bg-white shadow-xl overflow-y-auto">
            <div className="p-4 space-y-4">
              <div className="flex items-center justify-between">
                <h2 className="text-lg font-bold">📱 移动功能</h2>
                <button
                  onClick={() => setShowMobileFeatures(false)}
                  className="text-gray-500 hover:text-gray-700"
                >
                  ✕
                </button>
              </div>

              <LocationComponent />
              <NotificationComponent />

              {selectedOrderForPhoto && (
                <CameraComponent
                  orderId={selectedOrderForPhoto}
                  onPhotoTaken={(path) => {
                    console.log("Photo taken:", path);
                    setSelectedOrderForPhoto(null);
                  }}
                />
              )}

              <button
                onClick={() => setSelectedOrderForPhoto("demo-order-123")}
                className="w-full p-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
              >
                📷 演示拍照功能
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Settings Panel */}
      {showSettings && (
        <div className="fixed inset-0 z-[9999] bg-black/50 backdrop-blur-sm">
          <div className="absolute right-0 top-0 h-full w-80 bg-white shadow-xl overflow-y-auto">
            <div className="p-4">
              <SettingsComponent onClose={() => setShowSettings(false)} />
            </div>
          </div>
        </div>
      )}

      {/* Demo Controls */}
      <DemoControls
        isVisible={showDemoControls}
        onToggleVisibility={() => setShowDemoControls(!showDemoControls)}
        onAddRandomOrder={handleAddRandomOrder}
        onSimulateNotification={handleSimulateNotification}
        onResetData={handleResetData}
      />

      {/* Development Info Panel */}
      {process.env.NODE_ENV === "development" && (
        <div className="fixed top-4 right-4 z-[9999] rounded-lg bg-black/80 p-3 text-xs text-white backdrop-blur-sm">
          <div className="space-y-1">
            <div>📱 Tab: {webApp.activeTab}</div>
            <div>👷 Status: {webApp.isWorking ? "工作中" : "已收工"}</div>
            <div>📦 Orders: {webApp.orders.length}</div>
            <div>🔔 Notifications: {webApp.unreadNotificationsCount}</div>
            <div className="mt-2 text-[10px] text-gray-300">
              New: {webApp.ordersCounts.newTasks} | Pickup:{" "}
              {webApp.ordersCounts.pickup} | Delivery:{" "}
              {webApp.ordersCounts.delivery}
            </div>
            <div className="mt-1 text-[10px] text-blue-300">
              Filter: {webApp.currentFilterLabel}
            </div>
            {isMobileApp && (
              <div className="mt-1 text-[10px] text-green-300">
                🚀 Tauri Mobile Ready
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
