This folder contains some different files storing emoji data. As of right now, Mojibar loads emojis in from 'emoji-min.json'. The file is included into the binary at compile time, and then converted into a Rust data structure at runtime.
Ideally, all of that would happen at compile time. This would require me to write out all of the emoji data as Rust code.
I plan on writing a script to do that at some point.

emoji-min.json is a copy of emoji.json with all emojis currently not working with the OpenMoji font removed (I am almost certain these are currently just emojis with a length greater than 1 character (created by a combination of more than one unicode char).
