# Welcome to the Glitter docs

The docs for glitter. Keep on reading if you are interested!

To use a flag, ie `-d`  you do something like this:

```sh
glitter -d -- ...
```

where ... is the rest of your command. The -- just signals you are done using flags and can start using the actual action / command.

## GlitterRc structure

## The main structure

If it is surrounded with Option<...> it means that it is optional.
A Vec<...> is an array

```rs
// The commit message template, use $<arg-idx> to get a specific arg and suffix with a + for a rest argument.
commit_message: Option<String>

// The array of commit message arguments.
commit_message_arguments?: Option<Vec<Arguments>>

// execute `git fetch` in commands
fetch: Option<bool>

// Custom tasks specified which can be executed via the cc command.
custom_tasks: CustomTaskOptions[]

// Things to run before `git commit`. You specify the name of a custom task defined in `custom_tasks`. The whole process will be aborted if the command returns exit status of 1
hooks: Option<Vec<String>> 
```

### Structures referenced

```rs
struct Arguments {
    // The argument index
    argument: i32 
    
    // the case to convert the argument to
    case: Option<String>

    // Type enums, these are used to validate arguments to make sure you only allow specific things to pass through
    type_enums: Option<Vec<String>>
}

struct CustomTaskOptions {
    // The name of the custom task
    name: String 

    // The commands to execute in order
    execute: Vec<String> 
}
```

Suggestions are to be put in issues or discussions.

## Examples

### Push to a seperate branch

```sh
glitter --branch branch-name -- push arguments..
```

### push to a fresh branch (that is not on the hosted repo)

```sh
glitter -- --branch branch-name --nohost -- push arguments..
```

### Dry run

```sh
glitter -- --dry -- push arguments..
```
