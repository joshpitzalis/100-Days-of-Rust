import { emit } from "@tauri-apps/api/event";
import { BaseDirectory, create } from "@tauri-apps/plugin-fs";
import type { NavigateFunction } from "react-router-dom";
import { nextUntitledName } from "../../utils";

export async function createNote(navigate?: NavigateFunction) {
	const name = await nextUntitledName();
	const file = await create(name, { baseDir: BaseDirectory.AppData });
	await file.close();
	emit("note-created", name);
	if (navigate) {
		navigate(`/note/${name}`);
	}
}
