import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type DropOverlayProps = {
  paths: string[];
};

type Status = "hidden" | "loading" | "ready" | "error";

const DropOverlay = ({ paths }: DropOverlayProps) => {
  const [status, setStatus] = useState<Status>("hidden");

  useEffect(() => {
    if (paths.length === 0) {
      setStatus("hidden");
      return;
    }

    setStatus("loading");
    invoke("has_streams", { paths }).then((result) => {
      setStatus(result ? "ready" : "error");
    });
  }, [paths]);

  return (
    <div
      className={`absolute z-10 top-0 left-0 w-full h-full transition-[opacity] bg-black/20 duration-200 ease-in-out ${
        status === "hidden" ? "opacity-0 pointer-events-none" : "opacity-100"
      }`}
    >
      <div className="flex flex-col items-center justify-center shadow h-full">
        <div className="text-2xl font-bold text-zinc-200">
          {status === "loading"
            ? "Loading..."
            : status === "ready"
            ? "Ready"
            : status === "error"
            ? "Error"
            : "Hidden"}
        </div>
      </div>
    </div>
  );
};

export default DropOverlay;
