import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = (event) => {
	console.log('Verifying authorization');
	const authorization = event.locals.authorization;

	if (!authorization || !authorization.authorized) {
		console.log('User not authorized, redirecting');
		throw redirect(302, '/login');
	}

	return {
		authorization
	};
};
