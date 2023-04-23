# Solana Cpis Depth test
There are two Main programs in this repo:
1 - An "Adder" (That receives a number and adds it to an accumulator)
2 - A Counter (That adds 1 to a counter every time it gets called)

The first one receives a number as a param in its unique ix, the latter has no params.

It also provides 4 Proxy programs, numerated in reversed order from 3 to -1, which unique ix executes a cpi call forwarding accounts and params.

The motivation was to test the CPI depth of Solana, currently set to 4.

# Usage
Once deployed all programs and set their clients with the returned Ids, pass a private key in cli/constants from the program you want to execute the tx. 

Spoiler: Executing the sequence {0, 1, 2, 3, Add} works. {-1, 0, 1, 2, 3, Add} doesn't.



