import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core";

interface SuccessResponse<T> {
  data: T;
}

interface ErrorResponse {
  error: string;
}

export async function ipc<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
  let response = await invoke<string>(cmd, args, options);
  console.log(response);
  let res: SuccessResponse<T> | ErrorResponse = JSON.parse(response);
  if ("error" in res) throw new Error(res.error);
  return res.data;
}
