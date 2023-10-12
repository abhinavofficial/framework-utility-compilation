# Dynamic Programming

In a regular recursion, say fibonacci calculation, we calculate the value of calling subtree over and over. It makes time complexity exponential. In fact, it is `O(2**n)` for fibonacci calculation.

This problem of overlapping sub-problem is known as dynamic programming. While trying to use Dynamic Programming,

* Notice any overlapping sub-problems.
* Decide what is the trivially smallest input (base case)
* think recursively to use Momoization
* think iteratively to use Tabulation
* Draw a strategy first!!!

We use memoization to store the value of solved sub-problem, so we can just look up without calculating. So, if we can maintain keys as args to the function, and value as the calculated value which can returned.

Memoization recipe

* Make it work
  * Visualize the problem as a tree
  * Implement the tree using recursion. Base case are critical.
  * Test it
* Make it efficient
  * Add a memo object (a dict), with keys as args of the function and value as return values of the function
  * This memo object should be shared across the function
  * Add a base case to return memo values if the key is present.
  * Store return values into the memo

* Numeric
  * canSum -> Decision Problem
  * howSum -> Combinatoric Problem
  * bestSum -> Optimization problem
* String Use of prefix
  * canConstruct
  * countConstruct
  * allConstruct

https://github.com/abhinavofficial/prep-inteview/blob/main/DynamicProgramming.py

Tabulation
This is strategy where we solve dynamic programming problem in a table format. This involves iteration instead of recursion.

Memoization recipe

* Make it work
  * Visualize the problem as a table
  * Size the table based on the inputs (take care of out of bound condition)
  * initialize the table with default values [with some valid number if a number problem or with some valid boolean if a decision problem, etc]
  * seed the trivial answer into the table (take the most trivial case whose answer we already know)
  * iterate through the table
  * fill further positions based on the current position

https://en.wikipedia.org/wiki/Dynamic_programming

Practice problems
https://leetcode.com/tag/dynamic-programming

https://www.udemy.com/course/master-the-art-of-dynamic-programming/?utm_source=adwords&utm_medium=udemyads&utm_campaign=LongTail_la.EN_cc.INDIA&utm_content=deal4584&utm_term=_._ag_118445032537_._ad_533094112755_._kw__._de_c_._dm__._pl__._ti_aud-1738475842996%3Adsa-1212271230479_._li_9062084_._pd__._&matchtype=&gclid=Cj0KCQjw6KunBhDxARIsAKFUGs9fe3JuBF836W4dA5GuV5EtrJs5wTo_F-Y2Ca45H2Nsp4U49HHMF40aAsGqEALw_wcB

https://www.educative.io/courses/grokking-dynamic-programming-a-deep-dive-using-java/course-overview?eid=5082902844932096&gclid=Cj0KCQjw6KunBhDxARIsAKFUGs_GR64OBJhEoT5z_Kbi5jDEtbVbhJ9UknT_IWJ6cPkA-wLRXo5lO3oaAsLrEALw_wcB&hsa_acc=5451446008&hsa_ad=&hsa_cam=18931439518&hsa_grp=&hsa_kw=&hsa_mt=&hsa_net=adwords&hsa_src=x&hsa_tgt=&hsa_ver=3&utm_campaign=brand_educative&utm_campaign=%5BNew%5D%20Performance%20Max&utm_content=performance_max_india&utm_medium=ppc&utm_medium=ppc&utm_source=google&utm_source=adwords&utm_term=

https://www.educative.io/courses/grokking-modern-system-design-interview-for-engineers-managers?eid=5082902844932096&utm_content=search&utm_medium=cpc&utm_source=google&utm_campaign=system-design&term=sitelink&utm_term=&utm_campaign=%5BNew%5D+Performance+Max&utm_source=adwords&utm_medium=ppc&hsa_acc=5451446008&hsa_cam=18931439518&hsa_grp=&hsa_ad=&hsa_src=x&hsa_tgt=&hsa_kw=&hsa_mt=&hsa_net=adwords&hsa_ver=3&gclid=Cj0KCQjw6KunBhDxARIsAKFUGs8-HKXDUBUoZxTgNm6PQrJhZc-4JlsNPWaHIfAAcYyqJaeSeR77H10aAr3ZEALw_wcB

http://web.mit.edu/15.053/www/AMP-Chapter-11.pdf
https://www.codechef.com/wiki/tutorial-dynamic-programming
https://www.coursera.org/learn/delivery-problem/home/week/1
https://medium.com/techie-delight/top-50-dynamic-programming-practice-problems-4208fed71aa3

    Longest Common Subsequence | Introduction & LCS Length
    Longest Common Subsequence | Finding all LCS
    Longest Common Substring problem
    Longest Palindromic Subsequence using Dynamic Programming
    Longest Repeated Subsequence Problem
    Implement Diff Utility
    Shortest Common Supersequence | Introduction & SCS Length
    Shortest Common Supersequence | Finding all SCS
    Longest Increasing Subsequence using Dynamic Programming
    Longest Bitonic Subsequence
    Increasing Subsequence with Maximum Sum
    The Levenshtein distance (Edit distance) problem
    Find size of largest square sub-matrix of 1’s present in given binary matrix
    Matrix Chain Multiplication using Dynamic Programming
    Find the minimum cost to reach last cell of the matrix from its first cell
    Find longest sequence formed by adjacent numbers in the matrix
    Count number of paths in a matrix with given cost to reach destination cell
    0–1 Knapsack problem
    Maximize the Value of an Expression
    Partition problem | Dynamic Programming Solution
    Subset Sum Problem
    Minimum Sum Partition Problem
    Find all N-digit binary strings without any consecutive 1’s
    Rod Cutting Problem
    Maximum Product Rod Cutting
    Coin change-making problem (unlimited supply of coins)
    Coin Change Problem (Total number of ways to get the denomination of coins)
    Longest Alternating Subsequence Problem
    Count number of times a pattern appears in given string as a subsequence
    Collect maximum points in a matrix by satisfying given constraints
    Count total possible combinations of N-digit numbers in a mobile keypad
    Find Optimal Cost to Construct Binary Search Tree
    Word Break Problem | Dynamic Programming
    Word Break Problem | Using Trie Data Structure
    Total possible solutions to linear equation of k variables
    Wildcard Pattern Matching
    Find Probability that a Person is Alive after Taking N steps on an Island
    Calculate sum of all elements in a sub-matrix in constant time
    Find Maximum Sum Submatrix in a given matrix
    Find Maximum Sum Submatrix present in a given matrix
    Find maximum sum of subsequence with no adjacent elements
    Maximum Subarray Problem (Kadane’s algorithm)
    Single-Source Shortest Paths — Bellman Ford Algorithm
    All-Pairs Shortest Paths — Floyd Warshall Algorithm
    Pots of Gold Game using Dynamic Programming
    Find minimum cuts needed for palindromic partition of a string
    Maximum Length Snake Sequence
    3-Partition Problem
    Calculate size of the largest plus of 1’s in binary matrix
    Check if given string is interleaving of two other given strings