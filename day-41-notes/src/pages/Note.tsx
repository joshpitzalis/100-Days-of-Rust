import { TrashIcon } from "@radix-ui/react-icons";
import { emit } from "@tauri-apps/api/event";
import {
	BaseDirectory,
	create,
	readTextFile,
	remove,
	rename,
	writeTextFile,
} from "@tauri-apps/plugin-fs";
import { ask } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useRef, useState, useMemo } from "react";
import { useNavigate, useParams } from "react-router-dom";
import EasyMde from "../components/EasyMde";
import TagsInput from "../components/TagsInput";
import { getTagsForNote, insertTag, deleteTag } from "../db";
import { nextUntitledName, trimExtensionName } from "../utils";

export default function Note() {
	const { file } = useParams<{ file: string }>();
	const navigate = useNavigate();
	const [content, setContent] = useState("");
	const [savedContent, setSavedContent] = useState("");
	const [tags, setTags] = useState<string[]>([]);
	const [saving, setSaving] = useState(false);
	const [editing, setEditing] = useState(false);
	const [draft, setDraft] = useState("");
	const inputRef = useRef<HTMLInputElement>(null);
	const timerRef = useRef<ReturnType<typeof setTimeout>>(null);
	const contentRef = useRef(content);
	const savedContentRef = useRef(content);

	useEffect(() => {
		if (file) {
			readTextFile(file, { baseDir: BaseDirectory.AppData }).then((text) => {
				setContent(text);
				setSavedContent(text);
			});
			getTagsForNote(file).then(setTags);
		} else {
			nextUntitledName().then(async (name) => {
				const f = await create(name, { baseDir: BaseDirectory.AppData });
				await f.close();
				navigate(`/note/${name}`, { replace: true });
			});
		}
	}, [file, navigate]);

	const handleTagsChange = useCallback(
		async (newTags: string[]) => {
			if (!file) return;
			const added = newTags.filter((t) => !tags.includes(t));
			const removed = tags.filter((t) => !newTags.includes(t));
			for (const tag of added) await insertTag(tag, file);
			for (const tag of removed) await deleteTag(tag, file);
			setTags(newTags);
		},
		[file, tags],
	);

	const saveNote = useCallback(
		async (file: string, text: string) => {
			setSaving(true);
			await writeTextFile(file, text, { baseDir: BaseDirectory.AppData });
			setSavedContent(text);
			setSaving(false);
		},
		[],
	);

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
		contentRef.current = content;
	}, [content]);

	useEffect(() => {
		savedContentRef.current = savedContent;
	}, [savedContent]);

	useEffect(() => {
		if (!file) return;
		if (content === savedContent) return;
		if (timerRef.current) clearTimeout(timerRef.current);
		timerRef.current = setTimeout(() => {
			saveNote(file, content);
		}, 500);
		return () => {
			if (timerRef.current) clearTimeout(timerRef.current);
		};
	}, [content, file, savedContent, saveNote]);

	// Flush pending save when navigating away
	useEffect(() => {
		return () => {
			if (timerRef.current) clearTimeout(timerRef.current);
			if (file && contentRef.current !== savedContentRef.current) {
				writeTextFile(file, contentRef.current, { baseDir: BaseDirectory.AppData });
			}
		};
	}, [file]);

	const saveStatus = useMemo(() => {
		if (saving) return "saving";
		if (content !== savedContent) return "unsaved";
		return "saved";
	}, [saving, content, savedContent]);

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
			<TagsInput value={tags} onChange={handleTagsChange} />
		<EasyMde value={content} onChange={setContent} />
			<p className={`text-xs mt-2 ${saveStatus === "unsaved" ? "text-ronin-yellow" : saveStatus === "saving" ? "text-spring-blue" : "text-autumn-green"}`}>
				{saveStatus === "saving" ? "Saving..." : saveStatus === "saved" ? "Saved" : "..."}
			</p>
		</div>
	);
}
