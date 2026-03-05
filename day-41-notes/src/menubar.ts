import { invoke } from "@tauri-apps/api/core";
import { Menu, PredefinedMenuItem, Submenu } from "@tauri-apps/api/menu";
import { type } from "@tauri-apps/plugin-os";
import { createNote } from "./features/notes/actions";

const os = type();

async function initMenubar() {
	const main = [
		{
			id: "new-window",
			text: "New Window",
			action: () => {
				invoke("new_window");
			},
		},
		await PredefinedMenuItem.new({
			item: "CloseWindow",
		}),
		await PredefinedMenuItem.new({
			item: "Quit",
		}),
	];

	let menu: Menu;

	if (os === "macos") {
		menu = await Menu.new({
			items: [
				await Submenu.new({
					id: "main",
					text: "Main",
					items: main,
				}),
				await Submenu.new({
					id: "file",
					text: "File",
					items: [
						{
							id: "new",
							text: "New Note",
							action: () => {
								createNote();
							},
						},
					],
				}),
			],
		});
	} else {
		menu = await Menu.new({
			items: [
				await Submenu.new({
					id: "file",
					text: "File",
					items: [
						{
							id: "new",
							text: "New Note",
							action: () => {
								createNote();
							},
						},
						...main,
					],
				}),
			],
		});
	}

	menu.setAsAppMenu();
}

initMenubar();
