
use std::fmt;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use reqwest::header::Header;
use reqwest::header::Formatter;
use reqwest::header::Raw;
use hyper::Error as HyperError;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct DropboxAPIArg<D>(pub D);

impl<D: DeserializeOwned + Serialize + Sync + Clone + Send + 'static> Header for DropboxAPIArg<D> {
    fn header_name() -> &'static str {
        "Dropbox-API-Arg"
    }
    fn parse_header(raw: &Raw) -> Result<Self, HyperError> {
        let header_content: Vec<u8> = raw.into_iter().flat_map(|l| l.to_vec()).collect::<_>();

        Ok(DropboxAPIArg(serde_json::from_slice(&header_content).map_err(|_| HyperError::Header)?))
    }
    fn fmt_header(&self, f: &mut Formatter) -> ::std::fmt::Result {
        f.fmt_line(self)
    }
}

impl<D: Serialize + Sync + Clone + Send> fmt::Display for DropboxAPIArg<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = serde_json::to_string(&self.0).map_err(|_| ::std::fmt::Error)?;
        f.write_str(&header)
    }
}
