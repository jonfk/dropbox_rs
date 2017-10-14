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

## Run tests
To run tests you will need a dropbox access token. To get one:
- Create a new app [Link](https://www.dropbox.com/developers/apps)
- In the OAuth 2 section, use the "Generate access token" button to generate a new access token

Afterwards, you can export the access token with the `DROPBOX_TOKEN` environment variable.
You can also create a `.env` file and save the access token there as follows:
```bash
DROPBOX_TOKEN='*********************************'
DROPBOX_TOKEN_REVOKABLE='*********************************'
```
