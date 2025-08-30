import { useCallback, useContext, useEffect } from "react";
import * as Q from "@/api";

import type {
  Query,
  QueryData,
  QueryRequest,
  QueryResponse,
  QueryResult,
} from "@/types/query.ts";
import { QueryClient } from "@/utils/query_client.ts";
import { QueryContext } from "@/components/create-query/query-context.tsx";

export const useQuery = <T>(
  query: Query,
  req: any = undefined,
): QueryResult<T> => {
  const { queryResultCache } = useContext(QueryContext);
  const queryFn = useQueryTask(query);

  useEffect(() => {
    queryFn(req, emptyFn);
  }, [query]);

  return (
    queryResultCache.get(query) ?? {
      data: undefined as T,
      error: {} as Error,
      response: {} as QueryResponse<T>,
      isLoading: true,
      fn: undefined,
    }
  );
};

function emptyFn() {}

export const useQueryTask = <T extends Query, R>(
  query: T,
): ((data: QueryData<T>, fn: (data: R) => void) => QueryResult<R>) => {
  const { queryResultCache, setQueryResultCache } = useContext(QueryContext);

  const callback = useCallback(
    (data: QueryData<Query>, fn: (data: R) => void): QueryResult<R> => {
      // console.log(data)
      let queryReq = findQueryRequest(query);

      if (!queryReq || queryReq.symbol !== query) {
        const error = new Error(`Query data was not found!`);
        setQueryResultCache(query, {
          isLoading: false,
          data: undefined as R,
          response: {} as QueryResponse<T>,
          error,
        });
        throw error;
      }

      if (!!data) {
        queryReq = { ...queryReq, ...data };
      }

      let body;
      if (!!queryReq.body) {
        if (!!queryReq.normalize_req) {
          body = queryReq.normalize_req(queryReq.body);
        } else {
          body = queryReq.body;
        }
      }

      const urlParams = new URLSearchParams(
        queryReq.urlParams as Record<string, string>,
      );

      let isTemplatePath = false;
      if (queryReq.path.indexOf("}") >= 0) {
        isTemplatePath = true;
        if (!queryReq.pathParams) {
          throw new Error("Path Params is Empty! path: " + queryReq.path);
        }
      }

      new QueryClient()
        .setMethod(queryReq.method)
        .setParams(urlParams)
        .setBody(body)
        .setHeaders(queryReq.headers)
        .setPath(isTemplatePath ? "" : queryReq.path)
        .setPathWithTemplate(...[queryReq.path, queryReq.pathParams!])
        .query<T>()
        .then((res) => {
          let data = {};
          if (!!res.body && !!queryReq.normalize_res) {
            data = queryReq.normalize_res(res.body);
          }

          setQueryResultCache(query, {
            data,
            response: res,
            error: {} as Error,
            isLoading: false,
          });
        })
        .catch((error) => {
          setQueryResultCache(query, {
            data: undefined as R,
            response: {} as QueryResponse<R>,
            error,
            isLoading: false,
          });
        });
      return (
        queryResultCache.get(query) ?? {
          data: undefined as R,
          error: {} as Error,
          response: {} as QueryResponse<R>,
          isLoading: true,
          then: fn,
        }
      );
    },
    [query],
  );

  return callback;
};

const findQueryRequest = (query: Query): QueryRequest | void => {
  const key = query.description!;
  if (key in Q) {
    return Q[key as keyof typeof Q] as QueryRequest;
  }
};
