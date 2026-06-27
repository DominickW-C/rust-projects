# Basic Calculator

Only does basic math. This was to focus on learning syntax, and how
coding in rust works. 

## Takeaways
- The basics
- syntax
- modules
- matching with enums

# Version 2 

## what chaged

I ran it through gemini to see how I could make it better. I had a 
match in a match so I did not have to create a new variable, but 
turns out that I could use a match on the same line as a parse to
make num1 a float, and return if it was an error, meaning only one
var was needed, and no more nesting match statements.

## Takeaways
- you can use match and parse together

