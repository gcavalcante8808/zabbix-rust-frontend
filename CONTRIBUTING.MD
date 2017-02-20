Pull Requests
-------------

When creating a pull request you should:

 * Open an issue first: Confirm that the change of feature will be accepted;
 * Verify if the tests are passing and no notices or warning are returned;
 * Update the documentation: Every step into the glory should be put on the documentation.

Installing Dependencies
-----------------------

You can install the rust language using the available documentation at https://www.rust-lang.org/en-US/install.html.

After the installation, clone this repo and create the first build (the cargo will download all needed crates in the proccess):

```
    git clone git@github.com:gcavalcante8808/zabbix-rust-frontend.git
    cd zabbix-rust-frontend
    cargo build && cargo test
```
