import { useEffect, useState } from "react";
import { Route, Routes } from "react-router-dom";
import FuzzyFinder from "./components/FuzzyFinder";
import SideBar from "./components/SideBar";
import Home from "./pages/Home";
import Note from "./pages/Note";
import Tags from "./pages/Tags";

export default function App() {
	const [showFuzzyFinder, setShowFuzzyFinder] = useState(false);

	useEffect(() => {
		function handleKeyDown(event: KeyboardEvent) {
			const isCmdOrCtrl = event.metaKey || event.ctrlKey;

			if (isCmdOrCtrl && event.key.toLowerCase() === "p") {
				setShowFuzzyFinder((prev) => !prev);
				event.preventDefault();
			}

			if (event.key === "Escape" && showFuzzyFinder) {
				setShowFuzzyFinder(false);
				event.preventDefault();
			}
		}

		document.addEventListener("keydown", handleKeyDown);
		return () => document.removeEventListener("keydown", handleKeyDown);
	}, [showFuzzyFinder]);

	return (
		<div className="flex relative">
			<SideBar />
			<main className="w-full">
				<Routes>
					<Route path="/" element={<Home />} />
					<Route path="/note/:file" element={<Note />} />
					<Route path="/tags" element={<Tags />} />
				</Routes>
			</main>
			{showFuzzyFinder && (
				<FuzzyFinder onSelected={() => setShowFuzzyFinder(false)} />
			)}
		</div>
	);
}
