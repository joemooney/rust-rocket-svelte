<script>
	export let name;
	function toggleName() {
		if (name === 'world') {
			name = 'svelte'
		} else {
			name = 'world'
		}
	}
	async function getServerStuff() {
		console.log("getting server stuff...");
		const res = await fetch('/hello');
		// const res = await fetch('/hello/joe');
		// const text = await res.text();
		console.log("getting server json...");
		// const text = await res.json();
		const text = await res.text();
		console.log("got server json...");

		if (res.ok) {
			console.log("got server ok...", text);
			return text;
		} else {
			console.log("got server error...", text);
			throw new Error(text);
		}
	}
	let promise = getServerStuff();

	function handleClick() {
		promise = getServerStuff();
	}
</script>

<main>
	<h1>3 Hello {name}!</h1>
	<button on:click={toggleName}>Toggle Name</button>
	<button on:click={handleClick}>Download</button>
	{#await promise}
		<p>...waiting</p>
	{:then text}
		<p>The number is {text}</p>
	{:catch error}
		<p style="color: red">{error.message}</p>
	{/await}
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>