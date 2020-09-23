#[derive(Debug)]
pub enum Tree {
    Base(Box<String>),
    Enc(Box<Tree>, Box<Tree>),
    Hash(Box<Tree>),
    Pair(Box<Tree>, Box<Tree>),
}
