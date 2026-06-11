import { useQuery } from "react-query";
import { useAuth } from "./AuthContext";
import { ReactElement } from "react";
import { AxiosResponse } from "axios";

type Props<T> = {
  dataKey: string;
  retriever: (token: string) => Promise<AxiosResponse<T, any>>;
  renderData: (data: T) => ReactElement;
};

export default function <T>(props: Props<T>): ReactElement {
  const auth = useAuth();
  const token = auth.token ?? "";
  const { data, isLoading, isError, error } = useQuery([props.dataKey, token], () => props.retriever(token), {
    enabled: token.length > 0,
  });

  if (isLoading) {
    return <p className="mt-4 text-sm text-gray-500">Loading...</p>;
  }

  if (isError) {
    return <p className="mt-4 text-sm text-red-600">Failed to load data: {error instanceof Error ? error.message : "Unknown error"}</p>;
  }

  if (data) {
    return props.renderData(data.data);
  }

  return <></>;
}
