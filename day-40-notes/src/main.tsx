import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { MemoryRouter } from "react-router-dom";
import App from "./App";
import "./styles.css";
import { ensureAppDirExists } from "./utils";
import "./menubar.ts";
import "./shortcuts.ts";
import "./db.ts";

createRoot(document.getElementById("app")!).render(
	<StrictMode>
		<MemoryRouter>
			<App />
		</MemoryRouter>
	</StrictMode>,
);

ensureAppDirExists();
