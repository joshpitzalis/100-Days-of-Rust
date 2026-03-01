import { Routes, Route } from "react-router-dom";
import SideBar from "./components/SideBar";
import Home from "./pages/Home";

export default function App() {
  return (
    <div className="flex relative">
      <SideBar />
      <main className="w-full">
        <Routes>
          <Route path="/" element={<Home />} />
        </Routes>
      </main>
    </div>
  );
}
