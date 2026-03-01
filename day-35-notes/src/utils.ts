import { BaseDirectory, exists, mkdir } from "@tauri-apps/plugin-fs";

export async function ensureAppDirExists() {
	if (!(await exists("", { baseDir: BaseDirectory.AppData }))) {
		await mkdir("", { baseDir: BaseDirectory.AppData });
	}
}

export async function nextUntitledName(): Promise<string> {
	const base = "Untitled";
	let name = `${base}.md`;
	let i = 1;
	while (await exists(name, { baseDir: BaseDirectory.AppData })) {
		name = `${base} ${i}.md`;
		i++;
	}
	return name;
}
