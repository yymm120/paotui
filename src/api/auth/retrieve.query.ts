import { Query_code, Query_init } from "@/api/auth/query";
import type { Profile } from "@/components/profile-context-provider.tsx";

const normalize_res = (data: any): Profile => {
  return {
    userId: data.user.user_id,
    userName: data.user.user_name,
    userPhone: data.user.phone_number,
    userType: data.user.userType ?? "UnsignedUpUser",
    userWorkingStatus: data.user.working_status ?? "rest",
  };
};

export const RetrieveQueryForInit = {
  symbol: Query_init,
  path: "/api/_",
  method: "GET",
  normalize_res: normalize_res,
} as const;

export const RetrieveQueryForCode = {
  symbol: Query_code,
  path: "/api/code",
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: { phone_number: undefined as unknown as string },
  // normalize_res: normalize_res,
} as const;
