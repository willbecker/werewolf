# Werewolf
Will Becker wibecker@pdx.edu

This is a text based version of the game <a href="https://en.wikipedia.org/wiki/Mafia_(party_game)">Werewolf</a>. 
This version of werewolf is going to be designed for online play, although the online implementation won't come until after the core functionaliy is complete.

Rustc version: 1.38.0-nightly

## Roles
In Werewolf, each player gets assigned a random role. Every role has a win condition, and some roles get extra information or have an ability.

### Villager
The villager is a very basic role that does not get an ability or extra information, however, villagers will make up a majority of the group. The villagers win by executing all werewolfs.

### Werewolf
The werewolf is an active role that has the ability kill one villager every night. There are few werewolfs in the group, but they know identity of all the other werewolfs. Werewolfs win by killing all of the villagers.

### The Witch
The witch is a special villager who has one healing potion and one poison potion. During the night, the witch can choose to use the healing potion to save the person that got attacked by the werewolf, or use the poison potion to kill one person. Each potion can only used once per game. The witch wins when all of the werewolfs are dead. 

### The Oracle
The oracle is a special villager that can find out the identities of other group members. During each night, the oracle chooses one player and learns their role. The oracle wins when all of the werewolfs are dead.

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
