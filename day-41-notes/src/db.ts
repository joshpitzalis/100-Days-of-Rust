import Database from "@tauri-apps/plugin-sql";

let dbPromise: Promise<Database> | null = null;

function getDb() {
	if (!dbPromise) {
		dbPromise = Database.load("sqlite:db.sqlite");
	}
	return dbPromise;
}

interface TagRecord {
	tag: string;
	file_name: string;
}

export async function getAllTags() {
	const db = await getDb();
	const result: TagRecord[] = await db.select(
		"select distinct tag, file_name from tags",
	);

	return result.map((record) => record.tag);
}

export async function getNotesWithTag(tag: string) {
	const db = await getDb();
	const result: TagRecord[] = await db.select(
		"select * from tags where tag = $1",
		[tag],
	);

	return result.map((record) => record.file_name);
}

export async function getTagsForNote(fileName: string) {
	const db = await getDb();
	const result: TagRecord[] = await db.select(
		"select * from tags where file_name = $1",
		[fileName],
	);

	return result.map((record) => record.tag);
}

export async function insertTag(tag: string, fileName: string) {
	const db = await getDb();
	await db.execute("insert into tags (tag, file_name) values ($1, $2)", [
		tag,
		fileName,
	]);
}

export async function deleteTag(tag: string, fileName: string) {
	const db = await getDb();
	await db.execute("delete from tags where tag = $1 and file_name = $2", [
		tag,
		fileName,
	]);
}
