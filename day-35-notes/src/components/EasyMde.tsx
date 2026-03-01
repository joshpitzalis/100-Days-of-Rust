import MDEditor from "@uiw/react-md-editor";
import { useState } from "react";

interface MarkdownEditorProps {
	value?: string;
	onChange?: (value: string) => void;
}

export default function MarkdownEditor({
	value,
	onChange,
}: MarkdownEditorProps) {
	const [internal, setInternal] = useState("");
	const controlled = value !== undefined && onChange !== undefined;

	return (
		<MDEditor
			value={controlled ? value : internal}
			onChange={(val) =>
				controlled ? onChange(val ?? "") : setInternal(val ?? "")
			}
			preview="edit"
			hideToolbar
		/>
	);
}
