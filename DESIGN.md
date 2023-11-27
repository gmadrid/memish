# Name ideas

I want something that implies that it's memdeck work without actually
using the word "memdeck." "Memish" will be my working name, but may
change before we're done. 

__List additional ideas here__


# Short description

A tool that uses spaced repetition (specifically a Leitner box)
to help learn a memdeck.

- Should work with the standard options:
  - Mnemonica
  - Memorandum
  - Faro 5
  - Redford
  - Aronson
- Should allow an individual to work with multiple stacks - separately.
- Introduce new cards slowly, and only when the current set of 
  cards has been correctly guessed.
- Allows learning a half-stack
- Does spaced repetition
- Allows a timer
- keeps track of stats per card
  - successes
  - fails
  - correct percentage
  - highest level

## The Leitner box



# Intent/goal

# Basic game phases / loops

# Minimum viable product

# Random Ideas

# Details

## AppState

`MainMenu` - Game starts here. Has options to 'Play', 'Prefs', 'Quit'. Also
has a brief blurb and some game status information. 
  - Active questions
  - Missed on last attempt
  - Questions in next session

`Prefs` - Allows setting options:
  - Stack (Select)
    - Mnemonica
    - Memorandum
    - Aronson
    - Redford
    - Faro 5
  - Half stack (Checkbox)
  - Question types (multi-select)
    - Card to index
    - Index to card
    - Next card
    - Prev card
  - Time limits (Select)
    - No limit
    - 10, 5, 3, 2, 1 secs
  - Max questions per game (Select)
    - 40, 20, 10, 5

`Play` - the game is running

`Done` - The game is over. Give stats for the previous game, and option to 
  play again, prefs, or quit.

## Main menu

A very simple menu. Just a title, and the three buttons, all in a centered 
column on the screen. 

> TODO: You need to pick some better colors here.

