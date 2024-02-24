// https://nuxt.com/docs/api/configuration/nuxt-config
import wasm from "vite-plugin-wasm";

export default defineNuxtConfig({
	srcDir: "src",
	ssr: false,
	app: {
		baseURL: "/",
		buildAssetsDir: "assets",
	},
	devtools: { enabled: false },
	modules: ["@nuxtjs/tailwindcss"],
	css: ["~/assets/css/main.css"],
	experimental: { viewTransition: true },
	vite: {
		plugins: [wasm()],
		build: {
			target: "esnext",
		}
	},
});
