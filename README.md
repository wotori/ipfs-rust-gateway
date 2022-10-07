# ipfs rust gateway

This simple api that allow you to save your files on server and pin them to ipfs.

# todo

- token api
- add password / token validation / secret header check
- save files to users folder based on token

# how to use

just run this commands:

```cmd
% cargo build
% cargo run
```

build for production and run
```cmd
% cargo build --relese
% ./target/release/ipfs-api
```