# runix

`runix` is a blazing fast CLI tool that allows you to cast ASCII spells directly in your terminal. Whether you want to generate high-quality ASCII word art or render classic ASCII illustrations, `runix` has you covered.

## Installation

```bash
cargo install runix-cli
```

## Usage

Generate standard word art:
```bash
runix "Hello"
```

Render illustrations from the built-in library using the `--art` (or `-a`, `--a`) flag:
```bash
runix wolf -a
runix ghost --art
runix dragon --a
```

---

## The runix Art Library

`runix` comes pre-compiled with a curated library of over 80+ classic ASCII art pieces, sourced from the public domain. 

Here are all the available spells you can cast:

### Animals (26)
`cat`, `kitten`, `dog`, `cow`, `unicorn`, `turtle`, `butterfly`, `whale`, `elephant`, `wolf`, `bear`, `fish`, `shark`, `bird`, `eagle`, `penguin`, `snail`, `spider`, `rabbit`, `possum`, `kangaroo`, `bat`, `owl`, `snake`, `dragon`, `scorpion`

### Nature (12)
`sun`, `moon`, `mountain`, `tree` (or `pine`), `flower`, `wave`, `rose`, `snowman`, `leaf`, `mushroom`, `cactus`

### Celestial (7)
`planet` (or `earth`), `saturn`, `star`, `shooting-star`, `ufo`, `comet`

### Objects (41)
`house`, `castle`, `rocket`, `car`, `train`, `ship`, `plane`, `helicopter`, `sword`, `crown`, `heart`, `skull`, `shield`, `key`, `lock`, `book`, `clock`, `hourglass`, `trophy`, `lantern` (or `lamp`), `coffee` (or `cup`), `pizza`, `bomb`, `fire` (or `flame`), `lightning` (or `bolt`, `thunder`), `anchor`, `diamond`, `ghost`, `robot`, `alien`, `snowflake`, `dna` (or `helix`), `atom`, `guitar` (or `music`), `note`, `chess` (or `pawn`), `target`, `maze`, `compass`, `flag`, `heart2` (or `love`), `ship2` (or `galleon`), `crown2` (or `king`)

### People / Fantasy (4)
`wizard`, `angel`, `santa`, `pirate`

### Scenes (4)
`ocean` (or `sea`), `forest`, `campfire`

### Sci-Fi (2)
`enterprise` (or `starship`)
