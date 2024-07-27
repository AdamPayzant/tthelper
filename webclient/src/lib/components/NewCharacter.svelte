<script lang="ts">
	import type { ActionData } from './$types';
	import { enhance } from '$app/forms';

	export let form: ActionData;
</script>

<div>
	<form method="post" action="?/new_character" use:enhance>
		<div>
			<label for="name">Character Name: </label>
			<input type="text" name="name" id="name" required />
		</div>
		<div>
			<label for="alignment">Alignment: </label>
			<input type="text" name="alignment" id="alignment" required />
		</div>
		<div>
			<label for="ancestry">Ancestry: </label>
			<input type="text" name="ancestry" id="ancestry" required />
		</div>
		<div>
			<label for="background">Background: </label>
			<input type="text" name="background" id="background" required />
		</div>
		<div>
			<label for="class">Class: </label>
			<input type="text" name="class" id="class" required />
		</div>
		<div>
			<label for="key_ability">Key Ability: </label>
			<input type="text" name="class" id="class" required />
		</div>

		<div>
			<button type="submit">Submit</button>
			{#if form?.error}
				<div class="notice error">
					{form.error}
				</div>
			{/if}
		</div>
	</form>
</div>
