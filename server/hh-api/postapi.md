
Item fields:
	 id String, i set this
     author String,
     time i32, i set this
     itemtype String,
     title Option<String>,
     url Option<String>,
     text Option<String>,
     parentid Option<String>,
     descendents Option<i32> i set this,
     score Option<i32> i set this


For Posts:

Create: call create_post() w/ json:

     author String,
     itemtype String,
     title String,
     url (Can be null),
     text String (Can be null)),





