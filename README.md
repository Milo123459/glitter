<p align="center">
  <img src="./assets/glitter.png" alt="Glitter" />
</p>
<h1 align="center">❯ Glitter</h1>
<h3 align="center">
Git tooling of the future
<h3>
<h3 align="center">
    <a href="https://github.com/Milo123459/glitter/releases">
    <img src="https://img.shields.io/github/downloads/Milo123459/glitter/total.svg">
    </a>
    <img src="https://img.shields.io/github/stars/Milo123459/glitter">
    <img src="https://tokei.rs/b1/github/Milo123459/glitter?category=lines">
    <img src="https://www.codefactor.io/repository/github/milo123459/glitter/badge">
</h3>
<h1></h1>

![Alt](https://repobeats.axiom.co/api/embed/94616a17e7b0081aad0b1634999ac54c23bd5e5c.svg "Repobeats analytics image")

## New feature: Glitter hooks!

Click [here](#--glitter-hooks) for more info!

## ❯ 👀 Features
- Config files (with defaults!)
- Fast
- Easy to use
- Friendly errors (how to fix them included!)
- Multi branch support (defaults to the one you are on!)
- Beautiful, as if it just rolled in **Glitter**!
- Glitter Hooks (Git hooks but Glitter)

## ❯ 📚 Documentation

For proper docs, see [here](/docs/index.md), this also includes examples.

## ❯ ✋ What, how and why should you use Glitter?

Glitter is a wrapper for Git essentially, allowing you to compress multiple commands into one. Glitter is written in **rust** which not only makes it fast but also efficient. We simply parse your `.glitterrc` (if it exists, if not it'll use the default config) and run a few git commands under the hood. Why? Simplicity. If you maintain a project, this is probably the thing for you.

## ❯ 😀 Installation

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
git clone https://github.com/Milo123459/glitter
cd glitter
cargo install --path .
```

To update:

```
cd glitter
git pull
cargo install --path .
```

**🛑 Please report any bug reports in issues, I'll try and respond ASAP**

## ❯ 🎉 Get started

We're so happy to see your interest in Glitter! Leave a ⭐ if you like Glitter!

Firstly, lets create a `.glitterrc` - the configuration file for Glitter.

The glitterrc is basically JSON but a fancy extension. If we want to make it so it automatically templates our commit message we can do it like so:
```json
{
    "commit_message": "$1($2): $3+"
}
```
and you can use it like so: `glitter push chore deps bump deps`, this would produce the message `chore(deps): bump deps`. As you probably understand, $ is the prefix of arguments, ie, you can do $1 for the first argument, $2 for the second, etc. The + basically means all arguments after the one you specify will be placed there. Lets say we want it to be `chore(Deps): bump deps` and it formats with that case. How? Easy. Lets add another key to our file:
```json
{
    ...
    "commit_message_arguments": []
}
```
inside this key we can then add the argument to configure, in this case, the 2nd one, let's add that.
```json
{
    ...
    "commit_message_arguments": [
        {
            "argument": 2
        }
    ]
}
```
and then, to configure the case, add the key `case` and watch the magic!
```json
{
    ...
    "commit_message_arguments": [
        {
            "argument": 2,
            "case": "pascal"
        }
    ]

}
```
Running `glitter push chore deps bump deps` would then give us the commit message of `chore(Deps): bump deps` 🎉!

## ❯ 📷 FAQ

> Does **"this hello"** count as 1 or 2 arguments?

**This example counts as 1.** For example `glitter push hello "world how" are you` would give the following arguments:
```
1: hello
2: world how
3: are
4: you
```

> Why is it fast sometimes but not the next?

**That's reliant on your internet connection.** - We are just running git commands under the hood. Git will be the thing taking it's sweet time.

## ❯ 🎣 Glitter Hooks

Glitter Hooks are Git hooks without the bash. Here is an example of how we can run `cargo fmt` before commiting to this codebase.

```json
{
    ...
    "custom_tasks": [
        {
            "name": "fmt",
            "execute": ["cargo fmt"]
        }
    ],
    "hooks": ["fmt"]
}
```

From this, before we commit, `cargo fmt` will be executed. You can add more commands to be executed by simply adding another command to the `fmt` field, or, adding another custom_task and referencing that in hooks.
If you need help, you can make a discussion, and if you find a bug, please make a bug report!

## ❯ 📣 Available Cases

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

.. got a suggestion? Please make a discussion.

Installation errors are to go in issues.

Hope Glitter helps you!
