import {
	BaseDirectory,
	create,
	exists,
	readTextFile,
	writeTextFile,
} from "@tauri-apps/plugin-fs";
import { useCallback, useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import EasyMde from "../components/EasyMde";

async function nextUntitledName(): Promise<string> {
	const base = "Untitled";
	let name = `${base}.md`;
	let i = 1;
	while (await exists(name, { baseDir: BaseDirectory.AppData })) {
		name = `${base} ${i}.md`;
		i++;
	}
	return name;
}

export default function Note() {
	const { file } = useParams<{ file: string }>();
	const navigate = useNavigate();
	const [content, setContent] = useState("");

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

	useEffect(() => {
		if (!file) return;
		saveNote(file, content);
	}, [content, file, saveNote]);

	return (
		<div className="p-8 mx-auto max-w-[400px] text-center">
			<h1 className="text-2xl font-bold mb-5 mt-12">{file}</h1>
			<EasyMde value={content} onChange={setContent} />
		</div>
	);
}
