import { useState, useEffect } from "react";
import { Button } from "../ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import { useTauriMobile } from "../../hooks/useTauriMobile";
import { cn } from "@/lib/utils";

interface NotificationComponentProps {
  className?: string;
}

export function NotificationComponent({
  className,
}: NotificationComponentProps) {
  const { sendMobileNotification } = useTauriMobile();
  const [notifications, setNotifications] = useState<
    Array<{
      id: string;
      title: string;
      body: string;
      timestamp: Date;
      read: boolean;
    }>
  >([]);
  const [permissionStatus, setPermissionStatus] = useState<
    "granted" | "denied" | "default"
  >("default");

  useEffect(() => {
    // Check notification permission on mount
    checkNotificationPermission();

    // Add some sample notifications
    setNotifications([
      {
        id: "1",
        title: "新订单提醒",
        body: "您有3个新的配送订单等待接单",
        timestamp: new Date(),
        read: false,
      },
      {
        id: "2",
        title: "配送提醒",
        body: "订单 #12345 预计15分钟后送达",
        timestamp: new Date(Date.now() - 30 * 60 * 1000),
        read: true,
      },
    ]);
  }, []);

  const checkNotificationPermission = async () => {
    try {
      // In a real Tauri app, this would check actual permissions
      const permission = Notification.permission;
      setPermissionStatus(permission);
    } catch (error) {
      console.error("Failed to check notification permission:", error);
    }
  };

  const requestNotificationPermission = async () => {
    try {
      const permission = await Notification.requestPermission();
      setPermissionStatus(permission);

      if (permission === "granted") {
        await sendMobileNotification(
          "通知权限已开启",
          "您将收到新订单和重要提醒",
        );
      }
    } catch (error) {
      console.error("Failed to request notification permission:", error);
    }
  };

  const sendTestNotification = async () => {
    await sendMobileNotification("测试通知", "这是一个测试通知消息");

    // Add to local notifications list
    const newNotification = {
      id: Date.now().toString(),
      title: "测试通知",
      body: "这是一个测试通知消息",
      timestamp: new Date(),
      read: false,
    };

    setNotifications((prev) => [newNotification, ...prev]);
  };

  const markAsRead = (id: string) => {
    setNotifications((prev) =>
      prev.map((notif) => (notif.id === id ? { ...notif, read: true } : notif)),
    );
  };

  const clearAllNotifications = () => {
    setNotifications([]);
  };

  const unreadCount = notifications.filter((n) => !n.read).length;

  const getPermissionStatusColor = () => {
    switch (permissionStatus) {
      case "granted":
        return "text-green-600 bg-green-50 border-green-200";
      case "denied":
        return "text-red-600 bg-red-50 border-red-200";
      default:
        return "text-yellow-600 bg-yellow-50 border-yellow-200";
    }
  };

  const getPermissionStatusText = () => {
    switch (permissionStatus) {
      case "granted":
        return "✅ 通知已允许";
      case "denied":
        return "❌ 通知被拒绝";
      default:
        return "⚠️ 需要权限";
    }
  };

  return (
    <Card className={cn("w-full", className)}>
      <CardHeader>
        <CardTitle className="flex items-center justify-between text-sm">
          <span className="flex items-center gap-2">
            🔔 通知中心
            {unreadCount > 0 && (
              <span className="bg-red-500 text-white text-xs px-2 py-1 rounded-full">
                {unreadCount}
              </span>
            )}
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Permission Status */}
        <div
          className={cn(
            "rounded-lg border p-3 text-center",
            getPermissionStatusColor(),
          )}
        >
          <p className="text-sm font-medium">{getPermissionStatusText()}</p>
        </div>

        {/* Permission Controls */}
        {permissionStatus !== "granted" && (
          <div className="space-y-2">
            <Button
              onClick={requestNotificationPermission}
              className="w-full bg-blue-600 hover:bg-blue-700"
              size="sm"
            >
              🔔 开启通知权限
            </Button>
            <p className="text-xs text-gray-500 text-center">
              开启通知以接收新订单和重要提醒
            </p>
          </div>
        )}

        {/* Test Notification */}
        {permissionStatus === "granted" && (
          <Button
            onClick={sendTestNotification}
            variant="outline"
            size="sm"
            className="w-full"
          >
            📨 发送测试通知
          </Button>
        )}

        {/* Notifications List */}
        {notifications.length > 0 && (
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <p className="text-sm font-medium">通知历史</p>
              <Button
                onClick={clearAllNotifications}
                variant="ghost"
                size="sm"
                className="text-xs h-6"
              >
                清空
              </Button>
            </div>

            <div className="space-y-2 max-h-48 overflow-y-auto">
              {notifications.map((notif) => (
                <div
                  key={notif.id}
                  className={cn(
                    "p-3 rounded-lg border cursor-pointer transition-colors",
                    notif.read
                      ? "bg-gray-50 border-gray-200"
                      : "bg-blue-50 border-blue-200",
                  )}
                  onClick={() => markAsRead(notif.id)}
                >
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <p
                        className={cn(
                          "text-sm",
                          notif.read
                            ? "text-gray-700"
                            : "font-medium text-blue-900",
                        )}
                      >
                        {notif.title}
                      </p>
                      <p className="text-xs text-gray-600 mt-1">{notif.body}</p>
                    </div>
                    {!notif.read && (
                      <div className="w-2 h-2 bg-blue-500 rounded-full mt-1 ml-2 flex-shrink-0" />
                    )}
                  </div>
                  <p className="text-xs text-gray-400 mt-2">
                    {notif.timestamp.toLocaleTimeString("zh-CN")}
                  </p>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Notification Settings */}
        <div className="text-xs text-gray-500 space-y-1">
          <p>• 新订单通知会自动推送</p>
          <p>• 重要配送提醒不可关闭</p>
          <p>• 可在系统设置中��整通知</p>
        </div>
      </CardContent>
    </Card>
  );
}
