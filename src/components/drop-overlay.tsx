import { type ReactNode, useEffect, useState } from "react";
import { match, P } from "ts-pattern";
import {
	File as FileIcon,
	FileText,
	Film,
	Image,
	Loader2,
	Music,
	XCircle,
} from "lucide-react";
import { commands } from "@/bindings";
import type { File, FileCandidacy, MediaType } from "@/bindings";

type DropOverlayProps = {
	paths: string[];
};

const formatFileSize = (bytes: number): string => {
	if (bytes === 0) return "0 B";
	const k = 1024;
	const sizes = ["B", "KB", "MB", "GB"];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

const getFileIcon = (candidacy: FileCandidacy): ReactNode => {
	return match(candidacy)
		.with("Loading", () => (
			<Loader2 className="w-5 h-5 text-blue-400 animate-spin" />
		))
		.with({ Error: P._ }, () => <XCircle className="w-5 h-5 text-red-400" />)
		.with({ Success: { type: P.select() } }, (mediaType: MediaType) => {
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
		})
		.exhaustive();
};

const getStatusColor = (candidacy: FileCandidacy): string => {
	return match(candidacy)
		.with("Loading", () => "border-blue-500/50")
		.with({ Error: P._ }, () => "border-red-500/50")
		.with({ Success: P._ }, () => "border-green-500/50")
		.exhaustive();
};

const FileItem = ({ file }: { file: File }) => {
	const icon = getFileIcon(file.candidacy);
	const statusColor = getStatusColor(file.candidacy);
	const fileSize = formatFileSize(file.size);

	const subtitle = match(file.candidacy)
		.with("Loading", () => "Analyzing...")
		.with({ Error: { reason: P.select() } }, (reason: string) => reason)
		.with({ Success: { type: P.select() } }, (mediaType: MediaType) => {
			switch (mediaType) {
				case "Audio":
					return "Audio file";
				case "Video":
					return "Video file";
				case "Image":
					return "Image file";
				case "Document":
					return "Document file";
				case "Executable":
					return "Executable file";
				case "Archive":
					return "Archive file";
				case "Library":
					return "Library file";
				default:
					return "Unknown file type";
			}
		})
		.exhaustive();

	return (
		<div
			className={`flex items-center gap-3 px-4 py-3 rounded-lg bg-neutral-800 border ${statusColor} transition-all duration-200`}
			style={{
				maxWidth: "100%",
				marginBottom: "0.75rem",
			}}
		>
			{icon}
			<div className="flex-1 min-w-0">
				<div className="truncate text-neutral-100 font-medium">
					{file.filename}
				</div>
				<div className="truncate text-neutral-400 text-sm mt-1">
					{fileSize} • {subtitle}
				</div>
			</div>
		</div>
	);
};

const DropOverlay = ({ paths }: DropOverlayProps) => {
	const [files, setFiles] = useState<File[]>([]);
	const [isLoading, setIsLoading] = useState(false);

	useEffect(() => {
		if (paths.length === 0) {
			setFiles([]);
			setIsLoading(false);
			return;
		}

		setIsLoading(true);
		setFiles([]);

		// Initialize with loading state for all files
		const loadingFiles: File[] = paths.map((path) => {
			const filename = path.split(/[/\\]/).pop() || "unknown";
			return {
				filename,
				size: 0,
				candidacy: "Loading" as const,
			};
		});
		setFiles(loadingFiles);

		// Analyze files
		commands
			.analyzeFiles(paths)
			.then((analyzedFiles) => {
				setFiles(analyzedFiles);
				setIsLoading(false);
			})
			.catch((error) => {
				console.error("Failed to analyze files:", error);
				// Set all files to error state
				const errorFiles: File[] = paths.map((path) => {
					const filename = path.split(/[/\\]/).pop() || "unknown";
					return {
						filename,
						size: 0,
						candidacy: {
							Error: {
								reason: "Failed to analyze file",
							},
						},
					};
				});
				setFiles(errorFiles);
				setIsLoading(false);
			});
	}, [paths]);

	if (files.length === 0) {
		return null;
	}

	return (
		<div className="absolute z-10 top-0 left-0 w-full h-full backdrop-blur-[1px] backdrop-saturate-0 transition-all duration-300 ease-in-out">
			<div className="flex flex-col justify-center items-center h-full p-8">
				<div className="rounded-xl p-6 max-w-2xl w-full">
					<div className="flex flex-col items-center gap-4">
						{isLoading && (
							<div className="flex items-center gap-2 text-blue-400 mb-4">
								<Loader2 className="w-6 h-6 animate-spin" />
								<span className="text-lg font-medium">
									Analyzing {files.length} file{files.length > 1 ? "s" : ""}...
								</span>
							</div>
						)}
						<div className="max-h-96 overflow-y-auto w-full">
							{files.map((file, index) => (
								<FileItem key={`${file.filename}-${index}`} file={file} />
							))}
						</div>
					</div>
				</div>
			</div>
		</div>
	);
};

export default DropOverlay;
