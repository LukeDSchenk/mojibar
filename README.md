# 🙂👭🏂🐳 Mojibar 🐆🦎©🈲🆖😯

A quick access tab and search bar for emojis. Simply click the emoji you want to use and it will be copied to your clipboard. Want to try it out before you decide if you like it? Click here to [try out Mojibar in your browser](https://lukedschenk.github.io/Mojibar/) (note that the desktop version will offer more than the basic click-to-copy functionality).

# Installation Instructions 📄

## ⚡ The lightning quick easy method ⚡

Simply run this command in your unix terminal and badabing badaboom, Mojibar will be installed!

*I haven't actually figured out the command yet* 🚌🆙🚌🆙

## Linux 🐧

Linux instructions to come soon.

## Mac 🍎

Mac instructions to come soon.

## Windows 🪟

If you're a chach and you use windows you need to be patient 🥴.

## Compiling from source 🖥⌨

Before proceeding, do yourself a favor and [install Rust](https://www.rust-lang.org/tools/install).

# Known Issues

* Emojis are all monochrome! (I know it sucks! I didn't realize when I started but the UI framework I used makes all emojis monochrome. I thought this problem would be fixed by using a custom font but it was not ☹. I will do whatever it takes to remedy this, I just hope it doesn't end up forcing me to switch UI frameworks.)
* The search bar has some weird issues where certain substrings will match on emojis where other more "complete" substrings of a word will not. Actively looking into the cause of this.
* Pasting emojis into the search bar uses a different font and it's weird.

# Coming Soon 🕦

* Better documenatation
* UI improvements
* Search improvements
* Select emoji with arrow keys (currently only works with tabbing or clicking)
* Keyboard shortcuts and other speed of use improvements

## Future 📡🚀🤖

Features I want but am not prioritizing right now.

* Favorite emojis
* Emoji categories
* Custom fonts?
* Basically a cooler copy of the emoji functionality of slack or IOS, etc. but it works on any platform and isn't tied to a single app
* Run this program as a daemon of sorts that is bound to a keyboard shortcut so it can be accessed and closed extremely quickly
* Emojipedia integration! For learning 📒
* Some kind of API for integrating into other apps so each app doesn't have to make their own damn emoji toolbar???!
* More supported emojis
