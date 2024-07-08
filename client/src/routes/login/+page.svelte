<script lang="ts">
	import type { ActionData } from './$types';
	import { enhance } from '$app/forms';

	export let form: ActionData;
</script>

<svelte:head>
	<title>Sign In</title>
</svelte:head>

<h1>Sign In</h1>

<section>
	<form method="post" action="?/login" use:enhance>
		<div>
			<label for="username">Username</label>
			<input type="username" name="username" id="username" placeholder="Username" required />
		</div>
		<div>
			<label for="password">Password</label>
			<input type="password" name="password" id="password" placeholder="Password" required />
		</div>

		<div>
			<button type="submit">Login</button>
		</div>
		{#if form?.error}
			<div class="notice error">
				{form.error}
			</div>
		{/if}
	</form>
	<div>
		<a href="/signup">Sign up</a>
	</div>
</section>
