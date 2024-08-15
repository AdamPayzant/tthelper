import { error } from '@sveltejs/kit';
import { DATA_SERVER_ADDRESS } from '$env/static/private';

export async function PUT({ params, request, cookies }): Promise<Response> {
	const auth_token = cookies.get('AutherizationToken');
	const user = cookies.get('Username');
	if (!auth_token || !user) {
		error(401, 'Authorization has failed unexepectedly');
	}
	const token = auth_token.split(' ')[1];

	const res = await fetch(`${DATA_SERVER_ADDRESS}/character/${params.cid}/abilities`, {
		method: 'PUT',
		headers: {
			user: user,
			access_token: token,
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(await request.json())
	});
	console.log(res);

	if (res.status != 200) {
		error(res.status, await res.json());
	}
	const json = await res.json();

	console.log(json);
	return new Response(JSON.stringify(json));
}
