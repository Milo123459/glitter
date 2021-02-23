<p align="center">
  <img src="./assets/glitter.png" alt="Glitter" />
</p>
<h1 align="center">❯ Glitter</h1>
<h3 align="center">
Git tooling of the future.
<h3>
<h3 align="center">
    <img src="https://codecov.io/gh/Milo123459/glitter/branch/master/graph/badge.svg">
</h3>
<h1></h1>

## ❯ 👀 Features
- Config files
- Fast
- Easy to use
- Friendly errors

## ❯ 📚 Documentation

For proper docs, see [here](/docs/index.md)

## ❯ ✋ What, how and why?

Glitter is a wrapper for Git essentially, allowing you to compress multiple commands into one. Glitter is written in **rust** which not only makes it fast but also efficient. We simply parse your `.glitterrc` and run a few git commands under the hood. Why? Simplicity. If you maintain a project, this is probably the thing for you.

## ❯ 😀 Installation

**Windows**

[Scoop](https://scoop.sh)

```
scoop install glitter
```

**Linux/MacOS**

[Homebrew](https://brew.sh)

```
brew install glitter
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

**INSTALLATION ERRORS** are to go in issues.

