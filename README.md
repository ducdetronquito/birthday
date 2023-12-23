# 🎂 Birthday

CLI tool to remember birthdays of people you know

## Usage

```shell
$ birthday add "Ben Dover" 03/05/1990
$ birthday add "Hugh Jarse" 10/12/1995
$ birthday add "Anita Bath" 22/09/1987

# Show all birthdays
$ birthday all
Ben Dover  03/05/1990 34yo
Hugh Jarse 10/12/1995 39yo
Anita Bath 22/09/1987 37yo

# Show the next birthday
$ birthday next
Hugh Jarse 10/12/1995 39yo

# Show today's birthdays
$ birthday today
Ben Dover  03/05/1990 34yo

# Get all birthdays where the name match "Ben"
$ birthday search --name Ben
Ben Dover  03/05/1990 34yo

# Get all birthdays for a specific day and month
$ birthday search --date 22/09
Anita Bath 22/09/1987 37yo
```
