#Request CLI

**About**
This is a project I made to learn rust and how rust works.
Its a simple project that allows you to make get, post and delete requests.
this is also still a work in **progress** and not done. 

**How to install**
You have to first make sure you have rust installed:

- Install rust if you dont have rust
**macOS, Linux, or another Unix-like OS**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
**Windows**
https://forge.rust-lang.org/infra/other-installation-methods.html

- clone repositry 
````bash
git clone https://github.com/Faouzi1406/getcli-rust.git
cd getcli-rust
````


****
**How to make a request**
````bash
cargo run "type" "url" 
````

Example
````bash
cargo run "post" "http://localhost:8080/post-route" 
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
cargo run "post" "http://localhost:8080/post" '"name": "value", "newname":"new value"'
````

