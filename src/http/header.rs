
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use reqwest::header::Header;
use reqwest::header::Formatter;
use reqwest::header::Raw;
use hyper::Error as HyperError;

#[derive(Debug, Clone, Copy)]
pub struct DropboxAPIArg<D: DeserializeOwned + Serialize + Sync + Clone + Send>(pub D);

impl<D: DeserializeOwned + Serialize + Sync + Clone + Send + 'static> Header for DropboxAPIArg<D> {
    fn header_name() -> &'static str {
        "Dropbox-API-Arg"
    }
    fn parse_header(raw: &Raw) -> Result<Self, HyperError> {
        let header_content: Vec<u8> = raw.into_iter().flat_map(|l| l.to_vec()).collect::<_>();

        Ok(DropboxAPIArg(serde_json::from_slice(&header_content).map_err(|_| HyperError::Header)?))
    }
    fn fmt_header(&self, f: &mut Formatter) -> ::std::fmt::Result {
        let header = serde_json::to_string(&self.0).map_err(|_| ::std::fmt::Error)?;
        write!(f, "{}", header)
    }
}
