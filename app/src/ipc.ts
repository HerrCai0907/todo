import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core";
import { message } from "antd";

interface SuccessResponse<T> {
  data: T;
}

interface ErrorResponse {
  error: string;
}

export async function ipc<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
  try {
    let response = await invoke<string>(cmd, args, options);
    let res: SuccessResponse<T> | ErrorResponse = JSON.parse(response);
    if ("error" in res) throw new Error(res.error);
    return res.data;
  } catch (e) {
    console.error(e);
    if (e instanceof Error) message.error(`Error fetching todo list\n${e.message}`);
    throw e;
  }
}
