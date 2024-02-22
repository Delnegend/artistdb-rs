// https://nuxt.com/docs/api/configuration/nuxt-config
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineNuxtConfig({
	srcDir: "src",
	ssr: false,
	app: {
		baseURL: "/",
		buildAssetsDir: "assets",
	},
	devtools: { enabled: true },
	modules: ["@nuxtjs/tailwindcss", "@nuxtjs/eslint-module"],
	css: ["~/assets/css/main.css"],
	eslint: { lintOnStart: false },
	experimental: { viewTransition: true },
	vite: {
		plugins: [
			wasm(),
			topLevelAwait({
				// The export name of top-level await promise for each chunk module
				promiseExportName: "__tla",
				// The function to generate import names of top-level await promise in each chunk module
				promiseImportName: i => `__tla_${i}`
			})
		]
	},
	nitro: {
		experimental: {
			wasm: true,
		},
	},
});
