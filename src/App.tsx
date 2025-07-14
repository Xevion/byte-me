type Frame = {
  id: string;
  data: { x: string | number; y: number }[];
};

import { getCurrentWebview } from "@tauri-apps/api/webview";
import { useEffect, useState } from "react";
import Graph from "./components/graph.js";
import DropOverlay from "./components/drop-overlay.js";

function App() {
  const data: Frame[] = [];

  const [paths, setPaths] = useState<string[]>([]);
  useEffect(() => {
    const unlistenPromise = getCurrentWebview().onDragDropEvent(
      async ({ payload }) => {
        if (payload.type === "enter") {
          setPaths(payload.paths);
          console.log("User hovering", payload);
        } else if (payload.type === "leave" || payload.type === "drop") {
          setPaths([]);
          console.log("User left", payload);
        }
      }
    );

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
    return () => {
      unlistenPromise.then((unlisten) => {
        unlisten();
        console.log("Unlistened");
      });
    };
  }, []);

  const graph = <Graph data={data} />;

  return (
    <div
      id="App"
      className="min-h-screen min-w-screen overflow-hidden"
      style={{ "--wails-drop-target": "drop" } as React.CSSProperties}
    >
      <DropOverlay paths={paths} />
      {graph}
    </div>
  );
}

export default App;
