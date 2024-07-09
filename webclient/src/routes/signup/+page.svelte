<script lang="ts">
	import type { ActionData } from './$types';
	import { enhance } from '$app/forms';

	export let form: ActionData;
</script>

<svelte:head>
	<title>Sign Up</title>
</svelte:head>

<h1>Sign Up</h1>

<section>
	<form method="post" use:enhance>
		<div>
			<label for="username">Username</label>
			<input type="username" name="username" id="username" required />
		</div>
		<div>
			<label for="password">Password</label>
			<input type="password" name="password" id="password" required />
		</div>

		<div>
			<button type="submit">Register</button>
		</div>
		{#if form?.error}
			<div class="notice error">
				{form.error}
			</div>
		{/if}
	</form>
	<div>
		<a href="/login">Log in</a>
	</div>
</section>
