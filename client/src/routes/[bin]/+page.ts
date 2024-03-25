import type { PageLoad } from './$types';
import { getBinByKey } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params, fetch }) => {
	try {
		return await getBinByKey(params.bin, fetch);
	} catch {
		error(404, 'Bin not found');
	}
}