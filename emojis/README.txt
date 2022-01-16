This folder contains some different files storing emoji data. As of right now, Mojibar loads emojis in from 'emojis.txt' with no additional data involved. 

* emoji-min.json is a copy of emoji.json with all emojis currently not working with the OpenMoji font removed (I am almost certain these are currently just emojis with a length greater than 1 character (created by a combination of more than one unicode char)

## Emoji Trie Notes
Create a Trie data structure, where the branches are the keywords of each emoji. The data stored on the end nodes of said branches will then be a struct containing the actual emoji and its name. 
