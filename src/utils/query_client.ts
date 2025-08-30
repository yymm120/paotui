import { isTauri } from "@tauri-apps/api/core";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";
import type { HttpMethod, QueryResponse } from "@/types/query.ts";
import { parseTemplate } from "url-template";
import urlJoin from "url-join";

export class QueryClient<T = Record<string, string>> {
  // public originQuery = isTauri() ? tauriFetch : window.fetch;
  private baseUrl: URL = new URL("https://192.168.10.107:4000");
  private path: string = "";
  private urlParams: URLSearchParams = new URLSearchParams();
  private method: HttpMethod = "GET";
  private headers: Headers = new Headers({
    "Access-Control-Allow-Origin": "*",
  });
  private body?: T = undefined;
  public query = async <R>(
    path_or_url: string | URL = "",
  ): Promise<QueryResponse<R>> => {
    let url =
      URL.parse(path_or_url) ?? (URL.parse(path_or_url, this.baseUrl) as URL);

    let path = "";
    if (!!url.pathname && url.pathname !== "/") {
      path = urlJoin(path, url.pathname);
    }
    if (!!this.path) {
      path = urlJoin(path, this.path);
    }
    url = URL.parse(path, url?.origin ?? this.baseUrl) as URL;
    if (!url) {
      console.error("URL 无法解析: ", this.baseUrl, url);
    }

    this.urlParams.forEach(
      (name, value) =>
        !url.searchParams.get(name) && url.searchParams.append(name, value),
    );

    let res;

    if (!isTauri() && !!window.fetch) {
      res = await fetch(url, {
        method: this.method,
        headers: this.headers,
        body: !!this.body ? JSON.stringify(this.body) : null,
      });
    } else {
      res = await tauriFetch(url, {
        method: this.method,
        headers: this.headers,
        body: !!this.body ? JSON.stringify(this.body) : null,
      });
    }
    const body = (await res.json()) as R;

    const queryResponse: QueryResponse<R> = {
      originResponse: res,
      content_length: 0,
      cookies: new Map(),
      headers: new Headers(),
      remote_addr: this.baseUrl.host,
      status: "200",
      url: url as URL,
      body,
    };
    return queryResponse;
  };

  public setMethod(method: HttpMethod) {
    this.method = method;
    return this;
  }

  public setHeaders(
    headers:
      | string
      | Headers
      | Record<string, string>
      | Map<string, string>
      | undefined,
  ) {
    if (typeof headers === "string") {
      Object.entries(JSON.parse(headers)).forEach(([name, value]) =>
        this.headers.append(name, value as string),
      );
    } else if (typeof headers !== "undefined") {
      Object.entries(headers).forEach(([name, value]) => {
        this.headers.append(name, value);
      });
    }
    return this;
  }

  public setHeader(name: string, value: string) {
    this.headers.append(name, value);
    return this;
  }

  public setPath(path: string) {
    if (path) {
      this.path = this.path + path.startsWith("/") ? path : "/" + path;
    }
    return this;
  }

  public setPathWithTemplate(path: string, data: Record<string, string>) {
    if (path) {
      let template = parseTemplate(
        this.path + path.startsWith("/") ? path : "/" + path,
      );
      this.path = template.expand(data);
    }
    return this;
  }

  public setBody(
    body: T | undefined | string | Record<string, string> | Map<string, string>,
  ) {
    if (typeof body === "string") {
      this.body = JSON.stringify(body) as T;
    } else if (body instanceof Map) {
      this.body = Object.fromEntries(body) as T;
    } else {
      this.body = body as T;
    }
    return this;
  }

  setParams(urlParams: URLSearchParams) {
    this.urlParams = urlParams;
    return this;
  }
}

// const queryInstance = (() => {
//   let instance: QueryClient;
//
//   return {
//     getInstance: () => {
//       if (!instance) {
//         instance = new QueryClient();
//       }
//       return instance;
//     },
//   };
// })();
//
// // 使用方式
// export const queryClient = queryInstance.getInstance();
