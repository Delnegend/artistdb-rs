import { parse } from "smol-toml";
import { openDB } from "idb";
import type { GetArtistResponse } from "./GetArtistResponseType";

type ArtistList = Record<string, Record<string, string>>;

self.onmessage = async (artistName: MessageEvent<string>) => {
	// Open IndexedDB
	const db = await openDB("main", undefined, {
		upgrade(db) {
			db.createObjectStore("config");
			db.createObjectStore("artists");
		},
	});

	// Fetch artist from IndexedDB first
	try {
		const artist: Record<string, string> | undefined = await db.get(
			"artists",
			artistName.data,
		);
		let msg: GetArtistResponse = {
			success: true,
			data: artist,
		};
		self.postMessage(msg);
	} catch (error) {
		let msg: GetArtistResponse = {
			success: false,
			message: `Failed to fetch artist from IndexedDB: ${error}`,
		};
		self.postMessage(msg);
	}

	// Request artists.hash
	const responseHash = await fetch("/artists.hash");
	if (!responseHash.ok) {
		let msg: GetArtistResponse = {
			success: false,
			message: `Failed to fetch artists.hash: ${responseHash.status} ${responseHash.statusText}`,
		};
		self.postMessage(msg);
		return;
	}

	// Read artists.hash
	let currentHash: string;
	try {
		currentHash = await responseHash.text();
	} catch (error) {
		let msg: GetArtistResponse = {
			success: false,
			message: `Failed to read artists.hash: ${error}`,
		};
		self.postMessage(msg);
		return;
	}
	const lastHash = await db.get("config", "hash");

	// Check if IndexedDB is up to date
	if (currentHash === lastHash) {
		let msg: GetArtistResponse = {
			success: true,
			needRefresh: false,
		};
		self.postMessage(msg);
		return;
	}

	// Fetch artists.toml
	const responsToml = await fetch("/artists.toml");
	if (!responsToml.ok) {
		let msg: GetArtistResponse = {
			success: false,
			message: `Failed to fetch artists.toml: ${responsToml.status} ${responsToml.statusText}`,
		};
		self.postMessage(msg);
		return;
	}

	// Read artists.toml
	let artistsString: string;
	try {
		artistsString = await responsToml.text();
	} catch (error) {
		let msg: GetArtistResponse = {
			success: false,
			message: `Failed to read artists.toml: ${error}`,
		};
		self.postMessage(msg);
		return;
	}

	// Decode artists.toml
	let artistsList: ArtistList;
	try {
		artistsList = parse(artistsString) as ArtistList;
	} catch (error) {
		let msg: GetArtistResponse = {
			success: false,
			message: `Failed to parse artists.toml: ${error}`,
		};
		self.postMessage(msg);
		return;
	}

	// Update IndexedDB
	await db.put("config", currentHash, "hash");
	await db.clear("artists");
	for (const [name, info] of Object.entries(artistsList)) {
		await db.put("artists", info, name);
	}

	// Get newest data
	let artist: Record<string, string> | undefined = await db.get(
		"artists",
		artistName.data,
	);
	let msg: GetArtistResponse = {
		success: true,
		data: artist,
		needRefresh: true,
	};
	self.postMessage(msg);
};
