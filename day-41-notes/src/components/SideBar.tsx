import { Pencil2Icon, BookmarkIcon } from "@radix-ui/react-icons";
import { emit, listen } from "@tauri-apps/api/event";
import {
	BaseDirectory,
	create,
	type DirEntry,
	readDir,
} from "@tauri-apps/plugin-fs";
import { useCallback, useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { createNote } from "../features/notes/actions";
import { nextUntitledName, trimExtensionName } from "../utils";

export default function SideBar() {
	const navigate = useNavigate();
	const [files, setFiles] = useState<DirEntry[]>([]);

	const getNotes = useCallback(async () => {
		const dir = await readDir("", { baseDir: BaseDirectory.AppData });
		const files = dir.filter(
			(entry) => entry.isFile && entry.name.endsWith(".md"),
		);
		setFiles(files);
	}, []);

	useEffect(() => {
		getNotes();
		const unlisten = listen("note-created", getNotes);
		return () => {
			unlisten.then((fn) => fn());
		};
	}, [getNotes]);
	return (
		<div className="w-full max-w-[300px] xl:max-w-[350px] mr-6">
			<nav className="fixed left-4 top-[20px] max-w-[300px] xl:max-w-[350px] w-full bg-sumi-ink-0 text-old-white h-[calc(100vh-40px)] overflow-y-auto rounded-lg [&::-webkit-scrollbar]:hidden">
				<button
					type="button"
					className="flex items-center gap-3 mb-4 py-4 px-6 w-full text-fuji-white hover:bg-winter-blue hover:text-spring-blue border-b border-sumi-ink-3 cursor-pointer"
					onClick={() => createNote(navigate)}
				>
					<Pencil2Icon />
					<span>New Note</span>
				</button>

				<Link
					to="/tags"
					className="flex items-center gap-3 py-3 px-6 w-full text-fuji-white hover:bg-winter-blue hover:text-spring-blue border-b border-sumi-ink-3"
				>
					<BookmarkIcon />
					<span>Tags</span>
				</Link>

				<ul className="flex flex-col text-sm">
					{files.map((file) => (
						<li key={file.name}>
							<Link
								to={`/note/${file.name}`}
								className="block w-full px-6 py-1 hover:bg-winter-blue hover:text-spring-blue"
							>
								{trimExtensionName(file.name)}
							</Link>
						</li>
					))}
				</ul>
			</nav>
		</div>
	);
}
