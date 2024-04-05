export async function fetchUserInfo(username: string): Promise<string> {
    try {
        const res = await fetch(`/artists/${username}`);
        if (!res.ok) {
            return new Promise((_, reject) => reject("error"));
        }
        const content = await res.text();
        if (content.startsWith("<")) {
            return new Promise((_, reject) => reject("error"));
        }
        if (content.startsWith("@")) {
            const res = await fetch(`/artists/${content.slice(1)}`);
            return await res.text();
        }
        return new Promise((resolve) => resolve(content));
    } catch {
        return new Promise((_, reject) => reject("error"));
    }
}