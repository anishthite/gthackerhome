<script>
	import { onMount } from 'svelte';
	import List from './List.svelte';
	import Item from './Item.svelte';
	let item;
	let page;

	async function hashchange() {
		// the poor man's router!
		const path = window.location.hash.slice(1);

		if (path.startsWith('/item')) {
			const id = path.slice(6);
			item = await fetch(`https://brrrr.live:4343/item_api/${id}`).then(r => r.json());

			window.scrollTo(0,0);
		} else if (path.startsWith('/top')) {
			page = +path.slice(5);
			item = null;
		} else {
			window.location.hash = '/top/1';
		}
	}
	onMount(hashchange);
	window.onload = showUsername();
</script>

<style>
	main {
		position: relative;
		max-width: 800px;
		margin: 0 auto;
		min-height: 101vh;
		padding: 1em;
	}

	main :global(.meta) {
		color: #999;
		font-size: 12px;
		margin: 0 0 1em 0;
	}

	main :global(a) {
		color: rgb(0,0,150);
	}
	.topnav {
	  overflow: hidden;
	  background-color: #444;
	}

	.topnav a {
  		float: left;
  		color: #f2f2f2;
  		text-align: center;
  		padding: 14px 16px;
  		text-decoration: none;
  		font-size: 17px;
	}

	.topnav a:hover {
  		background-color: #999;
  		color: black;
	}

	.topnav a.active {
  		background-color: #984b43;
  		color: white;
	}
</style>

<svelte:window on:hashchange={hashchange}/>


<main>
	{#if item}
		<Item {item} returnTo="#/top/{page}"/>
	{:else if page}
		<List {page}/>
	{/if}
</main>
