// Import generated TypeScript types from ts-rs
import type { StreamResult } from "@/bindings/StreamResult";
import type { StreamDetail } from "@/bindings/StreamDetail";
import type { StreamResultError } from "@/bindings/StreamResultError";
import type { MediaType } from "@/bindings/MediaType";
import type { File } from "@/bindings/File";
import type { FileCandidacy } from "@/bindings/FileCandidacy";
import type { BitrateData } from "@/bindings/BitrateData";
import type { BitrateFrame } from "@/bindings/BitrateFrame";
export type { StreamResult, StreamDetail, StreamResultError, MediaType, File, FileCandidacy, BitrateData, BitrateFrame };

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
  },
  
  async analyzeFiles(paths: string[]): Promise<File[]> {
    return await invoke<File[]>("analyze_files", { paths });
  },

  async extractBitrateData(path: string): Promise<BitrateData> {
    return await invoke<BitrateData>("extract_bitrate_data", { path });
  }
};
