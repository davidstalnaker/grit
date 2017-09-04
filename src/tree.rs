
pub enum Tree {
    BlobEntry { name: String, hash: String },
    TreeEntry {
        name: String,
        hash: String,
        children: Vec<Tree>,
    },
}
