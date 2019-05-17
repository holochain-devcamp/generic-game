# Holochain Generic Game
## tic-tac-toe and checkers examples

## First Challenge - Play a game with yourself

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