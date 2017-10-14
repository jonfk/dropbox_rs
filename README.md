[![Build Status](https://travis-ci.org/jonfk/dropbox_rs.svg?branch=master)](https://travis-ci.org/jonfk/dropbox_rs)

# dropbox_rs
Rust bindings to the Dropbox API 

- Dropbox HTTP API Documentation [Link](https://www.dropbox.com/developers/documentation/http/documentation)

## APIs Implementation Status
- [x] auth
- [ ] file properties
- [ ] file requests
- [ ] files
- [x] paper (partially)
- [ ] sharing
- [ ] users


## Further improvements
- [ ] Better error handling. Deserialize errors into known types
- [ ] Add logging and logging configuration
    - I like structured logging and slog looks really good https://github.com/slog-rs/slog
- [ ] Better documentation. Document the various functions and point them to the relevant place in dropbox's documentation 
- [ ] Unstable async operations
- [ ] Content Hash to compare remote files with local files without downloading [Link](https://www.dropbox.com/developers/reference/content-hash)
- [ ] Webhook support?
