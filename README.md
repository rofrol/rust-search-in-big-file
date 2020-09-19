# rust-search-in-big-file

I got interview question how would I search in 2 GB file.

## Links

- https://users.rust-lang.org/t/optimizing-string-search-code-for-large-files/31992
  - You can build an index and get down to 2ms. That's what I can get from solr. https://users.rust-lang.org/t/optimizing-string-search-code-for-large-files/31992/9
- https://users.rust-lang.org/t/solved-how-to-scan-efficiently-big-binary-streams-files/6345
- https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#access-a-file-randomly-using-a-memory-map
- https://www.reddit.com/r/rust/comments/9ggtwf/rustmemchr_a_crate_for_fast_byte_searching_that/
