import {
	BaseDirectory,
	readTextFile,
	writeTextFile,
} from "@tauri-apps/plugin-fs";
import { useCallback, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import EasyMde from "../components/EasyMde";

export default function Note() {
	const { file } = useParams<{ file: string }>();
	const [content, setContent] = useState("");

	useEffect(() => {
		if (!file) return;
		readTextFile(file, { baseDir: BaseDirectory.AppData }).then(setContent);
	}, [file]);

	const saveNote = useCallback(async (file: string, content: string) => {
		if (!file) return;
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
