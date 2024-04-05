<script setup lang="ts">
import { useRoute } from "vue-router";

const router = useRoute();

type NetworkStatus = "error" | "loaded" | "loading";
const networkStatus = ref<NetworkStatus>("loading");

type Social = {
	isSpecial: boolean;
	link: string;
	desc: string;
};

const usernameToFetch = ref<string>("");

if (Array.isArray(router.params.name)) {
	usernameToFetch.value = router.params.name[0].toLowerCase().trim();
} else {
	usernameToFetch.value = router.params.name.toLowerCase().trim();
}

const rawContent = ref<string>("");

const displayName = ref<string>("");
const avatar = ref<string>("");
const socials = ref<Array<Social>>([]);

fetchUserInfo(usernameToFetch.value)
	.then((res) => {
		if (res === "") {
			networkStatus.value = "error";
			return;
		}

		networkStatus.value = "loaded";

		rawContent.value = res;
	})
	.catch(() => {
		networkStatus.value = "error";
	});

watchEffect(() => {
	if (rawContent.value === "") {
		return;
	}

	const lines = rawContent.value.split("\n");

	const firstLine = lines[0].split(",");

	displayName.value = firstLine[0];
	document.title = `${displayName.value} | ArtistDB`;

	const tempAvatar = firstLine[1];

	if (tempAvatar === "_") {
		avatar.value = "/avatar.svg";
	} else {
		avatar.value = `https://unavatar.io/${firstLine[1]}?size=400&fallback=https://artistdb.delnegend.com/avatar.svg`;
	}

	socials.value = lines.slice(1).map((line) => {
		const [link, desc] = line.split(",");

		if (link.startsWith("*")) {
			return { isSpecial: true, link: link.slice(1), desc };
		}

		return { isSpecial: false, link, desc };
	});
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
			class="display-name flex w-full flex-row items-center justify-center gap-3 py-7 text-center text-5xl font-bold text-white"
		>
			{{ displayName }}
		</div>

		<!-- links -->
		<div class="px-1rem flex w-full flex-col gap-3">
			<a
				v-for="social in socials"
				:key="social.link"
				:href="`https:${social.link}`"
				target="_blank"
				class="both flex w-full justify-center px-6 py-3 text-xl hover:font-bold"
				:class="social.isSpecial === true ? 'special-link' : 'normal-link'"
				>{{ social.desc }}</a
			>
		</div>
	</div>

	<div
		v-if="networkStatus === 'error'"
		class="flex h-screen flex-col items-center justify-center gap-5"
	>
		<span class="text-5xl">🤷</span
		><span class="text-3xl text-white/85">Artist not found in database</span>
	</div>
</template>

<style scoped>
.display-name {
	font-family: "Noto Serif Display", "Twemoji Country Flags", sans-serif;
	font-weight: 600;
}

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
