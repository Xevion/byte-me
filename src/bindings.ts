// Import generated TypeScript types from ts-rs
export type { StreamResult } from "./bindings/StreamResult";
export type { StreamDetail } from "./bindings/StreamDetail";
export type { StreamResultError } from "./bindings/StreamResultError";
export type { MediaType } from "./bindings/MediaType";

// Tauri invoke wrapper
import { invoke } from "@tauri-apps/api/core";

export type Result<T, E> = 
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

export const commands = {
  async hasStreams(paths: string[]): Promise<Result<StreamResult[], StreamResultError>> {
    try {
      const data = await invoke<StreamResult[]>("has_streams", { paths });
      return { status: "ok", data };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  }
};
