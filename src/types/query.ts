import type * as Q from "@/api";
import type { SomeZodObject } from "zod";

export type HttpMethod = "POST" | "GET" | "PATCH" | "DELETE" | "UPDATE";
export type HttpStatus = string | number;

export type Query = {
  [K in keyof typeof Q]: (typeof Q)[K] extends symbol ? (typeof Q)[K] : never;
}[keyof typeof Q];

export type QueryData<T extends symbol> = Omit<
  {
    [K in keyof typeof Q]: (typeof Q)[K] & {} extends { symbol: T }
      ? (typeof Q)[K]
      : never;
  }[keyof typeof Q],
  | "path"
  | "method"
  | "symbol"
  | "headers"
  | "cookies"
  | "normalize_res"
  | "normalize_req"
>;

export type QueryRequest = {
  symbol: Query;
  path: string;
  method: HttpMethod;
  body?: Record<string, any>;
  pathParams?: Record<string, string>;
  urlParams?:
    | string
    | URLSearchParams
    | Record<string, string>
    | Map<string, string>;
  headers?: string | Headers | Record<string, string> | Map<string, string>;
  // cookies?: string | Record<string, string>,
  zodSchema?: SomeZodObject;
  normalize_req?: (args: any) => any;
  normalize_res?: (args: any) => any;
};

export interface QueryResult<T = any> {
  isLoading: boolean;
  response: QueryResponse<T>;
  data: T;
  error: Error;
}

export interface QueryResponse<T> {
  originResponse: Response;
  url: URL;
  headers: Headers;
  status: HttpStatus;
  cookies: Map<string, string>;
  content_length: number;
  remote_addr: string;
  body: T;
}

// type ExtractBodyType<T> = T extends { body: infer B } ? B : never;
//
// // 获取所有已注册的 QueryData 类型
// type RegisteredQueries = {
//   [K in keyof typeof Q]: typeof Q[K] extends QueryDataB<infer S>
//     ? { symbol: S; body: ExtractBodyType<typeof Q[K]> }
//     : never;
// }[keyof typeof Q];
//
// // 最终参数类型
// type QueryPayload<T extends symbol> = Extract<
//   RegisteredQueries,
//   { symbol: T }
// >["body"];

// const UpdateTaskQuery: QueryDataB<typeof Query_update_delivery_task> = {
//   path: "/api/task",
//   method: "POST",
//   headers: { "Content-Type": "application/json" },
//   query: Query_update_delivery_task,
//   body: { id: "", status: "" } // 这里定义 body 结构
// } as const; // ⭐️ 关键：as const 触发字面量类型推断
//
//
// export function useQueryTask<T extends symbol>(
//   querySymbol: T
// ): (payload: QueryPayload<T>) => void {
//   return (payload) => {
//     // 这里 payload 已经是正确类型
//     console.log("Sending:", payload);
//   };
// }
//
// const abcc = useQueryTask(Query_update_delivery_task)
// abcc({ path: "/api/task", method: "POST" })
