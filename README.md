# 🎂 Birthday

CLI tool to remember birthdays of people you know

## Installation

```shell
cargo install birthday
```

## Usage

Birthdays are stored in a SQLite database located in the [standard data directory](https://dirs.dev/)
of your OS, but you can also override this behavior by defining a custom path via the `BIRTHDAY_DATA` environment variable. 


```shell
# Add someone's birthday
$ birthday add "Ben Dover" 03 05 1990
$ birthday add "Hugh Jarse" 10 12 1995
$ birthday add "Anita Bath" 22 09 1987

# Show all birthdays
$ birthday all
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 1  │ Ben Dover  │ 3 may        | 34 (1990) │ today         │
│ 3  │ Anita Bath │ 22 september │ 37 (1987) │ 142 days      │
│ 2  │ Hugh Jarse │ 10 december  │ 29 (1995) │ 221 days      │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Show the next birthday
$ birthday next
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 2  │ Hugh Jarse │ 10 december  │ 29 (1995) │ 221 days      │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Show today's birthdays
$ birthday today
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 1  │ Ben Dover  │ 3 may        | 34 (1990) │ today         │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Get all birthdays where the name match "Ben"
$ birthday search --name Ben
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 1  │ Ben Dover  │ 3 may        | 34 (1990) │ today         │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Get all birthdays for a specific year
$ birthday search --year 1987
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 3  │ Anita Bath │ 22 september │ 37 (1987) │ 142 days      │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Get all birthdays for a specific month
$ birthday search --month 12
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 2  │ Hugh Jarse │ 10 december  │ 29 (1995) │ 221 days      │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Get all birthdays for a specific day
$ birthday search --month 3
╭────┬────────────┬──────────────┬───────────┬───────────────╮
│ Id │ Name       │ Birthday     │ Age       │ Next birthday │
├────┼────────────┼──────────────┼───────────┼───────────────┤
│ 1  │ Ben Dover  │ 3 may        | 34 (1990) │ today         │
╰────┴────────────┴──────────────┴───────────┴───────────────╯

# Forget a birthday by ID
$ birthday forget 3
Birthday of 'Anita Bath' has been forgotten 🗑️
```
