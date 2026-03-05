import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { getAllTags, getNotesWithTag } from "../db";
import { trimExtensionName } from "../utils";

export default function Tags() {
	const [tagMap, setTagMap] = useState<Record<string, string[]>>({});

	useEffect(() => {
		async function load() {
			const tags = await getAllTags();
			const unique = [...new Set(tags)];
			const entries: Record<string, string[]> = {};
			for (const tag of unique) {
				entries[tag] = await getNotesWithTag(tag);
			}
			setTagMap(entries);
		}
		load();
	}, []);

	const tagNames = Object.keys(tagMap);

	return (
		<div className="p-8 mx-auto max-w-[500px] text-fuji-white">
			<h1 className="text-2xl font-bold mb-6 mt-12 text-center">Tags</h1>
			{tagNames.length === 0 ? (
				<p className="text-fuji-gray text-center text-sm">No tags yet.</p>
			) : (
				<div className="flex flex-col gap-4">
					{tagNames.map((tag) => (
						<div key={tag} className="bg-sumi-ink-0 rounded-lg p-4">
							<h2 className="text-spring-blue font-semibold mb-2">{tag}</h2>
							<ul className="flex flex-col gap-1">
								{tagMap[tag].map((file) => (
									<li key={file}>
										<Link
											to={`/note/${file}`}
											className="text-sm text-old-white hover:text-spring-blue hover:underline"
										>
											{trimExtensionName(file)}
										</Link>
									</li>
								))}
							</ul>
						</div>
					))}
				</div>
			)}
		</div>
	);
}
