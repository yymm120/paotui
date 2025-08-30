import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
// @ts-ignore
import "./index.css";
// import { NavigationTabs } from "@/components/delivery";
import "./api";
// import { QueryContextProvider } from "@/components/create-query/query-context.tsx";
import App from "@/App.tsx";
import { TanstackQuery } from "@/components/tanstack-query/tanstack-query.tsx";
import { AppContextProvider } from "@/components/app-context.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    {/*<QueryContextProvider>*/}
    <TanstackQuery>
      <AppContextProvider>
        <App />
      </AppContextProvider>
    </TanstackQuery>
    {/*</QueryContextProvider>*/}
  </StrictMode>,
);
