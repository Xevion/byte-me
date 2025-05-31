import { useState } from "react";
import logo from "./assets/images/logo-universal.png";
import "./App.css";
import { Greet } from "../wailsjs/go/main/App.js";
import { ResponsiveLine } from "@nivo/line";

function App() {
  const [resultText, setResultText] = useState(
    "Please enter your name below 👇"
  );
  const [name, setName] = useState("");
  const updateName = (e: any) => setName(e.target.value);
  const updateResultText = (result: string) => setResultText(result);

  const data = [
    {
      id: "japan",
      data: [
        {
          x: "plane",
          y: 99,
        },
        {
          x: "helicopter",
          y: 80,
        },
        {
          x: "boat",
          y: 60,
        },
        {
          x: "train",
          y: 179,
        },
        {
          x: "subway",
          y: 102,
        },
        {
          x: "bus",
          y: 68,
        },
        {
          x: "car",
          y: 200,
        },
        {
          x: "moto",
          y: 38,
        },
        {
          x: "bicycle",
          y: 32,
        },
        {
          x: "horse",
          y: 84,
        },
        {
          x: "skateboard",
          y: 93,
        },
        {
          x: "others",
          y: 206,
        },
      ],
    },
    {
      id: "france",
      data: [
        {
          x: "plane",
          y: 227,
        },
        {
          x: "helicopter",
          y: 278,
        },
        {
          x: "boat",
          y: 241,
        },
        {
          x: "train",
          y: 104,
        },
        {
          x: "subway",
          y: 140,
        },
        {
          x: "bus",
          y: 16,
        },
        {
          x: "car",
          y: 21,
        },
        {
          x: "moto",
          y: 135,
        },
        {
          x: "bicycle",
          y: 158,
        },
        {
          x: "horse",
          y: 41,
        },
        {
          x: "skateboard",
          y: 20,
        },
        {
          x: "others",
          y: 172,
        },
      ],
    },
    {
      id: "us",
      data: [
        {
          x: "plane",
          y: 54,
        },
        {
          x: "helicopter",
          y: 59,
        },
        {
          x: "boat",
          y: 165,
        },
        {
          x: "train",
          y: 213,
        },
        {
          x: "subway",
          y: 79,
        },
        {
          x: "bus",
          y: 248,
        },
        {
          x: "car",
          y: 184,
        },
        {
          x: "moto",
          y: 251,
        },
        {
          x: "bicycle",
          y: 122,
        },
        {
          x: "horse",
          y: 12,
        },
        {
          x: "skateboard",
          y: 269,
        },
        {
          x: "others",
          y: 101,
        },
      ],
    },
    {
      id: "germany",
      data: [
        {
          x: "plane",
          y: 177,
        },
        {
          x: "helicopter",
          y: 249,
        },
        {
          x: "boat",
          y: 37,
        },
        {
          x: "train",
          y: 173,
        },
        {
          x: "subway",
          y: 145,
        },
        {
          x: "bus",
          y: 283,
        },
        {
          x: "car",
          y: 50,
        },
        {
          x: "moto",
          y: 231,
        },
        {
          x: "bicycle",
          y: 100,
        },
        {
          x: "horse",
          y: 226,
        },
        {
          x: "skateboard",
          y: 5,
        },
        {
          x: "others",
          y: 139,
        },
      ],
    },
    {
      id: "norway",
      data: [
        {
          x: "plane",
          y: 234,
        },
        {
          x: "helicopter",
          y: 38,
        },
        {
          x: "boat",
          y: 254,
        },
        {
          x: "train",
          y: 228,
        },
        {
          x: "subway",
          y: 106,
        },
        {
          x: "bus",
          y: 213,
        },
        {
          x: "car",
          y: 289,
        },
        {
          x: "moto",
          y: 116,
        },
        {
          x: "bicycle",
          y: 272,
        },
        {
          x: "horse",
          y: 50,
        },
        {
          x: "skateboard",
          y: 263,
        },
        {
          x: "others",
          y: 196,
        },
      ],
    },
  ];

  function greet() {
    Greet(name).then(updateResultText);
  }

  return (
    <div id="App" className="min-h-screen min-w-screen">
      <ResponsiveLine /* or Line for fixed dimensions */
        data={data}
        margin={{ top: 50, right: 110, bottom: 50, left: 60 }}
        yScale={{
          type: "linear",
          min: "auto",
          max: "auto",
          stacked: true,
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
        axisLeft={{ legend: "count", legendOffset: -40 }}
        pointSize={10}
        pointColor={{ modifiers: [["brighter", 1100]], from: "" }}
        pointBorderWidth={2}
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
    </div>
  );
}

export default App;
