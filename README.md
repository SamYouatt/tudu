# Tudu

## Dates

Dates can be specified in the following ways:
- 10-6-2023: fully qualified dates, in d-m-yyyy format
- 10-6: partial dates, excluding the year will set it to this calendar year
- yesterday/today/tomorrow: there are three relative date commands that are also accepted
- none specified: when an optional date is accepted and none is given, the current date is used

## Commands

### Viewing tasks

`tudu` - Shows the tasks for today

`tudu view [date]` - Show the tasks for a specified date

### Adding tasks

`tudu add [task] *[date]` - Add the specified task to an optional date

### Setting task states

```
◯ - [N]ot started
◐ - [S]tarted
● - [C]ompleted
► - Carry [F]orward
x - [X] Not doing
```

`tudu set [index] [state] *[date]` - Set the task at the given index to the given state on an optional date

### Removing tasks

`tudu delete [index] *[date]` - Delete the task at the specified index on an optional date

### Completing tasks

`tudu complete [index] *[date]` - Mark the specified task as complete on an optional date

This is equivalent to a `set` command with the `C` state.

### Editing tasks

`tudu edit [index] [task] *[date]` - Update the task at the given index with the new task description, with an optional date

### Help

`tudu help`
