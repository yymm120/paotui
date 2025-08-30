import { type ReactNode, type TouchEvent, useState } from "react";
import { cn } from "@/lib/utils.ts";

export function PullToRefresh({
  children,
  onRefresh,
  className,
}: {
  children: ReactNode;
  onRefresh: () => Promise<void>;
  className?: string;
}) {
  const [refreshing, setRefreshing] = useState(false);
  const [startY, setStartY] = useState<number | null>(null);
  const [pullDistance, setPullDistance] = useState(0);
  const [isPulling, setIsPulling] = useState(false);

  const handleTouchStart = (e: TouchEvent) => {
    if (window.scrollY === 0) {
      setStartY(e.touches[0]!.clientY);
      setIsPulling(true);
    }
  };

  const handleTouchMove = (e: TouchEvent) => {
    if (startY === null || !isPulling) return;

    const y = e.touches[0]!.clientY;
    const diff = y - startY;

    // 只允许下拉，不允许上推
    if (diff > 0) {
      setPullDistance(diff);
    }
  };

  const handleTouchEnd = async () => {
    if (pullDistance > 50 && !refreshing) {
      // 下拉超过50px且松开时触发
      setRefreshing(true);
      try {
        await onRefresh();
      } finally {
        setRefreshing(false);
      }
    }
    // 重置状态
    setStartY(null);
    setPullDistance(0);
    setIsPulling(false);
  };

  // 计算下拉比例（用于动画效果）
  const pullRatio = Math.min(pullDistance / 50, 1);

  return (
    <div
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
      style={{ position: "relative" }}
      className={cn(className)}
    >
      {/* 下拉提示区域 */}
      <div
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          right: 0,
          height: `${Math.min(pullDistance, 50)}px`,
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          transform: `translateY(${-50 + Math.min(pullDistance, 50)}px)`,
          transition: refreshing ? "none" : "transform 0.2s ease",
          background: "#f5f5f5",
          opacity: pullRatio,
        }}
      >
        {refreshing ? "刷新中..." : "下拉刷新"}
      </div>

      {/* 内容区域 */}
      <div
        style={{
          transform: refreshing
            ? "translateY(60px)"
            : `translateY(${Math.min(pullDistance, 50)}px)`,
          transition: refreshing
            ? "transform 0.3s ease"
            : "transform 0.2s ease",
        }}
      >
        {children}
      </div>
    </div>
  );
}
