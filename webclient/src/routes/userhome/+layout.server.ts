import type { Actions, PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = (event) => {
    const authorization = event.locals.authorization;

    if (!authorization || !authorization.authorized) {
        console.log("User not authorized, redirecting");
        throw redirect(302, '/login');
    }

    return {
        authorization
    };
};
