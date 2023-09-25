# Webapp for FEAL4 Cryptanalysis

What do I want?

I want to read the graph.json produced, and make a webapp that allows me to enter deltas (in the differential cryptanalysis sense), and see them propagate through the system.

The graph is _assumed_ to be topologically ordered, so that edge["src"] < edge["dst"] is _always_ true.

So, there will really only be _one_ input: that delta.

In the Rust code tho, I want to be able to have primitives:

1. `<plaintext graph={graph} self={idx} />`
2. `<key0 graph={graph} self={idx} />`
3. `<key1 graph={graph} self={idx} />`
4. `<key2 graph={graph} self={idx} />`
5. `<key3 graph={graph} self={idx} />`
6. `<key4 graph={graph} self={idx} />`
7. `<key5 graph={graph} self={idx} />`
8. `<key6 graph={graph} self={idx} />`
9. `<key7 graph={graph} self={idx} />`
10. `<key8_11 graph={graph} self={idx} />`
11.  `<key12_15 graph={graph} self={idx} />`
12. `<xor graph={graph} self={idx} a={idx} b={idx} />`
13. `<left graph={graph} self={idx} src={idx} />`
14. `<right graph={graph} self={idx} src={idx} />`
15.  `<F graph={graph} self={idx} key={idx} data={idx}/>`
16. `<copy graph={graph} self={idx} src={idx} />`
17. `<swap graph={graph} self={idx} left={idx} right={idx} />`
18. `<ciphertext graph={graph} self={idx} inp={idx} />`

So.. Maybe on creation of the tag, it registers it's "self" id with graph?

Or. Maybe for now, lemme just make it do the encryption and then later we can make it do all the deltas etc.
