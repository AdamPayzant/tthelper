import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = (event) => {
	const authorization = event.locals.authorization;

	if (!authorization || !authorization.authorized) {
		console.log('User not authorized, redirecting');
		throw redirect(302, '/login');
	} else {
		throw redirect(302, '/userhome');
	}
};
