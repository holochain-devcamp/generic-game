# Holochain Generic Game

This is a generic game framework packaged in a Holochain application!

**The main challenge for the September 2019 devcamp is to built your own game using this game framework.**

## Overview

Previous read: [Fundamentals of Games on Holochain](https://hackmd.io/@FqBkpkUfTSKcADA4DAqhqw/S1gB6kOiE)

The generic game framework is tuned for the types of games that Holochain is good for. In the game framework, all the work of creating entries and linking them is already done for you. You only need to "fill in the blanks" to implement the behaviour of your game.

You will be building your game as the devcamp progresses - take it step by step.

Overview of the zome files: 

- `lib.rs`, `game.rs`, `game_move.rs`, `matchmaking.rs`: these files are the heart of the generic game framework. In this devcamp, we won't be editing these files, although you can look at them or experiment if you're curious.
- `tictactoe` folder: each folder constitues a game. You can use it as a reference game implementation when you are implementing your own.
- `your-game` folder: this is the folder that you will be editing. It contains todos that will guide you through the process.

## ✍️ First Challenge - Play a game with yourself

The first task lets you try out running a holochain instance locally with two agents to play a game of tic-tac-toe. Before you begin make sure you have the holochain conductor `holochain` available on your path.

### 1. Start the conductor

From the repo root directory run the following command

```
holochain -c ./conductor-config.toml
```

It might take a few seconds to unlock the keystore but you should see something like the following:

```
Using config path: ./conductor-config.toml
Unlocking agent keys:
Unlocking key for agent 'test_agent1': 
Reading keystore from ./agent1.keystore
Unlocking key for agent 'test_agent2': 
Reading keystore from ./agent2.keystore
2019-05-17 15:55:37 ThreadId(1):conductor: starting signal loop
Reading DNA from ./dist/generic-game.dna.json
Failed to load instance instance1 from storage: ErrorGeneric("State could not be loaded due to NoneError")
Initializing new chain...
...
```
and then a whole lot of colored debug output. Scanning the debug you should be able to see indication that the conductor is:

- unlocking the keystores
- loading the game DNA from file
- creating a local chain for each agent
- validating the agents first two local chain entries (the `DNA` entry and the `agent` entry)

These are the essential tasks for a holochain instance that is starting for the first time.

### v2. Start the CLI

To keep things simple we will be interacting with our Holochain conductor using a command line interface that connects via HTTP. Make sure you keep the conductor running and in a new terminal window run the following:

```
cd cli
cargo run http://localhost:3001 instance1
```

This will build the CLI and then run it. This is instructing the CLI to connect to a holochain instance running on localhost port 3001 with the instance id `instance1`. After it builds you should see the following:

```
######################################################################
CLI interface for games written using the Holochain Generic Game framework.
Enter "help" for a list of commands.
Use "create_game <agent_id>" or "join_game <game_address>" to start or join a game.
Press Ctrl-D or enter "quit" to exit.
######################################################################


Your agent address is "HcScjcgKqXC5pmfvka9DmtEJwVr548yd86UPtJGGoue9ynuikuRTN7oE5zcjgbi"

Send this to other players so they can invite you to a game.


No game> 
```


If you see this it means you are now successfully connected to the holochain instance and can participate as this agent. Be sure to test out the commands to see what you can do.

You can't play a game with one agent so open up ~another~ terminal window and connect to the conductor on the port/instance where the second agent is running:
```
cd cli
cargo run http://localhost:3002 instance2
```

### 3. Play a game

Now the tricky part, to play a game of tic-tac-toe with yourself! Keep the conductor running and both windows with the CLI. We'll refer to one of them as Agent A and the other as Agent B. 

Agent A will be the one to create the game. Copy the agent address from Agent B and run the following command in **Agent A**:
```
new_game HcScidPSdAT43q9qirJwt5rHJYjjsvougV3jgSBwdJujszw3bBu5Mktr74Rgnea
```

This should create a new game and show the following output:
```
Non-creator must make the first move 

  x  0 1 2
y
0   | | | |
1   | | | |
2   | | | |

QmTNHtXZye7vz3d4LQz5zgHvk1wvxbsBHcstorDWQxshfZ> 
```

We have just commit our first entry in the local chain and DHT! A `Game` entry was created with Agent B as the opponent and shared to the DHT. The hash at the bottom of the screen is the hash/address of the Game entry in the DHT. This is the unique identifier we can use to join this game (Note this will be different for everyone as our Game entry includes a timestamp).

Copy the game address and run the following in **Agent B**
```
join_game QmTNHtXZye7vz3d4LQz5zgHvk1wvxbsBHcstorDWQxshfZ
```

If the Game entry was successfully shared in the previous step Agent B shoud now see:

```
Setting current game hash to QmTNHtXZye7vz3d4LQz5zgHvk1wvxbsBHcstorDWQxshfZ

Non-creator must make the first move 

  x  0 1 2
y
0   | | | |
1   | | | |
2   | | | |


```

and as the non-creator we are allowed to make the first move. To see the available moves in this game you can run the `moves` command

```
QmTNHtXZye7vz3d4LQz5zgHvk1wvxbsBHcstorDWQxshfZ> moves
The valid moves are:
- {"Place":{"pos":{"x":0,"y":0}}}
```

Lets try making a move. Make sure you are Agent B and run

```
make_move {"Place":{"pos":{"x":0,"y":0}}}
```

and you should get:

```
making move: "{\"Place\":{\"pos\":{\"x\":0,\"y\":0}}}"
Move cast successfully
Waiting for gossip...
OK!

It is your opponents turn 

  x  0 1 2
y
0   |X| | |
1   | | | |
2   | | | |

```

Thats it! Now you know how it works you can play out the rest of the game. Make sure you test what happens if you try to make an invalid move.

## ✍️ Implement your own game

Here you need to make an important decision on what game you will be implementing:

- **Simple checkers game**: this is the main option. This is the game we will be implementing with all the group during the devcamp. Also, we already have reference implementation of the checkers game you can find in the other branch of this repository if you're stuck.
- **Design and implement your own game** (advanced rust programmer): feeling adventurous? You can build another kind of game, from idea to design to implementation. Exciting! As devcamp mentors, we'll help all we can, though we can't provide the same type of resources of the checkers option.

You already known which game you will be implementing? Good! You can begin these steps: 

1. Rename the folder `your-game` to, well, to your game name.
2. Look for `DEVCAMP TODO` in all the files inside that folder. You should see all the "fill in the blank" spots you will be implementing. These comments contain examples, hints and references to help you.

**Note**: as your game won't be completely ready to compile until the last `TODO` is completed, we recommend using the code completion and error highlighting of an IDE.

### ✍️ Exercises after Session 3 - 12/09

Describe the moves of your game.

1. Implement `DEVCAMP TODO #1`.
2. Implement `DEVCAMP TODO #2`.

### ✍️ Exercises after Session 4 - 13/09

Describe your game state, how it evolves and how it is displayed.

1. Implement `DEVCAMP TODO #3`.
2. Implement `DEVCAMP TODO #4`.
3. Implement `DEVCAMP TODO #5`.
4. Implement `DEVCAMP TODO #6`.

### ✍️ Exercises after Session 5 - 14/09

Describe whether a move is valid or not depending on the current game state.

1. Implement `DEVCAMP TODO #7`.
2. Implement `DEVCAMP TODO #8`.

### ✍️ Build and run

After completing all the steps above, your game will be ready to be compiled and executed. Hooray!

If you have opted to make the checkers game, you can use the [holochain-games-ui](https://github.com/holochain-devcamp/holochain-games-ui) to play with your game.

In any case, you can use the CLI included in this repo to play any game you have implemented. Refer to step 1 of this guide to know how to use the CLI.