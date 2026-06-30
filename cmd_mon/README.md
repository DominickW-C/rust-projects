# Text Based Pokemon Battles

Simulates a Pokemon battle in the command line

## Takeaways
- File handling
- Stdin

## Credit

- list of pokemon from pokemondb
- list can also be found from killshot13 on github
- pokemon info from bulbapedia

## Python script

There is over 1000 pokemon and entering in all those stats would
be very annoying. I have been getting the information previously
from bulbapedia and thought I could scrape the pages since all the
URLs are the same besides the pokemon name. I got a list of all
names, loop through it, and scrape each page for the stats. Sadly
the table does not have tags on the website, hence why numbers are
found with the title and I have to call next a ton.  

#### v2

The first had tons of problem with space in names, and HP/Attack
being mentioned before the stats table. Fixed this to pull from only
the table. It's messy but it works, probably won't clean up cause it
was a one time script to save time.

*TODO*

Currently can open files and read each line.
I now need to make a function that takes every line,
splits it at the commas, and puts the values into a struct
for the pokemon to be used. I don't think I will need a struct
unless the Pokemon is in battle 

Find amount of lines in dex so with battles a random line can
be found for a battle

*6/28*

Added a team file that appends when a new pokemon is added to it.
I will also need to add something to make sure that it does not 
exceed 6 pokemon which should not be hard, just read and make sure
it is less than 7 lines with the header. I might be able to remove
the header anyways now that I think of it. I really need to clean
the code up a bit, and fix it since the pokemon for the team are
being loaded from the dex and not the team. Main is getting too big.
How the team file is being opened seems a bit weird but it works how
I want it to (it opens in the function every time, it should close
itself though once the function finishes since it is out of scope?).

*6/29*

Fixed the UI a bit. Made it so it you select a starter it add it to
the team vec. Cleaned up the code. Also found out you can iterate
over multiple iterators in a for loop which was very helpful. 
