use crate::tree::Tree;

pub fn evaluate(tree: &Tree) -> u32 {
    score(tree, false)
}

fn score(tree: &Tree, internal: bool) -> u32 {
    match tree {
        Tree::Base(_) => 1,
        Tree::Enc(m, k) => 1 + score(&m, true) + score(&k, true),
        Tree::Hash(c) => 1 + score(&c, true),
        Tree::Pair(m1, m2) => { let o = if internal { 50 } else { 0 };
                          o + score(&m1, internal) + score(&m2, internal)
                        },

    }
}
