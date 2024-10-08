import { error } from '@sveltejs/kit';
import { DATA_SERVER_ADDRESS } from '$env/static/private';

export async function GET({ params, request, cookies }): Promise<Response> {
	const auth_token = cookies.get('AutherizationToken');
	const user = cookies.get('Username');
	if (!auth_token || !user) {
		error(401, 'Authorization has failed unexepectedly');
	}
	const token = auth_token.split(' ')[1];

	const res = await fetch(`${DATA_SERVER_ADDRESS}/character/${params.cid}`, {
		method: 'GET',
		headers: {
			user: user,
			access_token: token
		}
	});
	console.log(res);

	if (res.status != 200) {
		console.error(res.status);
		error(res.status, await res.json());
	}
	const json = await res.json();

	console.log(json);
	return new Response(JSON.stringify(json));
}
