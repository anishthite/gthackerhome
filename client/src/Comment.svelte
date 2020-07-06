<script>
	export let comment;
	import EditForm from './EditForm.svelte';
	let signed_in = false;
	if (getCookie("username") != "") {
		signed_in = true;
	} else {
		signed_in = false;
	}


	function calc_age() {
		const start = Math.floor(Date.now() / 1000);
		const elapsed_time = (start - comment.item.time);
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
		return `${start - comment.item.time}`;
	}

	function getCookie(cname) {
		var name = cname + "=";
		var decodedCookie = decodeURIComponent(document.cookie);
		var ca = decodedCookie.split(';');
		for(var i = 0; i <ca.length; i++) {
		  var c = ca[i];
		  while (c.charAt(0) == ' ') {
			c = c.substring(1);
		  }
		  if (c.indexOf(name) == 0) {
			return c.substring(name.length, c.length);
		  }
		}
		return "";
	  }

</script>

<style>
	article {
		border-top: 1px solid #eee;
		margin: 1em 0 0 0;
		padding: 1em 0 0 0;
		font-size: 14px;
	}

	.meta {
		color: #999;
	}

	.replies {
		padding: 0 0 0 3em;
	}
    .fake-link {
    color: #999;
    text-decoration: underline;
    cursor: pointer;
    }


</style>

<article>
	<p class="meta">{comment.item.author} {calc_age()}</p>

	{@html comment.item.text}
	{#if signed_in} 
		<EditForm parentid = {comment.item.id} />
	{/if}	
	<div class="replies">
		{#each comment.descendents as child}
			<svelte:self comment={child}/>
		{/each}
	</div>
</article>