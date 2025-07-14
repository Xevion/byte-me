import { ReactNode, useEffect, useRef, useState } from "react";
import { match, P } from "ts-pattern";

type DropOverlayProps = {
	paths: string[];
};

type State =
	| { status: "hidden" }
	| { status: "loading"; count: number }
	| { status: "ready"; files: { name: string; key: string }[] }
	| { status: "error"; reason: string; filename?: string };

import {
	CircleQuestionMarkIcon,
	File as FileIcon,
	Film,
	Image,
	Music,
} from "lucide-react";
import { commands } from "../bindings";

type FileItemProps = {
	filename: string;
	error?: string;
};

const Item = ({ icon, text }: { icon: ReactNode; text: ReactNode }) => {
	return (
		<div
			className="flex items-center gap-2 px-3 py-2 bg-neutral-800 rounded-md shadow-sm"
			style={{
				maxWidth: "100%",
				marginBottom: "0.5rem",
			}}
		>
			{icon}
			<span className="truncate text-neutral-100 max-w-md">{text}</span>
		</div>
	);
};

const FileItem = ({ filename, error }: FileItemProps) => {
	const ext = filename.split(".").pop()?.toLowerCase();
	const icon =
		error == null ? (
			match(ext)
				.with("mp3", "wav", "flac", "ogg", "m4a", "aac", () => (
					<Music className="w-5 h-5 text-blue-400" />
				))
				.with("mp4", "mkv", "webm", "mov", "avi", () => (
					<Film className="w-5 h-5 text-purple-400" />
				))
				.with("gif", () => <Image className="w-5 h-5 text-pink-400" />)
				.otherwise(() => <FileIcon className="w-5 h-5 text-neutral-300" />)
		) : (
			<CircleQuestionMarkIcon className="w-5 h-5 text-neutral-300" />
		);

	return <Item icon={icon} text={filename} />;
};

const DropOverlay = ({ paths }: DropOverlayProps) => {
	const [state, setState] = useState<State>({ status: "hidden" });
	const aborterRef = useRef<AbortController | null>(null);

	useEffect(() => {
		if (paths.length === 0) {
			setState({ status: "hidden" });
			return;
		}

		setState({ status: "loading", count: paths.length });

		aborterRef.current = new AbortController();

		commands.hasStreams(paths).then((result) => {
			setState((_state) => {
				return match(result)
					.with({ status: "ok" }, (r) => ({
						status: "ready" as const,
						files: r.data.map((item) => ({
							name: item.filename,
							key: item.path,
						})),
					}))
					.with({ status: "error" }, (r) => {
						if (r.error.filename) {
							return {
								status: "error" as const,
								reason: r.error.reason,
								filename: r.error.filename,
							};
						}

						return { status: "error" as const, reason: r.error.reason };
					})
					.exhaustive();
			});
		});
	}, [paths]);

	if (state.status === "hidden") {
		return null;
	}

	const inner = match(state)
		.with({ status: "loading" }, ({ count }) =>
			Array.from({ length: count }).map((_, i) => (
				<Item
					key={i}
					icon={
						<CircleQuestionMarkIcon className="w-5 h-5 text-neutral-300/50" />
					}
					text={
						<span className="inline-block w-32 h-5 bg-neutral-300/10 rounded animate-pulse" />
					}
				/>
			)),
		)
		.with({ status: "ready" }, (r) => {
			return r.files
				.slice(0, 8)
				.map((file) => <FileItem key={file.key} filename={file.name} />);
		})
		.with({ status: "error", filename: P.string }, (r) => {
			return <FileItem filename={r.filename} error={r.reason} />;
		})
		.with({ status: "error" }, ({ reason }) => {
			return (
				<Item
					icon={<CircleQuestionMarkIcon className="w-5 h-5 text-neutral-300" />}
					text={reason}
				/>
			);
		})
		.exhaustive();

	return (
		<div className="absolute z-10 top-0 left-0 w-full h-full bg-black/40 backdrop-blur-sm transition-all duration-300 ease-in-out">
			<div className="flex flex-col justify-center items-center h-full">
				<span className="text-white text-2xl">{inner}</span>
			</div>
		</div>
	);
};

export default DropOverlay;
