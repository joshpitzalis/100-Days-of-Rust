import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { trimExtensionName } from "../utils";

export default function FuzzyFinder({
	onSelected,
}: {
	onSelected: () => void;
}) {
	const [query, setQuery] = useState("");
	const [searchResults, setSearchResults] = useState<string[]>([]);
	const [selectedIndex, setSelectedIndex] = useState(0);
	const inputRef = useRef<HTMLInputElement>(null);
	const navigate = useNavigate();

	useEffect(() => {
		inputRef.current?.focus();
	}, []);

	async function search(value: string) {
		setQuery(value);
		setSelectedIndex(0);
		const results = await invoke<string[]>("find", { query: value });
		setSearchResults(results);
	}

	function selectResult(hit: string) {
		navigate("/note/" + hit);
		onSelected();
	}

	function handleKeyDown(e: React.KeyboardEvent) {
		if (e.key === "ArrowDown") {
			e.preventDefault();
			setSelectedIndex((i) => Math.min(i + 1, searchResults.length - 1));
		} else if (e.key === "ArrowUp") {
			e.preventDefault();
			setSelectedIndex((i) => Math.max(i - 1, 0));
		} else if (e.key === "Enter") {
			e.preventDefault();
			if (searchResults.length > 0) {
				selectResult(searchResults[selectedIndex]);
			}
		}
	}

	return (
		<div className="absolute top-0 left-0 w-screen h-screen bg-black/60 backdrop-blur-sm text-white z-50">
			<div className="max-w-[600px] mt-10 mx-auto">
				<input
					ref={inputRef}
					type="text"
					value={query}
					onChange={(e) => search(e.target.value)}
					onKeyDown={handleKeyDown}
					className="text-2xl p-2 w-full bg-sumi-ink-4 rounded-lg"
				/>
				<ul className="flex flex-col text-lg bg-sumi-ink-5 rounded-b-lg">
					{searchResults.map((hit, index) => (
						<li key={hit}>
							<Link
								to={`/note/${hit}`}
								onClick={onSelected}
								className={`block w-full py-2 px-3 hover:bg-winter-blue hover:text-spring-blue ${index === selectedIndex ? "bg-winter-blue text-spring-blue" : ""}`}
							>
								{trimExtensionName(hit)}
							</Link>
						</li>
					))}
				</ul>
			</div>
		</div>
	);
}
