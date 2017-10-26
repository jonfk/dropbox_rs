[![Build Status](https://travis-ci.org/jonfk/dropbox_rs.svg?branch=master)](https://travis-ci.org/jonfk/dropbox_rs)

# dropbox_rs
Rust bindings to the Dropbox API 

- Dropbox HTTP API Documentation [Link](https://www.dropbox.com/developers/documentation/http/documentation)

## APIs Implementation Status
- [x] [auth](https://www.dropbox.com/developers/documentation/http/documentation#auth)
- [ ] [file properties](https://www.dropbox.com/developers/documentation/http/documentation#file_properties)
- [ ] [file requests](https://www.dropbox.com/developers/documentation/http/documentation#file_requests)
- [ ] [files](https://www.dropbox.com/developers/documentation/http/documentation#files)
- [x] [paper](https://www.dropbox.com/developers/documentation/http/documentation#paper)
- [ ] [sharing](https://www.dropbox.com/developers/documentation/http/documentation#sharing)
- [ ] [users](https://www.dropbox.com/developers/documentation/http/documentation#users)


## Further improvements
- [x] Better error handling. Deserialize errors into known types
- [x] Add logging and logging configuration
    - I like structured logging and slog looks really good https://github.com/slog-rs/slog
    - It would be nice if we could compile out logging as a feature
- [ ] Better documentation. Document the various functions and point them to the relevant place in dropbox's documentation 
- [ ] Unstable async operations
- [ ] Content Hash to compare remote files with local files without downloading [Link](https://www.dropbox.com/developers/reference/content-hash)
- [ ] Webhook support?

## Run tests
To run tests you will need a dropbox access token. To get one:
- Create a new app [Link](https://www.dropbox.com/developers/apps)
- In the OAuth 2 section, use the "Generate access token" button to generate a new access token

Afterwards, you can export the access token with the `DROPBOX_TOKEN` environment variable. `DROPBOX_TOKEN='************' cargo test`

You can also create a `.env` file and save the access token there as follows:
```bash
DROPBOX_TOKEN='*********************************'
DROPBOX_TOKEN_REVOKABLE='*********************************'
```
