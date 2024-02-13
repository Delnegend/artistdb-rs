type GetArtistResponse = {
	success: boolean;
	message?: string;
	data?: Record<string, string>;
	needRefresh?: boolean;
};

export type { GetArtistResponse };
