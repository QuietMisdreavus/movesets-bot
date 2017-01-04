# movesets-bot

A friend asked what it would take to make a bot that could pull up random Pokemon movesets that were
"legal", meaning that instead of pulling from every single move that existed, you could only pull
moves from the list that the Pokemon in question could be able to learn. Pulling again from
[veekun/pokedex][], this is an attempt at doing just that.

[veekun/pokedex]: https://github.com/veekun/pokedex

Before running this, you'll need to do the following:

- [Install Rust](https://www.rust-lang.org/en-US/install.html)
- [Set up an application with Twitter](https://apps.twitter.com/)
- Put the consumer key/secret into the files `src/consumer_key` and `src/consumer_secret`

Once all that's set up, simply `cargo run` and it will load everything up.

On the first run, it will ask you to authenticate with the desired account. After that, every time
you run it, it will load up all the pokemon information and post one moveset to Twitter.
