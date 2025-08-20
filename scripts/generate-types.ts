#!/usr/bin/env node

import { execSync } from "child_process";
import { copyFileSync, mkdirSync, existsSync, readdirSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log("🔄 Generating TypeScript bindings...");

try {
	// Run the test to generate bindings
	execSync("cargo test export_bindings", {
		cwd: "./src-tauri",
		stdio: "inherit",
	});

	if (!existsSync(join(__dirname, "../src-tauri/bindings"))) {
		throw new Error(
			"Bindings directory not found. Bindings generation failed or improperly configured.",
		);
	}

	console.log("✅ TypeScript bindings generated successfully!");

	// Copy bindings to src directory
	const srcBindingsDir = join(__dirname, "../src/bindings");
	const files = readdirSync(join(__dirname, "../src-tauri/bindings")).filter(
		(file) => file.endsWith(".ts"),
	);

	if (files.length === 0) {
		throw new Error(
			"No bindings files found. Bindings generation failed or improperly configured.",
		);
	}

	for (const file of files) {
		const source = join(__dirname, "../src-tauri/bindings", file);
		const dest = join(srcBindingsDir, file);
		copyFileSync(source, dest);
		console.log(`📁 Copied ${file} to src/bindings/`);
	}

	console.log("🎉 All done! TypeScript bindings are up to date.");
} catch (error) {
	console.error("❌ Failed to generate TypeScript bindings:", error);
	process.exit(1);
}
