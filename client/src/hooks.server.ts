import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
    event.locals.authorization = null;

    console.log("Checking auth token");
    const auth_token = event.cookies.get('AutherizationToken');
    const user = event.cookies.get("Username");

    if (auth_token && user) {
        const token = auth_token.split(' ')[1];

        const res = await fetch(`http://localhost:8080/auth/${user}/${token}`, {
            method: 'GET',
        });

        if (res.ok) {
            console.log("Check passed")
            event.locals.authorization = { name: user, authorized: true };
        }
    }

    return await resolve(event);
}
