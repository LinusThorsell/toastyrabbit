import type { PageLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';

export const load: PageLoad = async () => {
    let tables = await fetch(`${PUBLIC_API_URL}/database/table`);

    return {
        tables: await tables.json()
    };
};
