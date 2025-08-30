import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
// @ts-ignore
import "./index.css";
// import { NavigationTabs } from "@/components/delivery";
import "./api";
import { QueryContextProvider } from "@/components/create-query/query-context.tsx";
// import { FormOrderDelivery } from "@/components/form-order-delivery/form-order-delivery.tsx";
// import {DeliveryCard} from "@/components/delivery";
// import type {DeliveryTask} from "@/types";
// import {mockDeliveryOrders} from "@/data/mockDeliveryOrders.ts";
import { ChatApp } from "@/components/demo/chat-app.tsx";
import { ProfileContextProvider } from "@/components/profile-context-provider.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryContextProvider>
      <ProfileContextProvider>
        <div className={"flex items-center justify-center"}>
          {/*<NavigationTabs />*/}
          {/*<DeliveryCard task={mockDeliveryOrders[0]!} />*/}
          <ChatApp />
        </div>
      </ProfileContextProvider>
    </QueryContextProvider>
  </StrictMode>,
);
