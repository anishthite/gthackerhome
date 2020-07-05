<script>
	import { beforeUpdate } from "svelte";
	import Summary from "./Summary.svelte";

	const PAGE_SIZE = 40;

	export let page;

	let items;
	let offset;
	let postsize

	$: fetch(`https://greetez.com:4343/item_api/posts`)
		.then(r => r.json())
		.then(data => {
			postsize = data.length;
			items = data.slice(PAGE_SIZE * (page-1), PAGE_SIZE * (page-1) + PAGE_SIZE);
			offset = PAGE_SIZE * (page - 1);
			window.scrollTo(0, 0);
			console.log(data)
		});
</script>

<style>
	a {
		padding: 2em;
		display: block;
	}

	.loading {
		opacity: 0;
		animation: 0.4s 0.8s forwards fade-in;
	}

	@keyframes fade-in {
		from { opacity: 0; }
		to { opacity: 1; }
	}
</style>

{#if items}
	{#each items as item, i}
		<Summary {item} {i} {offset}/>
	{/each}

	{#if PAGE_SIZE * (page-1) + PAGE_SIZE < postsize}
		<a href="#/top/{page + 1}">page {page + 1}</a>
	{/if}	
{:else}
	<p class="loading">loading...</p>
{/if}