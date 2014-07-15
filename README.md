# An implementation of a 2-3 tree in Rust

This is **not** a 2-3 B-Tree, but rather what Aho/Hopcroft/Ullman call a 2-3 tree:

 - interior nodes either have two or three children
 - all (and only) data is at the leaves
 - all leaf nodes are equal distance from the root.
 - interior nodes have two data fields: the first field records the smallest element that is a descendant of the second child. the second field exists iff the third child is present, and contains the smallest element that is a descendant of the third child
