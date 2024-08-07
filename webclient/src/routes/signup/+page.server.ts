import { redirect, type Action, fail, type ServerLoadEvent } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

import { DATA_SERVER_ADDRESS } from '$env/static/private';

export const load: PageServerLoad = (event: ServerLoadEvent) => {
	const user = event.locals.user;

	if (user) {
		throw redirect(302, '/home');
	}
};

export const actions: Action = {
	default: async (event) => {
		const form_data = Object.fromEntries(await event.request.formData());

		if (!form_data.username || !form_data.password) {
			return fail(400, {
				error: 'Missing email or password'
			});
		}

		const { username, password } = form_data as { username: string; password: string };
		const res = await fetch(`${DATA_SERVER_ADDRESS}/user`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});
		if (res.status != 200) {
			console.log('Error loading');
			console.log(res);
			return fail(401, {
				error: 'Server failed'
			});
		}

		throw redirect(302, '/login');
	}
};
