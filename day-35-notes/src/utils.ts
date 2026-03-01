import { BaseDirectory, exists, mkdir } from "@tauri-apps/plugin-fs";

export async function ensureAppDirExists() {
	if (!(await exists("", { baseDir: BaseDirectory.AppData }))) {
		await mkdir("", { baseDir: BaseDirectory.AppData });
	}
}
