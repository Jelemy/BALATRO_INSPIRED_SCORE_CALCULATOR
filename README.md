# Balatro-Inspired Score Calculator

A fan-made tool inspired by [**Balatro**](https://store.steampowered.com/app/2379780/Balatro/) that calculates the score of a poker hand enhanced by cards, editions, and jokers—just like in the game!

Give it a hand in **YAML format** structured like the following, and it’ll output the score.

```yaml
cards_played:
  - A♦ Glass Polychrome
  - K♦ Mult Holographic
  - Q♦ Bonus Foil
  - J♦ Wild
  - 10♦ Steel

cards_held_in_hand:
  - K♠ Steel Foil

jokers:
  - Splash Foil
  - Sock And Buskin Holographic
  - Zany Joker Polychrome
```

## Input Format (`.yml`)

The input is a `.yml` file with three sections:
- `cards_played`: a list of cards played in the current hand, each with optional enhancements and editions.
- `cards_held_in_hand`: optional list of other cards in hand.
- `jokers`: a list of joker cards currently active.

Each card can have multiple **enhancements** and **editions**, applied as suffixes in the same string.


## Features Supported

### Illegal Poker Hands
- Five of a Kind  
- Flush House  
- Flush Five

### Card Modifiers

**Enhancements:**
- Bonus  
- Mult  
- Wild  
- Glass  
- Steel  

**Card Editions:**
- Foil  
- Holographic  
- Polychrome  

### Supported Jokers

**Classic Jokers:**
- Joker  
- Jolly Joker  
- Zany Joker  
- Mad Joker  
- Crazy Joker  
- Droll Joker  
- Sly Joker  
- Wily Joker  
- Clever Joker  
- Devious Joker  
- Crafty Joker  
- Abstract Joker  

**Theme/Effect Jokers:**
- Raised Fist  
- Blackboard  
- Baron  
- Greedy Joker  
- Lusty Joker  
- Wrathful Joker  
- Gluttonous Joker  
- Fibonacci  
- Scary Face  
- Even Steven  
- Odd Todd  
- Photograph  
- Smiley Face  
- Flower Pot  

**Combo/Utility Jokers:**
- Four Fingers  
- Shortcut  
- Mime  
- Pareidolia  
- Splash  
- Sock and Buskin  
- Smeared Joker  
- Blueprint  

---
