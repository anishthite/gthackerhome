<script>
	export let item;
	export let i;
	export let offset;

	function comment_text() {
		const c = item.descendents;
		return `${c} ${c === 1 ? 'comment' : 'comments'}`;
	}

	function calc_age() {
		const start = Math.floor(Date.now() / 1000);
		const elapsed_time = (start - item.time);
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
		return `${start - item.time}`;
	}


	$: url = item.type === "ask" ? `https://news.ycombinator.com/${item.url}` : item.url;
</script>

<style>
	article {
		position: relative;
		padding: 0 0 0 2em;
		border-bottom: 1px solid #eee;
	}

	h2 {
		font-size: 1em;
		margin: 0.5em 0;
	}

	span {
		position: absolute;
		left: 0;
	}

	a {
		color: #333;
	}
</style>

<article>
	<span>{i + offset + 1}</span>
	<h2><a target="_blank" href={url}>{item.title}</a></h2>
	<p class="meta"><a href="#/item/{item.id}">{comment_text()}</a> by {item.author} {calc_age()}</p>
</article>
