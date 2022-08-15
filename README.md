<p align="center">
  <img src="./assets/glitter.png" alt="Glitter" />
</p>
<h1 align="center">❯ Glitter</h1>
<h3 align="center">
Git tooling of the future
<h3>
<h1></h1>

## Features
- Config files
- Simple errors
- Glitter Hooks (Git hooks natively built into Glitter)

## What is glitter?

Glitter is a tool for generating and structuring commit messages via arguments passed to the tool. It allows you to configure it extensively and easily.

## Installation

**Windows**

[Scoop](https://scoop.sh)

```
scoop install glitter
```

**Linux**

*This is a bash script that can install on **any** platform other than **windows**, not just linux.*

```
curl -fsSL https://raw.githubusercontent.com/Milo123459/glitter/master/install.sh | bash
```

**Other**

Check the [releases](https://github.com/Milo123459/glitter/releases) and download the appropriate binary. Or build from source.

To build from source run this:

*You need rust installed!*

```
cargo install --git https://github.com/Milo123459/glitter
```

## Get started

A simple example when using glitter would be a 3 step commit template. For example, something to turn `glitter push fix docs fix typo` into `fix: docs: fix typo`.

This example covers using type_enums, hooks and how glitters argument system works.

Firstly, we can define our `.glitterrc` to support 2 or more arguments.

```json
{
    "commit_message": "$1: $2: $3+"
}
```
This snippet alone now allows us to do `glitter push fix docs fix typo` and would template to `fix: docs: fix typo`. $1 is the first argument passed to glitter push, $2 is the second, and $3+ means that the third argument and anything after that should take it's place.

Now, lets take a look at `type_enums` - a way of validating arguments.

Let's add a `commit_message_arguments` to our `.glitterrc`:
```json
{
    "commit_message": "$1: $2: $3+",
    "commit_message_arguments": [
        {
            "argument": 1,
            "case": "lower",
            "type_enums": [
                "fix",
                "feat"
            ]
        }
    ]
}
```
This snippet now means that the first argument will:
- be converted to lower-case
- matched against the type_enums, and if it does not match, it fails

For example, `glitter push fix docs fix typo` would work, but `glitter push chore docs fix typo` would not, because `chore` isn't in the type enums.

Next: glitter hooks.

Glitter hooks are like git hooks, but always run before `git add` - it allows you to run/make your own hooks with ease.

An example of a hook to run `cargo fmt` would look like this:
```json
{
    "custom_tasks": [
        {
            "name": "fmt",
            "execute": [
                "cargo fmt"
            ]
        },
    ],
    "hooks": ["fmt"]
}
```
This defines a custom task, which can also be run via `glitter cc` (for example `glitter cc fmt` would run `cargo fmt`). We then have a hooks array which specifies a custom task to run before running `git add`.

## FAQ

> Does **"this hello"** count as 1 or 2 arguments?

**This example counts as 1.** For example `glitter push hello "world how" are you` would give the following arguments:
```
1: hello
2: world how
3: are
4: you
```

## Available Cases

- lower
- upper
- snake
- screaming-snake
- kebab
- train
- sentence
- title
- class
- pascal

![Alt](https://repobeats.axiom.co/api/embed/94616a17e7b0081aad0b1634999ac54c23bd5e5c.svg "Repobeats analytics image")
