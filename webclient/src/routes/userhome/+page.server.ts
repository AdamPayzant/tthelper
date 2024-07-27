import type { PageServerLoad } from './$types';
import type { Action } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async (event) => {
	const auth_token = event.cookies.get('AutherizationToken');
	const user = event.cookies.get('Username');

	try {
		if (auth_token && user) {
			const token = auth_token.split(' ')[1];

			let res = await fetch(`http://localhost:8080/character`, {
				method: 'GET',
				headers: {
					user: user,
					access_token: token
				}
			});
			if (res.status == 401) {
				// If authentication failed, put the user back to the login screen
				event.locals.authorization = null;
				throw redirect(302, '/login');
			} else if (res.status != 200) {
				return 'Failed to get character data';
			}

			return { character_list: await res.json() };
		}
	} catch (e) {
		console.error(e);
	}
};

export const actions: Action = {
	new_character: async (event) => {
		console.log('In new character action');
		const form_data = Object.fromEntries(await event.request.formData());
	}
};
