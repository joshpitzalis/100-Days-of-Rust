import { BaseDirectory, create } from "@tauri-apps/plugin-fs";
import { useNavigate } from "react-router-dom";

export default function Home() {
	const navigate = useNavigate();
	async function createNote() {
		const file = await create("My Note.md", { baseDir: BaseDirectory.AppData });
		await file.close();
		navigate("/note/My Note.md");
	}
	return (
		<div className="p-8 mx-auto max-w-[400px] text-center">
			<h1 className="text-2xl font-bold mb-5 mt-12">Notes</h1>
			<div>
				<button
					type="button"
					className="px-4 py-2 bg-oni-violet text-white rounded-lg cursor-pointer"
					onClick={() => createNote()}
				>
					Create Note
				</button>
			</div>
		</div>
	);
}
