import { createContext, type ReactElement } from "react";
import { Query_init_data } from "@/api";
import { useQuery, useQueryTask } from "@/hooks/use-query.ts";

export type Profile = {
  userName: string;
  userId: string;
  userPhone: string;
  userType: string;
  userWorkingStatus: "working" | "rest";
};

type ProfileContextProviderProps = {
  children: ReactElement;
} & Partial<Profile>;

export const ProfileContext = createContext<Profile>({} as Profile);
export function ProfileContextProvider({
  children,
}: ProfileContextProviderProps) {
  const { isLoading, data: profile, error } = useQuery<Profile>(Query_init);

  // const update = useQueryTask(Update_profile_data)

  console.log(profile, error);
  return (
    <ProfileContext.Provider value={profile}>
      {isLoading ? (
        <span>Loading...</span>
      ) : !!error ? (
        error?.message
      ) : (
        children
      )}
    </ProfileContext.Provider>
  );
}
