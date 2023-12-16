# DualHashKey

An implementation of a 64-bit Dual-Hash-Key, strongly inspired by the Linux kernels dcache hashes.

The primary use-case is to use it as key-type for an ordered collection,
implicitely making the collection hierarchical thru the DHK,
allowing to quickly find subkeys and permit range queries.

**TODO:** More README
