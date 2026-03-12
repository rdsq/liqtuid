# Liqtuid

Open source Liquid Sort Game (in TUI)

I really liked that kind of game, it was kind of addictive. The problem though: all of them were crap with ads. So I created my version. Not only is it open source and ad free, but also tweakable

## Features

1. **You can pass your own** number of bottles, bottle depth (haven't seen anything other than 4 in other implementations, so another W), and empty bottles number
2. **Random colours**. Which means you can create virtually infinite number of bottles, but yeah, sometimes the generator messes up and they look too similar. Mostly they are fine though
3. **Keyboard control**. You select bottles with number keys, then qwerty, then uppercase qwerty if you set way too many bottles. This also means that everything after uppercase `M` or bottle number `10 + 26 * 2 = 62` is unreachable. But hey, 62 is a lot more than other implementations offer anyway

## Installation

Rust project = simple installation

```sh
cargo install --git https://codeberg.org/rdsq/liqtuid
# Or
cargo install --git https://github.com/rdsq/liqtuid
```

## Controls

As mentioned above, you select bottles with `1-0;q-m;Q-M`

To exit the game, press `Ctrl+D` or `Ctrl+C`

If colours in game are ~~f#%ked~~ off, you can get new ones with `Ctrl+R`

And to start a new game in game you can press `Ctrl+N`! Not sure why, but you can
