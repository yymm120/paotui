export const getSavedToken = () => {
  return localStorage.getItem("token");
};

export const Query = async (input: RequestInfo, init?: RequestInit) => {
  const token = getSavedToken();
  const headers = {
    ...init?.headers,
    ...(token ? { Authorization: `${token}` } : {}),
    "Content-Type": "application/json",
  };

  return fetch(input, {
    ...init,
    headers,
  });
};
