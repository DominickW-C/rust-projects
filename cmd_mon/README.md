# Text Based Pokemon Battles

Simulates a Pokemon battle in the command line

## Takeaways
- File handling
- Stdin

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
