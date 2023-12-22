# 🎂 Birthday

CLI tool to remember birthdays of people I know

## Usage

```shell
$ birthday add "Ben Dover" 03/05/1990
$ birthday add "Hugh Jarse" 10/12/1995
$ birthday add --name "Anita Bath" --date 22/09/1987


# Show next birthday by default (or with --next)
$ birthday
Hugh Jarse 10/12/1995 39yo

$ birthday --all
Ben Dover  03/05/1990 34yo
Hugh Jarse 10/12/1995 39yo
Anita Bath 22/09/1987 37yo

$ birthday --today
Ben Dover  03/05/1990 34yo

# Get all birthdays where the name contain "Ben"
$ birthday --name Ben
Ben Dover  03/05/1990 34yo
```
