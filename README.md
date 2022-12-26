# DoFi

A simple dotfile manager

## Usage

```
Usage: dofi <COMMAND>

Commands:
  add    Add a rule
  del    Delete a rule
  show   Show rule information
  list   List rules
  apply  Apply rules
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### `add`

```
Usage: dofi add [OPTIONS] <RULE> <SRC> <DST>

Arguments:
  <RULE>  Rule name
  <SRC>   Path (relative or absolute)
  <DST>   Path (absolute)

Options:
  -m <MODE>         Rule mode [default: link]
  -p <PROFILE>      Profile name [default: default]
  -f                Overwrite existing rule
  -h, --help        Print help information
```

### `del`

```
Usage: dofi del [OPTIONS] <RULE>

Arguments:
  <RULE>  Rule name

Options:
  -p <PROFILE>      Profile name [default: default]
  -h, --help        Print help information
```

### `show`

```
Usage: dofi show [OPTIONS] <RULE>

Arguments:
  <RULE>  Rule name

Options:
  -p <PROFILE>      Profile name [default: default]
  -h, --help        Print help information
```

### `list`

```
Usage: dofi list [PROFILE]

Arguments:
  [PROFILE]  Profile name [default: default]

Options:
  -h, --help  Print help information
```

### `apply`

```
Usage: dofi apply [PROFILE]

Arguments:
  [PROFILE]  Profile name [default: default]

Options:
  -h, --help  Print help information
```
