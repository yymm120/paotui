import React, { useState, useEffect } from "react";
import { Button } from "../ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import {
  useTauriMobile,
  type DeliverySettings,
} from "../../hooks/useTauriMobile";
import { isWithinWorkingHours, getBatteryLevel } from "../../utils/mobile";
import { cn } from "@/lib/utils";

interface SettingsComponentProps {
  className?: string;
  onClose?: () => void;
}

export function SettingsComponent({
  className,
  onClose,
}: SettingsComponentProps) {
  const { settings, saveSettings, exportData } = useTauriMobile();
  const [localSettings, setLocalSettings] = useState<DeliverySettings | null>(
    null,
  );
  const [batteryLevel, setBatteryLevel] = useState<number>(100);
  const [isWithinHours, setIsWithinHours] = useState(false);

  useEffect(() => {
    if (settings) {
      setLocalSettings(settings);
      setIsWithinHours(
        isWithinWorkingHours(
          settings.working_hours_start,
          settings.working_hours_end,
        ),
      );
    }
  }, [settings]);

  useEffect(() => {
    getBatteryLevel().then(setBatteryLevel);
  }, []);

  const handleSaveSettings = async () => {
    if (localSettings) {
      await saveSettings(localSettings);
      onClose?.();
    }
  };

  const handleResetSettings = () => {
    const defaultSettings: DeliverySettings = {
      auto_accept_orders: false,
      min_order_value: 10.0,
      max_delivery_distance: 5000.0,
      sound_notifications: true,
      working_hours_start: "08:00",
      working_hours_end: "22:00",
    };
    setLocalSettings(defaultSettings);
  };

  const updateSetting = <K extends keyof DeliverySettings>(
    key: K,
    value: DeliverySettings[K],
  ) => {
    if (localSettings) {
      setLocalSettings((prev) => (prev ? { ...prev, [key]: value } : null));
    }
  };

  if (!localSettings) {
    return (
      <Card className={cn("w-full", className)}>
        <CardContent className="flex items-center justify-center h-32">
          <p className="text-sm text-gray-500">加载设置中...</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={cn("w-full", className)}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="text-base">⚙️ 接单设置</CardTitle>
          {onClose && (
            <Button onClick={onClose} variant="ghost" size="sm">
              ✕
            </Button>
          )}
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Auto Accept Orders */}
        <div className="space-y-2">
          <label className="text-sm font-medium">自动接单</label>
          <div className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={localSettings.auto_accept_orders}
              onChange={(e) =>
                updateSetting("auto_accept_orders", e.target.checked)
              }
              className="rounded"
            />
            <span className="text-sm text-gray-600">
              自动接受符合条件的订单
            </span>
          </div>
        </div>

        {/* Minimum Order Value */}
        <div className="space-y-2">
          <label className="text-sm font-medium">最低订单金额 (¥)</label>
          <input
            type="number"
            value={localSettings.min_order_value}
            onChange={(e) =>
              updateSetting("min_order_value", parseFloat(e.target.value) || 0)
            }
            className="w-full p-2 border rounded-lg text-sm"
            min="0"
            step="0.5"
          />
          <p className="text-xs text-gray-500">低于此金额的订单不会自动接受</p>
        </div>

        {/* Maximum Delivery Distance */}
        <div className="space-y-2">
          <label className="text-sm font-medium">最大配送距离 (米)</label>
          <input
            type="number"
            value={localSettings.max_delivery_distance}
            onChange={(e) =>
              updateSetting(
                "max_delivery_distance",
                parseFloat(e.target.value) || 0,
              )
            }
            className="w-full p-2 border rounded-lg text-sm"
            min="0"
            step="100"
          />
          <p className="text-xs text-gray-500">超过此距离的订单不会自动接受</p>
        </div>

        {/* Working Hours */}
        <div className="space-y-3">
          <label className="text-sm font-medium">工作时间</label>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="text-xs text-gray-500">开始时间</label>
              <input
                type="time"
                value={localSettings.working_hours_start}
                onChange={(e) =>
                  updateSetting("working_hours_start", e.target.value)
                }
                className="w-full p-2 border rounded-lg text-sm"
              />
            </div>
            <div>
              <label className="text-xs text-gray-500">结束时间</label>
              <input
                type="time"
                value={localSettings.working_hours_end}
                onChange={(e) =>
                  updateSetting("working_hours_end", e.target.value)
                }
                className="w-full p-2 border rounded-lg text-sm"
              />
            </div>
          </div>
          <div
            className={cn(
              "text-xs p-2 rounded border",
              isWithinHours
                ? "text-green-700 bg-green-50 border-green-200"
                : "text-orange-700 bg-orange-50 border-orange-200",
            )}
          >
            {isWithinHours ? "✅ 当前在工作时间内" : "⏰ 当前不在工作时间内"}
          </div>
        </div>

        {/* Sound Notifications */}
        <div className="space-y-2">
          <label className="text-sm font-medium">声音通知</label>
          <div className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={localSettings.sound_notifications}
              onChange={(e) =>
                updateSetting("sound_notifications", e.target.checked)
              }
              className="rounded"
            />
            <span className="text-sm text-gray-600">新订单时播放提示音</span>
          </div>
        </div>

        {/* System Info */}
        <div className="space-y-2">
          <label className="text-sm font-medium">系统信息</label>
          <div className="bg-gray-50 rounded-lg p-3 space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-gray-600">电池电量</span>
              <span
                className={cn(
                  "font-medium",
                  batteryLevel > 20 ? "text-green-600" : "text-red-600",
                )}
              >
                {Math.round(batteryLevel)}%
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-600">网络状态</span>
              <span className="text-green-600 font-medium">
                {navigator.onLine ? "已连接" : "已断开"}
              </span>
            </div>
          </div>
        </div>

        {/* Action Buttons */}
        <div className="space-y-3">
          <div className="grid grid-cols-2 gap-3">
            <Button
              onClick={handleSaveSettings}
              className="bg-green-600 hover:bg-green-700"
            >
              💾 保存设置
            </Button>
            <Button onClick={handleResetSettings} variant="outline">
              🔄 重置默认
            </Button>
          </div>

          <Button onClick={exportData} variant="outline" className="w-full">
            📤 导出数据备份
          </Button>
        </div>

        {/* Tips */}
        <div className="text-xs text-gray-500 space-y-1 border-t pt-3">
          <p>
            💡 <strong>提示：</strong>
          </p>
          <p>• 开启自动接单可提高收入效率</p>
          <p>• 合理设置配送距离避免过度疲劳</p>
          <p>• 定期备份数据防止丢失</p>
          <p>• 保持设备电量充足以确保正常工作</p>
        </div>
      </CardContent>
    </Card>
  );
}
