import { useDragDropPaths } from "./hooks/useDragDropPaths";
import Graph from "./components/graph";
import DropOverlay from "./components/drop-overlay";
import type { Frame } from "./types/graph";

function App() {
	const data: Frame[] = [];

	const paths = useDragDropPaths();

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
