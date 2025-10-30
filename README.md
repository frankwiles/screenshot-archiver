# Personal Screenshot archiver

## Usage 

```shell
$ screenshot-archiver --source ~/Desktop/ --destination ~/screenshots/
```


I run this once a day in the early AM as an automated job via Keyboard Maestro, but 
however you want to run it should be fine. 

It will archive images matching a couple of patterns that are a day old or older. 

## Building 

You will need a modern Rust setup, I suggest using [rustup](https://rustup.rs)

```shell
$ cargo install
```


## MAINTENANCE NOTE 

This is personal code, I don't really intend to maintain it for anyone else, but
someone really liked the idea on social media and asked if I would Open Source it
so I did. 

[Frank Wiles](https://www.frankwiles.com) 

