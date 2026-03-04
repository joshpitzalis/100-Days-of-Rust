import { invoke } from "@tauri-apps/api/core";
import { createNote } from "./features/notes/actions";

document.addEventListener("keydown", (event) => {
	const isCmdOrCtrl = event.metaKey || event.ctrlKey;

	if (isCmdOrCtrl && event.key.toLowerCase() === "n") {
		if (event.shiftKey) {
			invoke("new_window");
		} else {
			createNote();
		}
	}
	event.preventDefault();
});
