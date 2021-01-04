<script>
	import Comment from "./Comment.svelte";
	import EditForm from './EditForm.svelte';
	let signed_in = false;
	if (getCookie("username") != "") {
		signed_in = true;
	} else {
		signed_in = false;
	}
	export let item;
	export let returnTo;
	export let descendents = []
	descendents = item.descendents;
	console.log(item.descendents.length);

	let cats = [
		{ id: 'J---aiyznGQ', name: 'Keyboard Cat' },
		{ id: 'z_AbfPXTKms', name: 'Maru' },
		{ id: 'OUtn3pvWmpg', name: 'Henri The Existential Cat' }
	];
	console.log(cats);
	console.log(item.descendents);


	$: url = !item.item.url ? `https://gthackerhome.github.io/item/${item.item.id}` : item.item.url;

	function calc_age() {
		const start = Math.floor(Date.now() / 1000);
		const elapsed_time = (start - item.item.time);
		if (elapsed_time < 60) {
			return `${'just now'}`
		}
		if (elapsed_time < 3600) {
			return `${Math.floor(elapsed_time / 60)} ${'minutes ago'}`
		}
		if (elapsed_time < 86400) {
			return `${Math.floor(elapsed_time / 3600)} ${'hours ago'}`
		}
		else {
			return `${Math.floor(elapsed_time / 86400)} ${'days ago'}`
		}
		return `${start - item.item.time}`;
	}


</script>

<style>
	article {
		margin: 0 0 1em 0;
	}

	a {
		display: block;
		margin: 0 0 1em 0;
	}

	h1 {
		font-size: 1.4em;
		margin: 0;
	}
</style>

<a href=https://gthackerhome.github.io>&laquo; back</a>

<article>
	<a href="{url}">
		<h1>{item.item.title}</h1>
		{#if item.item.url}
			<small>{item.item.url}</small>
		{/if}
	</a>
	<p>{item.item.text}</p>

	<p class="meta">submitted by {item.item.author} {calc_age()}
	{#if signed_in} 
		<EditForm parentid = {item.item.id} />
	{/if}
</article>

<div class="comments">
	{#each item.descendents as comment}
		 <Comment {comment}/> 
	{/each}
</div> 
