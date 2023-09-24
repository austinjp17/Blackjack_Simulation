# Blackjack Simulation

## The Pack

A standard 52-card deck is used, however most casinos will shuffle multiple decks together. 6 decks (312 cards) is the most popular iteration. Casinos will include a blank card initially toward the bottom to indicate when to shuffle.

## Goal

Each player attempts to beat the dealer by getting as close to 21 as possible without going over.

## Card Values

- Ace: 11 unless it would cause the participant to bust, then 1.

- Face Cards: 10

- All Else: Pip value

## Betting

Before inital deal, each player places a bet between designated minimum and maxiumum limits. General limits are $2-$500.

## Shuffle and Cut

The dealer throughly shuffles the deck, designates a player to cut. **The plastic insert card is placed to that the last 60-70 cards are not used, making card counting more difficult.**

## The Deal

Post inital bets, the dealer gives one faceup card to all players in clockwise rotation, then one card faceup to themselves. Another round of faceup cards is dealt to the players, and the dealer receives a facedown card. (Some games have players recieve cards facedown, but virtually all casinos use faceup with no touching.)

_Does it matter where you sit?_

## Naturals

If a player's first two cards are an ace and 10 pip card, giving a count of 21, this is a natural, or "blackjack". If any player has a natural and the dealer does not, the dealer immediately pays 1.5 times their bet. If the dealer has a natural, they immediately collect the bets of all players who do not have naturals themselves. A _standoff_ occuers when the dealer and a player both have naturals, the player gets their chips back w/ no additional winnings.

## The Play

The left player goes first. Decides whether to "stand" (stay where they are) or "hit" (get another card). The dealer continues with a specific player until they hold.

## Dealer Play

Once every player is served, the dealers facedown card is overturned.

- If sum >= 17: Must stand

- if sum <= 16: Must hit

If the dealer has an ace whiched counted as 11, puts them at or over the 17 mark, they must stand. Can not treat as 1.

## Splitting Pairs

If the players first two cards are the same denomination, the may choose to treat them as seperate hands when their turn comes. The original bet goes on one of the cards, and an equal amount must be placed on the other. The player plays the hand on the left with the dealer, then right. Each hand is treated seperately, and the dealer settles with each on it's own merits.

**Pair of Aces**: The player is given one card for each ace and not allowed to draw again. If they recieve a 10-card on one of these draws, the payoff is equal to the bet (not 1.5 times).

## Doubling Down

Players may double their bet when the original cards equal 9, 10, 11. On their turn, they place a bet equal to their original bet, and the dealer gives just one card placed facedown, revealed only when bets are settled at the end of the hand. With two fives, a player may split, double down, or just play normally.

**The dealer doesn't have the ability to double down or split**

## Insurance

When a dealers face-up card is an ace, any player may make a side bet up to half their original that the dealer's face-down card is 10-pip. Once all insurance bets are placed, the dealer looks at their hidden card and if it is 10-pip, pay double the insurance bets. (2-1 payoff)

_Perhaps if we know there are an unusually high amount of 10's left in the deck?_

## Settlement

A key advantage the house has is that all players go first. If the house busts it only has to pay out players who stood. If a player and dealer have the same sum value, no chips are exchanged.

## Reshuffling

When each players bet is settled, the games cards are collected and set aside. Each game's cards are removed from play until the empty card is dealt, at which point all cards will be reshuffled, cut, and empty card replaced.

## Basic Strategy

- Dealer Up-card

  - Good (7, 8, 9, 10, 11): Draw until 17 or more

  - Fair (2, 3): Draw until 13 or greater

  - Poor (4, 5, 6): Draw until 12 or greater

    - Strategy here is never take a card if any chance of bust. The goal is to let the dealer bust.

- Doubling Down

  - 11: Always double down

  - 10: Double down unless 10 or ace dealer upcard

  - 9: Double down if dealer upcard is fair or poor (2-6)

- Splitting

  - Always split aces and eights

  - Don't split 10's, 5's, or 4's

    - 5's = 10, generally more effective to double down

  - Generally split 2s, 3s, or 7s

    - unless dealer has 8, 9, 10 or ace

  - Only Split 6s if dealer card is poor

### Unit Tests

To run the unit tests across all the libraries:

```bash
cargo test --workspace
```
