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
			item = await fetch(`https://node-hnapi.herokuapp.com/item/${id}`).then(r => r.json());

			window.scrollTo(0,0);
		} else if (path.startsWith('/top')) {
			page = +path.slice(5);
			item = null;
		} else {
			window.location.hash = '/top/1';
		}
	}
function showUsername() {
	var username = getCookie("username");
	if (username != "") {
		document.getElementById("signup").style.visibility="hidden";
		document.getElementById("welcomeusername").innerHTML="<a href='https://greetez.com:4343/user/" + username + "'>Welcome " + username + " </a>";
		document.getElementById("welcomeusername").style.visibility="visible";
	} else {
		document.getElementById("signup").style.visibility="visible";
		document.getElementById("welcomeusername").style.visibility="hidden";
	}
		
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

<div class="topnav" id="myTopnav">
  <a href="#home" class="active">GT Hacker Home</a>
  <a href="./about.html">About</a>
    <div id="signup"><a href="./login.html" >Login</a>
	<a href="./signup.html" class="active">Sign Up</a></div>
	<div id="welcomeusername"></div>
  <a href="javascript:void(0);" class="icon" onclick="myFunction()">
  <i class="fa fa-bars"></i>
  </a>
</div>

<main>
	{#if item}
		<Item {item} returnTo="#/top/{page}"/>
	{:else if page}
		<List {page}/>
	{/if}
</main>
