# Hashes

Hashing is the process of mapping any amount of data to a specified size using an algorithm. This is known as a hash value. Hashing is a one-way function, whereas encryption is a two-way function. Unlike encryption, which is intended to protect data in transit, hashing is intended to authenticate that a file or piece of data has not been altered—that it is authentic. In other words, it functions as a checksum.

## Common hashing algorithms

### MD5

This is one of the first algorithms that has gained widespread acceptance. MD5 is hashing algorithm made by Ray Rivest that is known to suffer vulnerabilities. It was created in 1992 as the successor to MD4. Currently, MD6 is in the works, but as of 2009 Rivest had removed it from NIST consideration for SHA-3.

### SHA

SHA stands for Security Hashing Algorithm, and it’s probably best known as the hashing algorithm used in most SSL/TLS cipher suites. A cipher suite is a collection of ciphers and algorithms that are used for SSL/TLS connections. SHA handles the hashing aspects. SHA-1, as we mentioned earlier, is now deprecated. SHA-2 is now mandatory. SHA-2 is sometimes known has SHA-256, though variants with longer bit lengths are also available.

### SHA256

SHA 256 is a member of the SHA 2 algorithm family, under which SHA stands for Secure Hash Algorithm. It was a collaborative effort between both the NSA and NIST to implement a successor to the SHA 1 family, which was beginning to lose potency against brute force attacks. It was published in 2001.
The importance of the 256 in the name refers to the final hash digest value, i.e. the hash value will remain 256 bits regardless of the size of the plaintext/clear-text. Other algorithms in the SHA family are similar to SHA 256 in some ways.

### Luhn

The Luhn algorithm, also renowned as the modulus 10 or mod 10 algorithm, is a straightforward checksum formula used to validate a wide range of identification numbers, including credit card numbers, IMEI numbers, and Canadian Social Insurance Numbers. A community of mathematicians developed the LUHN formula in the late 1960s. Companies offering credit cards quickly followed suit. Since the algorithm is in the public interest, anyone can use it. The algorithm is used by most credit cards and many government identification numbers as a simple method of differentiating valid figures from mistyped or otherwise incorrect numbers. It was created to guard against unintentional errors, not malicious attacks.

## Python Hash

Python uses siphash24 algorithm for hash calculation. https://peps.python.org/pep-0456/

```python
>>> sys.hash_info
sys.hash_info(width=64, modulus=2305843009213693951, inf=314159, nan=0, imag=1000003, algorithm='siphash24', hash_bits=64, seed_bits=128, cutoff=0)

>>> def fnv(p):
        if len(p) == 0:
            return 0
        # bit mask, 2**32-1 or 2**64-1
        mask = 2 * sys.maxsize + 1
        x = hashsecret.prefix
        x = (x ^ (ord(p[0]) << 7)) & mask
        for c in p:
            x = ((1000003 * x) ^ ord(c)) & mask
        x = (x ^ len(p)) & mask
        x = (x ^ hashsecret.suffix) & mask
        if x == -1:
            x = -2
        return x
```

/*[clinic input]
class dict "PyDictObject *" "&PyDict_Type"
[clinic start generated code]*/
/*[clinic end generated code: output=da39a3ee5e6b4b0d input=f157a5a0ce9589d6]*/

/*
To ensure the lookup algorithm terminates, there must be at least one Unused slot (NULL key) in the table.
To avoid slowing down lookups on a near-full table, we resize the table when it's USABLE_FRACTION (currently two-thirds) full.
*/

#define PERTURB_SHIFT 5

/*
Major subtleties ahead:  Most hash schemes depend on having a "good" hash function, in the sense of simulating randomness.  Python doesn't:  its most important hash functions (for ints) are very regular in common cases:

>>>[hash(i) for i in range(4)]
[0, 1, 2, 3]

This isn't necessarily bad!  To the contrary, in a table of size 2**i, taking the low-order i bits as the initial table index is extremely fast, and there are no collisions at all for dicts indexed by a contiguous range of ints. So this gives better-than-random behavior in common cases, and that's very desirable.

OTOH, when collisions occur, the tendency to fill contiguous slices of the hash table makes a good collision resolution strategy crucial.  Taking only the last i bits of the hash code is also vulnerable:  for example, consider the list [i << 16 for i in range(20000)] as a set of keys.  Since ints are their own hash codes, and this fits in a dict of size 2**15, the last 15 bits of every hash code are all 0:  they *all* map to the same table index.

But catering to unusual cases should not slow the usual ones, so we just take the last i bits anyway.  It's up to collision resolution to do the rest.  If we *usually* find the key we're looking for on the first try (and, it turns out, we usually do -- the table load factor is kept under 2/3, so the odds are solidly in our favor), then it makes best sense to keep the initial index computation dirt cheap.

The first half of collision resolution is to visit table indices via this recurrence:

```shell
    j = ((5*j) + 1) mod 2**i
```

For any initial j in range(2**i), repeating that 2**i times generates each int in range(2**i) exactly once (see any text on random-number generation for proof).  By itself, this doesn't help much:  like linear probing (setting j += 1, or j -= 1, on each loop trip), it scans the table entries in a fixed order.  This would be bad, except that's not the only thing we do, and it's actually *good* in the common cases where hash keys are consecutive.  In an example that's really too small to make this entirely clear, for a table of size 2**3 the order of indices is:

```text
    0 -> 1 -> 6 -> 7 -> 4 -> 5 -> 2 -> 3 -> 0 [and here it's repeating]
```

If two things come in at index 5, the first place we look after is index 2, not 6, so if another comes in at index 6 the collision at 5 didn't hurt it. Linear probing is deadly in this case because there the fixed probe order is the *same* as the order consecutive keys are likely to arrive.  But it's extremely unlikely hash codes will follow a 5*j+1 recurrence by accident, and certain that consecutive hash codes do not.

The other half of the strategy is to get the other bits of the hash code into play.  This is done by initializing a (unsigned) vrbl "perturb" to the full hash code, and changing the recurrence to:

```text
    perturb >>= PERTURB_SHIFT;
    j = (5*j) + 1 + perturb;
    use j % 2**i as the next table index;
```

Now the probe sequence depends (eventually) on every bit in the hash code, and the pseudo-scrambling property of recurring on 5*j+1 is more valuable, because it quickly magnifies small differences in the bits that didn't affect the initial index.  Note that because perturb is unsigned, if the recurrence is executed often enough perturb eventually becomes and remains 0.  At that point (very rarely reached) the recurrence is on (just) 5*j+1 again, and that's certain to find an empty slot eventually (since it generates every int in range(2**i), and we make sure there's always at least one empty slot).

Selecting a good value for PERTURB_SHIFT is a balancing act.  You want it small so that the high bits of the hash code continue to affect the probe sequence across iterations; but you want it large so that in really bad cases the high-order hash bits have an effect on early iterations.  5 was "the best" in minimizing total collisions across experiments Tim Peters ran (on both normal and pathological cases), but 4 and 6 weren't significantly worse.

Historical: Reimer Behrends contributed the idea of using a polynomial-based approach, using repeated multiplication by x in GF(2**n) where an irreducible polynomial for each table size was chosen such that x was a primitive root. Christian Tismer later extended that to use division by x instead, as an efficient way to get the high bits of the hash code into play.  This scheme also gave excellent collision statistics, but was more expensive:  two if-tests were required inside the loop; computing "the next" index took about the same number of operations but without as much potential parallelism(e.g., computing 5*j can go on at the same time as computing 1+perturb in the above, and then shifting perturb can be done while the table index is being masked); and the PyDictObject struct required a member to hold the table's polynomial.  In Tim's experiments the current scheme ran faster, produced equally good collision statistics, needed less code & used less memory.

*/
