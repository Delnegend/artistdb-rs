// const supportedSocials = new Map([
// 	["twitter", "Twitter"],
// 	["telegram", "Telegram"],
// 	["deviantart", "DeviantArt"],
// 	["instagram", "Instagram"],
// 	["dribbble", "Dribbble"],
// 	["duckduckgo", "DuckDuckGo"],
// 	["github", "GitHub"],
// 	["google", "Google"],
// 	["gravatar", "Gravatar"],
// 	["microlink", "Microlink"],
// 	["readcv", "ReadCV"],
// 	["reddit", "Reddit"],
// 	["soundcloud", "SoundCloud"],
// 	["substack", "Substack"],
// 	["youtube", "YouTube"],
// ]);

// const unavatar = {
// 	getAvatarUrl(providedSocials: Array<[string, string]>): string | undefined {
// 		Object.entries(providedSocials).forEach(([provided, uname]) => {
// 			Object.entries(supportedSocials).forEach(([supported, _]) => {
// 				if (provided.toLowerCase().includes(supported)) {
// 					return `https://unavatar.io/${supported}/${uname}?json&size=400`;
// 				}
// 			});
// 		});
// 		return undefined;
// 	},
// 	getSocialsDisplayNames(
// 		providedSocials_: Record<string, string>,
// 	): Map<string, string> {
// 		const providedSocials = Object.keys(providedSocials_).map((social) =>
// 			social.toLowerCase(),
// 		);
// 		const mapped = Object.keys(supportedSocials);
// 		const result: [string, string][] = providedSocials.map((provided) => {
// 			const supported = mapped.find((supported) =>
// 				provided.includes(supported),
// 			);

// 			if (supported === undefined) {
// 				return [provided, ""];
// 			}

// 			const display_name = supportedSocials.get(supported);
// 			if (display_name === undefined) {
// 				return [provided, ""];
// 			}

// 			return [provided, display_name];
// 		});
// 		return new Map(result);
// 	},
// } as const;

// export default unavatar;
