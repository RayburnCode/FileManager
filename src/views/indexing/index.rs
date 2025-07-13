struct FileIndex {
    trie: RadixTrie<PathBuf>,      // Prefix search
    content_index: TantivyIndex,    // Full-text
    last_modified: BTreeMap<SystemTime, PathBuf>
}

impl FileIndex {
    fn search(&self, query: &str) -> Vec<PathBuf> {
        // SIMD-accelerated search
        if query.len() < 4 {
            self.trie.get_prefix(query) 
        } else {
            self.content_index.search(query)
        }
    }
}