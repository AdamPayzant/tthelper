import { redirect, fail } from "@sveltejs/kit";
import type { Action, ServerLoadEvent, Cookies } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { LoginRequest } from "$lib/user_mgmt_types";

export const load: PageServerLoad = (event: ServerLoadEvent) => {
    const is_authorized = event.locals.authorized;

    if (is_authorized) {
        throw redirect(302, '/home');
    }
};

export const actions: Action = {
    login: async ({ request, fetch, cookies }) => {
        console.log("Attempting login");

        const form_data = await request.formData();
        const username = String(form_data.get('username'));
        const password = String(form_data.get('password'));

        const login_req: LoginRequest = { username, password };

        const res = await fetch('http://localhost:8080/auth', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(login_req)
        });

        if (!res.ok) {
            return fail(401, { error: "Server failed" });
        }

        if (res.headers.has('Set-Cookie')) {

            const data = await res.json();
            cookies.set("Username", username, {
                httpOnly: true,
                path: '/',
                secure: true,
                sameSite: 'strict',
                maxAge: 60 * 60 * 24 * 7,
            });
            cookies.set('AutherizationToken', `Bearer ${data.token}`, {
                httpOnly: true,
                path: '/',
                secure: true,
                sameSite: 'strict',
                maxAge: 60 * 60 * 24 * 7,
            });

            return redirect(302, '/userhome');
        }

    }
}
