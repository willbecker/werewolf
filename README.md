# Werewolf
Will Becker wibecker@pdx.edu

This is an online multiplayer, text based version of the game [Werewolf](https://en.wikipedia.o/wiki/Mafia_(party_game)) 

Progress
--------
Some far, I have been able to implement most of the networking code. I'm using WebSockets as my communiction protocal which I implement using [WS-RS](https://github.com/housleyjk/ws-rs).

Running the Game
----------------
The game is split up into a server and a client. You can specify which one to run using command line arguments.

Run the client with
```shell
cargo run
```
Run the server with
```bash
cargo run -- -s
```
At this point, the server is hard coded to run on localhost at port 8888. The client will prompt for an address when it is started. You can hit enter to accept default address which will just connect you to a server running on the same computer.

You can enter commands into to server terminal to interact with the game.

Commadns so far
```
list - lists all the players that are in the game
quit - shuts the server down
```

Testing
-------
I don't really know where to start with writing automatic test for the networking. Maybe I can create a fake server in the client test, and fake clients in the server tests. With how much my server and client have to communicate for basic tasks, it seems like unit testing would be quite difficult.

The testing I have done has been pretty manual. I open a server terminal and a bunch of client terminals, and just play around to see if I can break anything. The network testing situation isn't great. I would like to improve this going forward, but I think it will take some time implement. On the other hand, I think writting tests for the game part will be easier, which I should be able to manage going forward.

Todo
----
The networking has taken signifcantly more work then I had anticipated. Right now, the game is essentially a glorified waiting room that tells you how many players are connected. Most of the Werewolf game machanics still need to be done. I need to make the server address and port configurable, so that the game can run over the internet. I need to expand testing, and include some documentation. In the far future, I would like make the client into a webapp to make it more accessible.

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
