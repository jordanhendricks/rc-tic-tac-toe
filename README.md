# Tic Tac Toe

## Features

* scalable dimension board size
* 2 players

## Win Conditions

The player wins if their piece:

* occupies every space on a row of the board
* occupies every space on a column of the board
* occupies every space on the left diagonal of the board
* occupies every space on the right diagonal of the board

Note that this means that for an NxN board, N pieces are required to win.

## Play

### Starting the game

2 players:

```
$ ./tic-tac-toe --size 5
```

Single player (against CPU):

```
$ ./tic-tac-toe --solo
```
