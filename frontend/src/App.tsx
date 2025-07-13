import { ResponsiveLine } from "@nivo/line";
import { useEffect, useMemo } from "react";
import { OnFileDrop, OnFileDropOff } from "../wailsjs/runtime/runtime.js";
import "./App.css";
import { formatBytes } from "./lib/format.js";

type Frame = {
  id: string;
  data: { x: string | number; y: number }[];
};

function App() {
  useEffect(() => {
    OnFileDrop((_x, _y, paths) => {}, true);
    return () => OnFileDropOff();
  }, []);

  const data: Frame[] = [];
  // const data: Frame[] = useMemo(() =>
  //   // Array.from({ length: 4 }, (_, i) => {
  //   //   const d = Math.random();
  //   //   const g = Math.random();
  //   //   return {
  //   //     id: `file-${i}`,
  //   //     data: Array.from({ length: 500 }, (_, j) => {
  //   //       if (Math.random() < 0.5) return null;
  //   //       return {
  //   //         x: j,
  //   //         y: Math.random() * 256 * d + (1 - g) * 1024,
  //   //       };
  //   //     }).filter((i) => i !== null),
  //   //   };
  //   // }),
  //   []
  // );

  console.log(data);

  const graph = (
    <ResponsiveLine
      data={data}
      margin={{ top: 50, right: 110, bottom: 50, left: 60 }}
      xScale={{ type: "linear" }}
      yScale={{
        type: "linear",
        min: 0,
        max: "auto",
        stacked: false,
        reverse: false,
      }}
      theme={{
        tooltip: {
          container: {
            backgroundColor: "#2e2b45",
          },
        },
        grid: {
          line: {
            stroke: "rgb(252, 191, 212)",
            strokeWidth: 0.35,
            strokeOpacity: 0.75,
          },
        },
        crosshair: {
          line: {
            stroke: "#fdd3e2",
            strokeWidth: 1,
          },
        },
        axis: {
          legend: {},

          domain: {
            line: {
              stroke: "rgb(252, 191, 212)",
              strokeWidth: 0.5,
              strokeOpacity: 0.5,
            },
          },
        },
        text: {
          fill: "#6e6a86",
        },
      }}
      axisBottom={{ legend: "transportation", legendOffset: 36 }}
      axisLeft={{
        legend: "count",
        legendOffset: -40,
        format: (v) => formatBytes(v * 1024 * 53),
      }}
      pointSize={10}
      colors={[
        "#3e8faf",
        "#c4a7e7",
        "#f5c276",
        "#EA9B96",
        "#EB7092",
        "#9CCFD8",
      ]}
      // pointColor={{ modifiers: [["brighter", 1100]] }}
      pointBorderWidth={0}
      pointBorderColor={{ from: "seriesColor" }}
      pointLabelYOffset={-12}
      enableSlices={"x"}
      enableTouchCrosshair={true}
      useMesh={true}
      legends={[
        {
          anchor: "bottom-right",
          direction: "column",
          translateX: 100,
          itemWidth: 80,
          itemHeight: 22,
          symbolShape: "circle",
        },
      ]}
    />
  );

  return (
    <div
      id="App"
      className="min-h-screen min-w-screen overflow-hidden"
      style={{ "--wails-drop-target": "drop" } as React.CSSProperties}
    >
      <div
        id="drop-target"
        className="absolute z-10 top-0 left-0 w-full h-full transition-[opacity] duration-200 ease-in-out"
      >
        <div className="flex flex-col items-center justify-center shadow h-full">
          <div className="text-2xl font-bold text-zinc-200">
            Drag and Drop to Add
          </div>
        </div>
      </div>
      {graph}
    </div>
  );
}

export default App;
