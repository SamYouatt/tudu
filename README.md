# Tudu


## Viewing tasks

`tudu` - Shows the tasks for today

```
When have tasks:
1   ◯ - Do dishes
2   ◐ - Do taxes
3   ● - 5 pushups

When no tasks:
You have no tasks for today, to add a task today use the 'tudu add "Example task"' command
```

`tudu today/tomorrow/yesterday` - Shows the tasks for the relative day

`tudu 10-6` - Shows the tasks for the 10th of June in the current year

`tudu 10-6-2022` - Shows the tasks for the 10th of June 2022

## Adding tasks

`tudu add "Example task"` - Add the task to today's list

`tudu add yesterday "Example task"` - Add the task to the relative day

`tudu add 10-6` - Add the task to the 6th of June in the current year

`tudu add 10-6-2022` - Add the task to the 6th of June 2023

## Setting task states

```
◯ - [N]ot started
◐ - [S]tarted
● - [C]ompleted
► - Carry [F]orward
x - [X] Not doing
```

`tudu set 1 C` - Set the first task of today to complete

`tudu set yesterday 3 X` - Set the third task of yesterday to Not Doing

`tudu set 10-6 2 F` - Set the second task of 6th of June this year to Carrying Forward

`tudu set 10-6-2022 2 S` - Set the second task of 10th of June 2022 to Started

`tudu complete 2` - Complete the second task from today

## Removing tasks

`tudu delete 3` - Delete the third task from today
