import { TrashIcon } from "@radix-ui/react-icons";
import { emit } from "@tauri-apps/api/event";
import { ask } from "@tauri-apps/plugin-dialog";
import {
	BaseDirectory,
	create,
	readTextFile,
	remove,
	rename,
	writeTextFile,
} from "@tauri-apps/plugin-fs";
import { useCallback, useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import EasyMde from "../components/EasyMde";
import { nextUntitledName, trimExtensionName } from "../utils";

export default function Note() {
	const { file } = useParams<{ file: string }>();
	const navigate = useNavigate();
	const [content, setContent] = useState("");
	const [editing, setEditing] = useState(false);
	const [draft, setDraft] = useState("");
	const inputRef = useRef<HTMLInputElement>(null);

	useEffect(() => {
		if (file) {
			readTextFile(file, { baseDir: BaseDirectory.AppData }).then(setContent);
		} else {
			nextUntitledName().then(async (name) => {
				const f = await create(name, { baseDir: BaseDirectory.AppData });
				await f.close();
				navigate(`/note/${name}`, { replace: true });
			});
		}
	}, [file, navigate]);

	const saveNote = useCallback(async (file: string, content: string) => {
		await writeTextFile(file, content, { baseDir: BaseDirectory.AppData });
		console.log("write", content);
	}, []);

	function startRename() {
		if (!file) return;
		setDraft(trimExtensionName(file));
		setEditing(true);
		setTimeout(() => inputRef.current?.select(), 0);
	}

	async function deleteNote() {
		if (!file) return;
		const confirmed = await ask(`Delete "${trimExtensionName(file)}"?`, {
			title: "Delete Note",
			kind: "warning",
		});
		if (!confirmed) return;
		await remove(file, { baseDir: BaseDirectory.AppData });
		emit("note-created"); // refresh sidebar
		navigate("/", { replace: true });
	}

	async function commitRename() {
		setEditing(false);
		if (!file) return;
		const newName = `${draft.trim()}.md`;
		if (!draft.trim() || newName === file) return;
		await rename(file, newName, {
			oldPathBaseDir: BaseDirectory.AppData,
			newPathBaseDir: BaseDirectory.AppData,
		});
		emit("note-created"); // refresh sidebar
		navigate(`/note/${newName}`, { replace: true });
	}

	useEffect(() => {
		if (!file) return;
		saveNote(file, content);
	}, [content, file, saveNote]);

	return (
		<div className="p-8 mx-auto max-w-[400px] text-center text-fuji-white">
			{editing ? (
				<input
					ref={inputRef}
					className="text-2xl font-bold mb-5 mt-12 bg-transparent border-b border-fuji-white text-fuji-white outline-none text-center w-full"
					value={draft}
					onChange={(e) => setDraft(e.target.value)}
					onBlur={commitRename}
					onKeyDown={(e) => {
						if (e.key === "Enter") commitRename();
						if (e.key === "Escape") setEditing(false);
					}}
				/>
			) : (
				<div className="flex items-center justify-center gap-3 mb-5 mt-12">
					<h1
						className="text-2xl font-bold cursor-pointer hover:underline"
						onDoubleClick={startRename}
						title="Double-click to rename"
					>
						{file && trimExtensionName(file)}
					</h1>
					<button
						type="button"
						className="text-old-white hover:text-red-400 cursor-pointer"
						onClick={deleteNote}
						title="Delete note"
					>
						<TrashIcon width={18} height={18} />
					</button>
				</div>
			)}
			<EasyMde value={content} onChange={setContent} />
		</div>
	);
}
