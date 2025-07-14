import { ResponsiveLine } from "@nivo/line";
import { formatBytes } from "../lib/format.js";

type Frame = {
	id: string;
	data: { x: string | number; y: number }[];
};

type GraphProps = {
	data: Frame[];
};

const Graph = ({ data }: GraphProps) => (
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
		colors={["#3e8faf", "#c4a7e7", "#f5c276", "#EA9B96", "#EB7092", "#9CCFD8"]}
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

export default Graph;
