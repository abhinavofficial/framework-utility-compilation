# Blockchain

## hash function
Takes an arbitrary length input and returns a unique deterministic fixed length output. It is a one way function - you can generate output but cannot regenerate the original value from the hash output. There are many hash engines like Hashing engines supported: md2, md4, md5, sha1, sha224, sha256, sha384, sha512, ripemd128, ripemd160, ripemd256, ripemd320, whirlpool, tiger128,3, tiger160,3, tiger192,3, tiger128,4, tiger160,4, tiger192,4, snefru, snefru256, gost, gost-crypto, adler32, crc32, crc32b, fnv132, fnv1a32, fnv164, fnv1a64, joaat, haval128,3, haval160,3, haval192,3, haval224,3, haval256,3, haval128,4, haval160,4, haval192,4, haval224,4, haval256,4, haval128,5, haval160,5, haval192,5, haval224,5, haval256,5, etc.

[Online Hash Password](https://www.tools4noobs.com/online_tools/hash/)


Rainbow tables - is a precompiled tables of passwords and its hash values. To avoid, some companies like Google, will add some salt to the password - this is random generated value based on some algorithm and now this is what is hashed and saved.

Hash is 

## Encryption
Two-way function

It is a means of securing data by encoding it

Data encryption is the process that translates the data from its original form to another form. The original form of data is known as plaintext and the encrypted form of data is known as ciphertext. The ciphertext is decrypted by a secret key.

[Difference between both of them](https://www.baeldung.com/cs/hashing-vs-encryption)

There are two types of encryption. Symmetric and Asymmetric encryption.

Asymmetric Encryption - public key and private key.

* Create the cipher text using public key, and I can decipher it using private key. Example - Message interaction like Whatsapp
* Create the cipher text using private key, and I can decipher it using public key. Example - Digital signature

We will use Asymmetric encryption in Blockchain.