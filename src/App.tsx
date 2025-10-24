import { useEffect, useState } from "react";
import { useDragDropPaths } from "@/hooks/useDragDropPaths";
import Graph from "@/components/graph";
import DropOverlay from "@/components/drop-overlay";
import type { Frame } from "@/types/graph";
import { commands } from "@/bindings";
import type { BitrateData } from "@/bindings";

function App() {
	const [data, setData] = useState<Frame[]>([]);
	const [isLoading, setIsLoading] = useState(false);
	const paths = useDragDropPaths();

	useEffect(() => {
		if (paths.length === 0) {
			return;
		}

		// For minimal prototype, just process the first file
		const firstPath = paths[0];
		setIsLoading(true);

		commands
			.extractBitrateData(firstPath)
			.then((bitrateData: BitrateData) => {
				// Transform BitrateData to Nivo's Frame format
				const frame: Frame = {
					id: bitrateData.id,
					data: bitrateData.frames.map((frame) => ({
						x: frame.frame_num,
						y: Number(frame.packet_size),
					})),
				};
				setData([frame]);
				setIsLoading(false);
			})
			.catch((error) => {
				console.error("Failed to extract bitrate data:", error);
				setIsLoading(false);
			});
	}, [paths]);

	const graph = <Graph data={data} />;

	return (
		<div
			id="App"
			className="min-h-screen min-w-screen overflow-hidden"
			style={{ "--wails-drop-target": "drop" } as React.CSSProperties}
		>
			<DropOverlay paths={paths} />
			{isLoading && (
				<div className="absolute z-20 top-4 right-4 text-white bg-blue-600 px-4 py-2 rounded-lg">
					Extracting bitrate data...
				</div>
			)}
			{graph}
		</div>
	);
}

export default App;
