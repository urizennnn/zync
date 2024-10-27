<div align="center">
  <img src="media/zync-logo-borderless.svg" width="240" alt="Zync Logo">
</div>

[![rustc](https://img.shields.io/badge/rustc-1.54+-blue.svg)](https://www.rust-lang.org)
[![dependency status](https://deps.rs/repo/github/urizennnn/zync/status.svg)](https://deps.rs/repo/github/urizennnn/zync)
[![GitHub contributors](https://img.shields.io/github/contributors/urizennnn/zync)](https://github.com/urizennnn/zync/graphs/contributors)


> Powered by Ratatui

### Demo of the project at the moment
This is how the project looks like at the moment. The project is still in development and the final version may look different.
![Demo](https://github.com/urizennnn/zync/blob/bump-v2/media/demo.gif?raw=true)

### Documentation
So far this is what we have at the moment. We are planning on replacing the recent logs screen with a session screen. 
![Session](https://github.com/urizennnn/zync/blob/bump-v2/media/session-preview.png?raw=true)

This change takes priority after some major bugs have been fixed.

## Bugs
- [ ] Input field character overflow
- [ ] Error popup has to be manually closed instead of timing out
 
 More to come

## Features to come 
- [ ] Implement Session Page 
- [ ] Implement the TCP libraries
- [ ] Implement the P2P libraries using lib-p2p maybe

Some others that I can't think of atm

### How to run 
Install the rust via rustup or your package installer clone this repo 
```git
git clone https://github.com/urizennnn/zync.git
```
then run the code using cargo 
```rs
cargo run
```
or if you prefer to run it in release mode to view how the human panic output errors
```rs
cargo run --release
```
after that follow the prompt.

