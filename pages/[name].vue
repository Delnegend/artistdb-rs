<script setup lang="ts">
import { useRoute } from "vue-router";
import { get_avatar_url } from "~/composables/wasm";

const test_social = [
	"twitte|foobar1",
	"facebook|foobar2",
	"insta|foobar3",
	"youtube|foobar4",
	"twitter|foobar5",
]

console.log(get_avatar_url(test_social))

const router = useRoute();
const artistName = router.params.name;

// const worker = new Worker(
// 	new URL("../composables/getArtist.worker.ts", import.meta.url),
// 	{
// 		type: "module",
// 		name: "updateDB",
// 	},
// );
// worker.onmessage = (event) => {
// 	const data: GetArtistResponse = event.data;
// 	console.log(data);
// };
// worker.postMessage(artistName);
</script>

<template></template>
