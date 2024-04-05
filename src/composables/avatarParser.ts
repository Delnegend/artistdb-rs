export function avatarParser(original: string): string {
    if (original.startsWith("_")) {
        return "/avatar.svg"
    }
    if (original.startsWith("/")) {
        return original
    }
    return `https://unavatar.io/${original}?size=400&fallback=https://artistdb.delnegend.com/avatar.svg`
}