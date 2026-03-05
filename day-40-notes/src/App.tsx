import { Route, Routes } from "react-router-dom";
import SideBar from "./components/SideBar";
import Home from "./pages/Home";
import Note from "./pages/Note";
import Tags from "./pages/Tags";

export default function App() {
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
		</div>
	);
}
