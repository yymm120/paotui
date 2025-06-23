import { Button } from "./ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";

interface DemoControlsProps {
  onAddRandomOrder: () => void;
  onSimulateNotification: () => void;
  onResetData: () => void;
  isVisible: boolean;
  onToggleVisibility: () => void;
}

export function DemoControls({
  onAddRandomOrder,
  onSimulateNotification,
  onResetData,
  isVisible,
  onToggleVisibility,
}: DemoControlsProps) {
  if (!isVisible) {
    return (
      <Button
        onClick={onToggleVisibility}
        className="fixed bottom-24 right-4 z-[9998] h-10 w-10 rounded-full bg-blue-600 p-0 text-white hover:bg-blue-700"
        size="icon"
      >
        ⚙️
      </Button>
    );
  }

  return (
    <Card className="fixed bottom-24 right-4 z-[9998] w-64 border border-gray-300 bg-white shadow-lg">
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm">Demo Controls</CardTitle>
          <Button
            onClick={onToggleVisibility}
            variant="ghost"
            size="sm"
            className="h-6 w-6 p-0"
          >
            ✕
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-2 pt-0">
        <Button
          onClick={onAddRandomOrder}
          variant="outline"
          size="sm"
          className="w-full text-xs"
        >
          📦 Add Random Order
        </Button>
        <Button
          onClick={onSimulateNotification}
          variant="outline"
          size="sm"
          className="w-full text-xs"
        >
          🔔 Simulate Notification
        </Button>
        <Button
          onClick={onResetData}
          variant="outline"
          size="sm"
          className="w-full text-xs"
        >
          🔄 Reset Data
        </Button>
        <div className="pt-2 text-[10px] text-gray-500">
          <p>• Toggle work status to see different states</p>
          <p>• Switch tabs to filter orders</p>
          <p>• Click filter to change sorting</p>
          <p>• Accept orders to move them to pickup</p>
        </div>
      </CardContent>
    </Card>
  );
}
