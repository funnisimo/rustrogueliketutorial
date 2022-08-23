# Introduction

---

**_About this tutorial_**

_This tutorial is free and open source, and all code uses the MIT license - so you are free to do with it as you like. My hope is that you will enjoy the tutorial, and make great games!_

_If you enjoy this and would like me to keep writing, please consider supporting [my Patreon](https://www.patreon.com/blackfuture)._

[![Hands-On Rust](./beta-webBanner.jpg)](https://pragprog.com/titles/hwrust/hands-on-rust/)

---

Every year, the fine fellows over at [r/roguelikedev](https://www.reddit.com/r/roguelikedev/new/) run a _Tutorial Tuesday_ series - encouraging new programmers to join the ranks of roguelike developers. Most languages end up being represented, and this year (2019) I decided that I'd use it as an excuse to learn Rust. I didn't really want to use `libtcod`, the default engine - so I created my own, [bracket-lib (aka RLTK)](https://github.com/thebracket/bracket-lib). My initial entry into the series isn't very good, but I learned a lot from it - you can find it [here](https://github.com/thebracket/rustyroguelike), if you are curious.

The series always points people towards an excellent series of tutorials, using Python and `libtcod`. You can find it [here](http://rogueliketutorials.com/tutorials/tcod/). Section 1 of this tutorial mirrors the structure of this tutorial - and tries to take you from zero (_how do I open a console to say Hello Rust_) to hero (_equipping items to fight foes in a multi-level dungeon_). I'm hoping to continue to extend the series.

I also _really_ wanted to use an Entity Component System. Rust has an excellent one called Specs, so I went with it. I've used ECS-based setups in previous games, so it felt natural to me to use it. It's also a cause of continual confusion on the subreddit, so hopefully this tutorial can shine some light on its benefits and why you might want to use one.

I've had a **blast** writing this - and hope to continue writing. Please feel free to contact me (I'm `@herberticus` on Twitter) if you have any questions, ideas for improvements, or things you'd like me to add. Also, sorry about all the Patreon spam - hopefully someone will find this sufficiently useful to feel like throwing a coffee or two my way. :-)

---

Copyright (C) 2019, Herbert Wolverson.

---
