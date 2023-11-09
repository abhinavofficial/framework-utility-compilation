# Byzantine Generals Problem and Fault Tolerance

Failure detection systems employed
* A node could appear both failed, and working
* Differs from observer to observer
* Nodes must correctly identify any failed components
* Through a consensus

Byzantine fault: Different observers seeing different symptoms

Byzantine failure: Loss of a system device, when byzantine fault occurs. It is the most general, and difficult class of failures.

The whole byzantine fault and problem stems its from Byzantine Generals Problem and is a game theory problem.

## Byzantine Generals Problem

Let's understand byzantine generals' problem in a simple form. Imagine several byzantine army divisions camped outside enemy city. There is a general per division. They can win if every general attacks or else will lose. Generals can agree to retreat as well. They are far off from each other and hence must communicate via messengers. There could be traitors among the generals, and they could act arbitrarily and may sabotage the plan. They may cast votes in favor of retreat and attack at the same time. Loyal generals must reach a consensus for a plan irrespective of traitors.

A commanding general must send an order to his n-1 lieutenant generals such that:
* Consistency / Agreement: All loyal lieutenants obey the same order
* Validity: If the commanding general is loyal, all lieutenants obey the order he sends
* Liveness / Termination: If Commanding General is loyal.

### Impossibility Result

If Generals can send only Oral Messages, then a solution will work only if at least two-thirds of them are Loyal.

Oral Message:
* Sent message is correctly delivered
* Authenticated Channel: The receiver knows who sent the message
* Synchronous Network: We can detect if message goes missing

Restating the Impossibility Result
In a synchronous network with an authenticated channel, if m Generals are traitors, then a solution will work only if there are at least 3m Generals


Case f =1
* Scenario 1: Commandant is Loyal, 1 Traitor Lieutenant.
  * Commandant: (1) ATTACK or (2) RETREAT. Remaining Lieutenants should (1) ATTACK or (2) RETREAT
* Scenario 1: Commandant is Traitor.
  * * Commandant: ATTACK to Lieutenant 1, RETREAT to Lieutenant 2. Consistency Broken! No solution!


## Byzantine Consensus Protocol (23 mins)


[Byzantine General Problem](https://zwyuan.github.io/2016/05/11/pr-lamport-tpls82-byzantine-general-problem/)