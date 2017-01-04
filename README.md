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

Once all that's set up, simply `cargo run` and it will load everything up. If you just want to see a
moveset without going through Twitter, you can pass `-s`/`--skip-twitter` to it, via `cargo run --
-s`.

On the first run, it will ask you to authenticate with the desired account. After that, every time
you run it, it will load up all the pokemon information and start up a loop where it will post one
moveset to Twitter every two hours.

Some example sets:

```text
Registeel @ Pixie Plate
Light Metal
- Protect
- Stealth Rock
- Charge Beam
- Frustration

Gastly @ Stone Plate
Levitate
- Snatch
- Spite
- Night Shade
- Snore

Bastiodon @ Hard Stone
Sturdy
- Round
- Block
- Attract
- Taunt

Geodude @ Toxic Plate
Sand Veil
- Superpower
- Gyro Ball
- Earth Power
- Tackle

Porygon @ Destiny Knot
Download
- Sleep Talk
- Rest
- Flash
- Confide

Hoopa @ Rose Incense
Magician
- Protect
- Astonish
- Skill Swap
- Dual Chop

Swellow @ Maranga Berry
Guts
- Wing Attack
- Snore
- Round
- Roost

Grotle @ Eject Button
Overgrow
- Nature Power
- Cut
- Confide
- Mega Drain
```
