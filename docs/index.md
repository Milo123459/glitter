## Welcome to the Glitter docs!

The docs for glitter. Keep on reading if you are interested!

To use a flag, ie `-d`  you do something like this:
```sh
$ glitter -d -- ...
```
where ... is the rest of your command. The -- just signals you are done using flags and can start using the actual action / command.

### GlitterRc structure

The main structure of the file:

If it is suffixed with a ? that means it is optional.

```
commit_message?: String: The commit message template, use $<arg-idx> to get a specific arg and suffix with a + for a rest argument.
commit_message_arguments?: Arguments[]: The array of commit message arguments.
fetch?: Bool: Execute git fetch in commands.
custom_tasks?: CustomTaskOptions[]: Custom tasks specified which can be executed via the cc command.
```

Structures referenced:

```
Arguments {
    argument: Number: The argument idx.
    case?: String: The case to convert this argument to.
    type_enums?: String[]: Type enums, these are used to validate arguments to make sure you only allow specific things to pass through.
}

CustomTaskOptions {
    name: String: The name of the custom task.
    execute: String[]: The commands to execute in order.
}
```

Suggestions are to be put in discussions.