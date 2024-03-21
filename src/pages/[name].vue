<script setup lang="ts">
import { useRoute } from "vue-router";
import { Artist, get_alias as getAlias } from "~/composables/bridge";

type NetworkStatus = "error" | "loaded" | "loading";

const router = useRoute();
const networkStatus = ref<NetworkStatus>("loading");

const username = ref<string>("");
const artistInfo = ref<Artist | undefined>(undefined);
const avatar = ref<string>("");

if (Array.isArray(router.params.name)) {
	username.value = router.params.name[0].toLowerCase().trim();
} else {
	username.value = router.params.name.toLowerCase().trim();
}

(async () => {
	try {
		const res = await fetch(`/artists/${username.value}`);
		const body = new Uint8Array(await res.arrayBuffer());

		const artistFromBitcode = Artist.from_bitcode(body);

		if (artistFromBitcode !== undefined) {
			artistInfo.value = artistFromBitcode;
			networkStatus.value = "loaded";
			return;
		}

		const alias = getAlias(body);

		if (alias === undefined) {
			networkStatus.value = "error";
			return;
		}

		const resAlias = await fetch(`/artists/${alias}`);
		const bodyAlias = new Uint8Array(await resAlias.arrayBuffer());

		const artistFromAliasBitcode = Artist.from_bitcode(bodyAlias);

		if (artistFromAliasBitcode !== undefined) {
			artistInfo.value = artistFromAliasBitcode;
			networkStatus.value = "loaded";
			return;
		}

		networkStatus.value = "error";
	} catch {
		networkStatus.value = "error";
	}
})().catch(() => {
	networkStatus.value = "error";
});

watchEffect(() => {
	if (artistInfo.value !== undefined) {
		avatar.value = artistInfo.value.avatar ?? "/avatar.svg";
	}

	document.title = `${artistInfo.value?.name ?? username.value} | Artist DB`;
});

const avatarLoaded = ref(false);

/** */
</script>

<template>
	<div class="fixed -z-10 h-screen w-full scale-125 bg-black blur-2xl brightness-50">
		<img v-show="avatarLoaded" :src="avatar" class="fixed -z-10 size-full object-cover" />

		<img v-show="!avatarLoaded" src="/avatar.svg" class="fixed -z-10 size-full object-cover" />
	</div>

	<div class="mx-auto max-w-96 py-12" v-if="networkStatus === 'loaded'">
		<!-- avatar -->
		<div class="flex w-full justify-center">
			<img
				v-show="avatarLoaded"
				@load="avatarLoaded = true"
				:src="avatar"
				class="aspect-square w-full max-w-60 rounded-full object-cover shadow-2xl"
			/>
			<div
				v-show="!avatarLoaded"
				class="aspect-square w-full max-w-60 animate-pulse rounded-full bg-black shadow-2xl"
			/>
		</div>

		<div
			class="flex w-full flex-row items-center justify-center gap-3 py-7 text-center text-3xl font-bold text-white"
		>
			<div>{{ artistInfo?.name ?? username }}</div>
			<Flag v-if="artistInfo?.flag !== undefined">{{ artistInfo?.flag }}</Flag>
		</div>

		<!-- links -->
		<div class="px-1rem flex w-full flex-col gap-3">
			<a
				v-for="social in artistInfo?.socials"
				:key="social.code"
				:href="social.link"
				target="_blank"
				class="both flex w-full justify-center px-6 py-3 text-lg hover:font-bold"
				:class="social.special === true ? 'special-link' : 'normal-link'"
				>{{ social.desc }}</a
			>
		</div>
	</div>

	<div
		v-if="networkStatus === 'error'"
		class="flex h-screen flex-col items-center justify-center gap-5"
	>
		<span class="text-5xl">🤷</span
		><span class="text-xl text-white/85">Artist not found in database</span>
	</div>
</template>

<style scoped>
.both {
	transition-property: background, color, border, font-weight;
	transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
	transition-duration: 150ms;
}

.normal-link {
	@apply text-white/60 hover:bg-black hover:text-white hover:border-black border-4 border-solid border-white/20;
}

.special-link {
	@apply text-black/70 shadow-2xl;

	background: linear-gradient(323deg, #ff7777, #e3ff00, #00ff42, #73d9ff, #fd00ff);
	background-size: 200% 200%;
	background-position: left center;
}

.special-link:hover {
	background-position: right center;
}
</style>
