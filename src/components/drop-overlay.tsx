import { type ReactNode, useEffect, useRef, useState } from "react";
import { match, P } from "ts-pattern";
import {
	CheckCircle,
	File as FileIcon,
	FileText,
	Film,
	Image,
	Loader2,
	Music,
	XCircle,
} from "lucide-react";
import { commands } from "@/bindings";
import type { MediaType, StreamDetail } from "@/bindings";

type DropOverlayProps = {
	paths: string[];
};

type State =
	| { status: "hidden" }
	| { status: "loading"; count: number }
	| {
			status: "ready";
			files: {
				name: string;
				key: string;
				media_type: MediaType;
				duration?: number | null;
				size: number;
				streams: StreamDetail[];
			}[];
	  }
	| { status: "error"; reason: string; filename?: string; error_type?: string };

type FileItemProps = {
	filename: string;
	media_type: MediaType;
	duration?: number | null;
	size: number;
	streams: StreamDetail[];
	error?: string;
	error_type?: string;
};

const formatFileSize = (bytes: number): string => {
	if (bytes === 0) return "0 B";
	const k = 1024;
	const sizes = ["B", "KB", "MB", "GB"];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

const formatDuration = (seconds: number): string => {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = Math.floor(seconds % 60);

	if (hours > 0) {
		return `${hours}:${minutes.toString().padStart(2, "0")}:${secs
			.toString()
			.padStart(2, "0")}`;
	}
	return `${minutes}:${secs.toString().padStart(2, "0")}`;
};

const getFileIcon = (
	mediaType: MediaType,
	error?: string,
	errorType?: string,
) => {
	// For non-media files, show a neutral icon instead of error icon
	if (errorType === "not_media") {
		switch (mediaType) {
			case "Executable":
				return <FileIcon className="w-5 h-5 text-orange-400" />;
			case "Archive":
				return <FileIcon className="w-5 h-5 text-yellow-400" />;
			case "Library":
				return <FileIcon className="w-5 h-5 text-indigo-400" />;
			case "Document":
				return <FileText className="w-5 h-5 text-green-400" />;
			default:
				return <FileIcon className="w-5 h-5 text-neutral-300" />;
		}
	}

	if (error) {
		return <XCircle className="w-5 h-5 text-red-400" />;
	}

	switch (mediaType) {
		case "Audio":
			return <Music className="w-5 h-5 text-blue-400" />;
		case "Video":
			return <Film className="w-5 h-5 text-purple-400" />;
		case "Image":
			return <Image className="w-5 h-5 text-pink-400" />;
		case "Document":
			return <FileText className="w-5 h-5 text-green-400" />;
		case "Executable":
			return <FileIcon className="w-5 h-5 text-orange-400" />;
		case "Archive":
			return <FileIcon className="w-5 h-5 text-yellow-400" />;
		case "Library":
			return <FileIcon className="w-5 h-5 text-indigo-400" />;
		default:
			return <FileIcon className="w-5 h-5 text-neutral-300" />;
	}
};

const getStreamInfo = (
	streams: StreamDetail[],
	mediaType: MediaType,
): string => {
	// For non-media files, return file type description
	if (!["Audio", "Video", "Image"].includes(mediaType)) {
		switch (mediaType) {
			case "Executable":
				return "Executable file";
			case "Archive":
				return "Archive file";
			case "Library":
				return "Library file";
			case "Document":
				return "Document file";
			default:
				return "Unknown file type";
		}
	}

	// For media files, analyze streams
	const videoStreams = streams.filter((s: any) => "Video" in s);
	const audioStreams = streams.filter((s: any) => "Audio" in s);
	const subtitleStreams = streams.filter((s: any) => "Subtitle" in s);

	const parts = [] as string[];
	if (videoStreams.length > 0) {
		const video: any = videoStreams[0] as any;
		if ("Video" in video) {
			const width = (video as any).Video.width;
			const height = (video as any).Video.height;
			const codec = (video as any).Video.codec;
			if (width && height) {
				parts.push(`${width}x${height} ${codec}`);
			} else {
				parts.push(codec);
			}
		}
	}
	if (audioStreams.length > 0) {
		const audio: any = audioStreams[0] as any;
		if ("Audio" in audio) {
			parts.push(`${(audio as any).Audio.codec} audio`);
		}
	}
	if (subtitleStreams.length > 0) {
		parts.push(`${subtitleStreams.length} subtitle(s)`);
	}

	return parts.join(", ");
};

const Item = ({
	icon,
	text,
	subtitle,
	status,
}: {
	icon: ReactNode;
	text: ReactNode;
	subtitle?: ReactNode;
	status?: "success" | "error" | "loading";
}) => {
	const statusColor =
		status === "success"
			? "border-green-500"
			: status === "error"
				? "border-red-500"
				: status === "loading"
					? "border-blue-500"
					: "border-neutral-600";

	return (
		<div
			className={`flex items-center gap-3 px-4 py-3 bg-neutral-800 rounded-lg shadow-lg border-2 ${statusColor} transition-all duration-200`}
			style={{
				maxWidth: "100%",
				marginBottom: "0.75rem",
			}}
		>
			{icon}
			<div className="flex-1 min-w-0">
				<div className="truncate text-neutral-100 font-medium">{text}</div>
				{subtitle && (
					<div className="truncate text-neutral-400 text-sm mt-1">
						{subtitle}
					</div>
				)}
			</div>
		</div>
	);
};

const FileItem = ({
	filename,
	media_type,
	duration,
	size,
	streams,
	error,
	error_type,
}: FileItemProps) => {
	const icon = getFileIcon(media_type, error, error_type);
	const fileSize = formatFileSize(size);

	let subtitle: ReactNode;
	let status: "success" | "error" | "loading" | undefined;

	if (error) {
		subtitle = error;
		// For non-media files, show as neutral instead of error
		status = error_type === "not_media" ? undefined : "error";
	} else {
		const streamInfo = getStreamInfo(streams, media_type);
		const durationStr = duration ? formatDuration(duration) : null;
		const details = [streamInfo, durationStr, fileSize].filter(
			Boolean,
		) as string[];
		subtitle = details.join(" • ");
		status = "success";
	}

	return (
		<Item icon={icon} text={filename} subtitle={subtitle} status={status} />
	);
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
							media_type: item.media_type,
							duration: item.duration,
							size: Number(item.size),
							streams: item.streams,
						})),
					}))
					.with({ status: "error" }, (r) => {
						if (r.error.filename) {
							return {
								status: "error" as const,
								reason: r.error.reason,
								filename: r.error.filename,
								error_type: r.error.error_type,
							};
						}

						return {
							status: "error" as const,
							reason: r.error.reason,
							error_type: r.error.error_type,
						};
					})
					.exhaustive();
			});
		});
	}, [paths]);

	if (state.status === "hidden") {
		return null;
	}

	const inner = match(state)
		.with({ status: "loading" }, ({ count }) => (
			<div className="flex flex-col items-center gap-4">
				<Loader2 className="w-8 h-8 text-blue-400 animate-spin" />
				<div className="text-white text-lg font-medium">
					Analyzing {count} file{count > 1 ? "s" : ""}...
				</div>
				{Array.from({ length: Math.min(count, 3) }).map((_, i) => (
					<Item
						key={i}
						icon={
							<Loader2 className="w-5 h-5 text-neutral-300/50 animate-spin" />
						}
						text={
							<span className="inline-block w-32 h-5 bg-neutral-300/10 rounded animate-pulse" />
						}
						status="loading"
					/>
				))}
			</div>
		))
		.with({ status: "ready" }, (r) => {
			return (
				<div className="flex flex-col items-center gap-4">
					<div className="flex items-center gap-2 text-green-400">
						<CheckCircle className="w-6 h-6" />
						<span className="text-lg font-medium">Files Ready</span>
					</div>
					<div className="max-h-96 overflow-y-auto">
						{r.files.slice(0, 8).map((file) => (
							<FileItem
								key={file.key}
								filename={file.name}
								media_type={file.media_type}
								duration={file.duration}
								size={file.size}
								streams={file.streams}
							/>
						))}
					</div>
				</div>
			);
		})
		.with({ status: "error", filename: P.string }, (r) => {
			return (
				<div className="flex flex-col items-center gap-4">
					<div className="flex items-center gap-2 text-red-400">
						<XCircle className="w-6 h-6" />
						<span className="text-lg font-medium">Error</span>
					</div>
					<FileItem
						filename={r.filename}
						media_type="Unknown"
						size={0}
						streams={[]}
						error={r.reason}
						error_type={r.error_type}
					/>
				</div>
			);
		})
		.with({ status: "error" }, ({ reason }) => {
			return (
				<div className="flex flex-col items-center gap-4">
					<div className="flex items-center gap-2 text-red-400">
						<XCircle className="w-6 h-6" />
						<span className="text-lg font-medium">Error</span>
					</div>
					<Item
						icon={<XCircle className="w-5 h-5 text-red-400" />}
						text={reason}
						status="error"
					/>
				</div>
			);
		})
		.exhaustive();

	return (
		<div className="absolute z-10 top-0 left-0 w-full h-full bg-black/60 backdrop-blur-sm transition-all duration-300 ease-in-out">
			<div className="flex flex-col justify-center items-center h-full p-8">
				<div className="bg-neutral-900 rounded-xl p-6 shadow-2xl max-w-2xl w-full">
					{inner}
				</div>
			</div>
		</div>
	);
};

export default DropOverlay;
