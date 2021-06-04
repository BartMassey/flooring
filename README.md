# flooring: solution to Floor Designer problem
Bart Massey

This is a program that (mostly) solves the given instance of
the problem *Floor Designer* as proposed
[here](https://www.reddit.com/r/dailyprogrammer_ideas/comments/npxzvr/intermediate_floor_designer/). It
employs a two-level depth-first search:

* First calculate the set of legal flooring rows.  Each row
  is ordered from largest board to smallest.  Turns out
  there are nine possible rows.

* Second, try to lay rows keeping track of used
  material. The search order is from
  smallest-number-of-boards rows toward larger; this isn't
  quite right (because there are more smaller boards in this
  instance) but is good enough.

Turns out that this solution is total overengineering for
this particular instance: greedy placement of boards from
longest to shortest would find the solution without
search. Ah well — maybe a harder instance will be given.

On my fast desktop running Linux, the solution is found in
less than 5ms.
