# To Do 📝

* Write a Python script to generate some Rust code for the EMOJIS data structure instead of having it parsed out of a JSON file at runtime (or find a better way to do that)

## Search issues 🔎

* handle duplicate search results due to keyword and name overlap (this might be best handled by pursuing the search "filtering" idea)
* Make general improvements to the radix tree crate I am using and reduce the inconsistency in the results
  * This includes allowing vectors of data and appending the data of duplicate keywords to to a node's vector of data instead of overwriting what is there
* Just make it less dogshit 💩

## UI issues 📺

* Figure out color emojis in egui (talk to the creator if need be)
* Ponder the idea of filtering with search and having a one pane UI
  * Favorites tab, and all emojis tab
  * search could work on only the open tab?? or maybe that's confusing
  * maybe just a frequently used or recent emojis is better
  * just copy slack? 😂😂😂😂😂😂😂😂😂
