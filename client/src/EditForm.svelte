
<script>
	export let parentid = "";
    let show_form = false;
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

	function handlereply() {
		show_form = true;
		console.log(show_form);
	}

function handleFormSubmit() {
var mydata = {
              "parentid": parentid,
              "text": document.getElementById('commentfield').value,
              
            };
  axios.post('https://greetez.com:4343/item_api/create_comment',
  mydata,{withCredentials: true}
  ).then((response) => {
	alert("created comment");
  window.location.assign("https://gthackerhome.github.io");
  }, (error) => {
    console.log(error);
	alert("There's been an error, please try using a different username");
  });

};

</script>

<style>
	article {
		border-bottom: 1px solid #eee;
		margin: 1em 0 0 0;
		/* padding: 1em 0 0 0; */
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
	{#if !show_form} 
		<p on:click="{handlereply}" class="fake-link">reply</p>
    {:else}
    	<label for="commentfield">Comment:</label>
        <textarea id="commentfield" name="commentfield" style="height:150px"></textarea>
        <input type="submit" value="Submit" on:click="{handleFormSubmit}">

	{/if}	
</article>