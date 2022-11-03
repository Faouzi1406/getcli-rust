#CLI DOCS

**About**
This is a project I made to learn rust and how rust works.
Its a simple project that allows you to make get, post and delete requests.


****
**I need help!**
````bash
cargo grul help 
````

**How to make a request**
````bash
cargo "type" "url" 
````

Example
````bash
cargo "post" "http://localhost:8080/post-route" 
````

This will the print out the response body to the terminal.

* "types"
	By using --type you can give the type that you want to use 
	* post
	* get
	* delete
***


**Add a body to the request**
If you ar making a post or delet request you can also use a body 
- Only JSON right now

a body for now can only have a  name and a value
- this while change in the future

example: 
````bash
cargo "post" "http://localhost:8080/post" '"name": "value", "newname":"new value"'
````



