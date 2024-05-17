# DoFi

A simple dotfile manager

## Usage

```
Usage: dofi <COMMAND>

Commands:
Usage: dofi <command> [<args>]

Options:
  --help            display usage information

Commands:
  add               add rule
  del               del rule
  show              show rule information
  list              list rules
  apply             apply rules
```

### `add`

```
Usage: dofi add <rule> <src> <dst> [-m <mode>] [-p <profile>] [-o]

Positional Arguments:
  rule              rule name
  src               source path
  dst               target path

Options:
  -m, --mode        apply method [copy, link]
  -p, --profile     profile name
  -o, --overwrite   overwrite existing rule
  --help            display usage information

```

### `del`

```
Usage: dofi del <rule> [-p <profile>]

Positional Arguments:
  rule              rule name

Options:
  -p, --profile     profile name
  --help            display usage information
```

### `show`

```
Usage: dofi show <rule> [-p <profile>]

Positional Arguments:
  rule              rule name

Options:
  -p, --profile     profile name
  --help            display usage information
```

### `list`

```
Usage: dofi list [-p <profile>]

Options:
  -p, --profile     profile name
  --help            display usage information
```

### `apply`

```
Usage: dofi apply [-p <profile>]

Options:
  -p, --profile     profile name
  --help            display usage information
```
