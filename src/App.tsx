import { useDragDropPaths } from "./hooks/useDragDropPaths.js";
import Graph from "./features/graph/graph.js";
import DropOverlay from "./features/drop/drop-overlay.js";
import type { Frame } from "./types/graph.js";

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
