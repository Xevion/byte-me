import { useEffect, useState } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

export function useDragDropPaths(): string[] {
	const [paths, setPaths] = useState<string[]>([]);

	useEffect(() => {
		const unlistenPromise = getCurrentWebview().onDragDropEvent(
			async ({ payload }) => {
				if (payload.type === "drop") {
					setPaths(payload.paths);
				} else if (payload.type === "leave") {
					setPaths([]);
				}
			},
		);
		return () => {
			unlistenPromise.then((unlisten) => unlisten());
		};
	}, []);

	return paths;
}
