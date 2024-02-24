<script setup lang="ts">
import { useRoute } from "vue-router";
import { Artist, get_alias } from "~/composables/bridge";

type NetworkStatus = "loading" | "loaded" | "error";

const router = useRoute();
const networkStatus = ref<NetworkStatus>("loading");

const username = ref<string>("");
const artistInfo = ref<Artist | undefined>(undefined);
const avatar = ref<string>("");

if (Array.isArray(router.params.name)) {
	username.value = router.params.name[0].toLowerCase();
} else {
	username.value = router.params.name.toLowerCase();
}

(async () => {
	try {
		const res = await fetch(`/artists/${username.value}`);
		const body = new Uint8Array(await res.arrayBuffer());
		artistInfo.value = Artist.from_bitcode(body);

		if (artistInfo.value !== undefined) {
			networkStatus.value = "loaded";
			return;
		}

		const alias = get_alias(body);
		if (alias === undefined) {
			networkStatus.value = "error";
			return;
		}

		const res_alias = await fetch(`/artists/${alias}`);
		const body_alias = new Uint8Array(await res_alias.arrayBuffer());

		artistInfo.value = Artist.from_bitcode(body_alias);
		if (artistInfo.value !== undefined) {
			networkStatus.value = "loaded";
			return;
		}

		networkStatus.value = "error";
	} catch {
		networkStatus.value = "error";
	}
})();

watchEffect(() => {
	if (artistInfo.value !== undefined) {
		avatar.value = artistInfo.value.avatar ?? "/avatar.svg";
	}
	document.title = `${artistInfo?.value?.name ?? username.value} | Artist DB`;
});
</script>

<template>
	<div
		class="fixed -z-10 h-[100vh] w-full scale-125 bg-black bg-cover bg-center bg-no-repeat blur-2xl brightness-50"
		:style="{ 'background-image': `url(${avatar})` }" />

	<div class="mx-auto max-w-96 py-12" v-if="networkStatus === 'loaded'">
		<!-- avatar -->
		<div class="flex w-full justify-center">
			<img
				:src="avatar"
				class="aspect-square w-full max-w-60 rounded-full object-cover shadow-2xl" />
		</div>

		<div
			class="flex w-full flex-row items-center justify-center gap-3 py-7 text-center text-3xl font-bold text-white">
			<div>{{ artistInfo?.name ?? username }}</div>
			<Flag v-if="artistInfo?.flag !== undefined">{{
				artistInfo?.flag
			}}</Flag>
		</div>

		<!-- links -->
		<div class="px-1rem flex w-full flex-col gap-3">
			<a
				v-for="social in artistInfo?.socials"
				:key="social.code"
				:href="social.link"
				target="_blank"
				class="flex w-full justify-center border-4 border-solid border-black px-6 py-3 text-lg text-white/60 transition-colors hover:bg-black hover:text-white">{{ social.desc }}</a>
		</div>
	</div>

	<div
		v-if="networkStatus === 'error'"
		class="flex h-[100vh] flex-col items-center justify-center gap-5">
		<span class="text-5xl">🤷</span><span class="text-xl text-white/85">Artist not found in database</span>
	</div>
</template>
