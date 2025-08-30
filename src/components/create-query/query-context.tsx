import { createContext, type ReactElement, useState } from "react";
import type { Query } from "@/types/query.ts";

type QueryContextType = {
  queryResultCache: Map<Query, any>;
  setQueryResultCache: (query: Query, result: any) => void;
};

export const QueryContext = createContext<QueryContextType>({
  queryResultCache: new Map<Query, any>(),
  setQueryResultCache: () => {
    throw new Error("不能在QueryContext组件之外使用, 该组件尚未初始化.");
  },
});

interface QueryProviderProps {
  children: ReactElement;
  queryResultCache?: Map<Query, any>;
}

export function QueryContextProvider({
  children,
  queryResultCache: cache,
}: QueryProviderProps) {
  const [queryResultCache, setQueryResultCacheDispatcher] = useState<
    Map<Query, any>
  >(cache ?? new Map());

  const setQueryResultCache = (query: Query, result: any) => {
    setQueryResultCacheDispatcher((prevState) =>
      new Map(prevState).set(query, result),
    );
  };

  return (
    <QueryContext.Provider value={{ queryResultCache, setQueryResultCache }}>
      {children}
    </QueryContext.Provider>
  );
}
