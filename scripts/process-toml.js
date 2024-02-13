import murmur from "murmurhash-js";
import fs from 'fs/promises';
import { watch } from 'chokidar';
import { parse, stringify } from 'smol-toml'

const TOML_FILE_PATH = "public/artists.toml";
const HASH_FILE_PATH = "public/artists.hash";
const WRITE_FILE = process.argv.includes("--write");

/**
 * @typedef {Object.<string, Record<string, string>>} ArtistList
 */

/** Cycle sort, only for prints steps to take to sort the item in toml
 *
 * @param {string[]} arr - Array to sort
 * @returns {void}
*/
function softFormat(arr) {
	const n = arr.length;
	let cycles = 0;
	for (let cycleStart = 0; cycleStart < n - 1; cycleStart++) {
		let item = arr[cycleStart];
		let pos = cycleStart;
		for (let i = cycleStart + 1; i < n; i++) {
			if (arr[i] < item) pos++;
		}
		if (pos === cycleStart) continue;
		while (item === arr[pos]) pos++;
		if (pos !== cycleStart) {
			[item, arr[pos]] = [arr[pos], item];
			console.log(`⚠️  ${item} ↔️  ${arr[pos]}`);
			cycles++;
		}
		while (pos !== cycleStart) {
			pos = cycleStart;
			for (let i = cycleStart + 1; i < n; i++) {
				if (arr[i] < item) pos++;
			}
			while (item === arr[pos]) pos++;
			if (pos !== cycleStart) {
				[item, arr[pos]] = [arr[pos], item];
				console.log(`⚠️  ${item} ↔️  ${arr[pos]}`);
				cycles++;
			}
		}
	}
};

/**
 * Sorts the ArtistList object by key
 *
 * @param {ArtistList} artistList - The artist list to sort
 * @returns {ArtistList}
 */
function hardFormat(artistList) {
	return Object.fromEntries(
		Object.entries(artistList).sort(([a], [b]) => a.localeCompare(b))
	);
}

/**
 * @returns {Promise<string>}
 */
async function main() {
	/** @type {string} */
	const artistString = (await fs.readFile(TOML_FILE_PATH, "utf8")).toString();

	/** @type {ArtistList} */
	let artistObject;
	try {
		artistObject = parse(artistString)
	} catch {
		console.error("Error parsing toml file");
		return 0;
	}

	/** @type {number} */
	if (!WRITE_FILE) {
		softFormat(Object.keys(artistObject));
		let artistHash = murmur.murmur3(artistString);
		return artistHash;
	}

	let sortedArtistObject = hardFormat(artistObject);
	let sortedArtistString = stringify(sortedArtistObject);
	let artistHash = murmur.murmur3(sortedArtistString);

	if (artistString !== sortedArtistString) {
		setTimeout(async () => {
			await fs.writeFile(TOML_FILE_PATH, sortedArtistString, "utf8");
		}, 500);
	}

	return artistHash;
}

main().catch(console.error);

if (process.argv.includes("--watch")) {
	let lastHash;
	watch(TOML_FILE_PATH).on("change", async () => {
		/** @type {number} */
		let newHash = await main()

		if (newHash !== lastHash) {
			lastHash = newHash;
			await fs.writeFile(HASH_FILE_PATH, newHash.toString());
			console.log(`New hash for '${TOML_FILE_PATH}': ${newHash}`);
		}
	});
}