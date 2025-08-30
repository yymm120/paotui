import { createContext, useState } from "react";
import { DeliveryAppLayout } from "./components/delivery";
// import { useTauriMobile } from "./hooks/useTauriMobile";
// @ts-ignore
import "./App.css";
import { isTauri } from "@tauri-apps/api/core";
import { MobileSidebar } from "@/components/delivery/mobile-sidebar.tsx";

import { LoginPage } from "@/components/page/LoginPage.tsx";
import { useInitial } from "@/hooks/query/use-initial.ts";
import { useGpsLocation } from "@/hooks/use-gps-location.ts";
import { Toaster } from "@/components/ui/sonner.tsx";
import { Button } from "@/components/ui/button.tsx";

function App() {
  const [showMobileFeatures] = useState(false);

  const isMobileApp = isTauri();

  const { isLoading, isLogin, setIsLogin } = useInitial();

  useGpsLocation();

  const handleClick = () => {
    console.log(isTauri());
  };

  return (
    <div className="bg-gray-100 w-full h-screen">
      <Toaster position={"top-center"} />
      {isLoading ? (
        <span>Loading</span>
      ) : !isLogin ? (
        <LoginPage setLogin={setIsLogin} />
      ) : (
        <DeliveryAppLayout className={"h-full"} />
      )}
      {isMobileApp && showMobileFeatures && <MobileSidebar />}
    </div>
  );
}

export default App;
