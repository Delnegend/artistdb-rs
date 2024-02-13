// https://nuxt.com/docs/api/configuration/nuxt-config
import wasm from "vite-plugin-wasm";

export default defineNuxtConfig({
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
		plugins: [wasm()]
	}
});
