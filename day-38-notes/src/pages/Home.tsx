import { useNavigate } from "react-router-dom";
import { createNote } from "../features/notes/actions";
export default function Home() {
	const navigate = useNavigate();

	return (
		<div className="p-8 mx-auto max-w-[400px] text-center">
			<h1 className="text-2xl font-bold mb-5 mt-12">Notes</h1>
			<div>
				<button
					type="button"
					className="px-4 py-2 bg-oni-violet text-white rounded-lg cursor-pointer"
					onClick={() => createNote(navigate)}
				>
					Create Note
				</button>
			</div>
		</div>
	);
}
