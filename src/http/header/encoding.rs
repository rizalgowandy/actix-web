use std::{fmt, str};

pub use self::Encoding::{
    Brotli, Chunked, Compress, Deflate, EncodingExt, Gzip, Identity, Trailers, Zstd,
};

/// A value to represent an encoding used in `Transfer-Encoding` or `Accept-Encoding` header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Encoding {
    /// The `chunked` encoding.
    Chunked,

    /// The `br` encoding.
    Brotli,

    /// The `gzip` encoding.
    Gzip,

    /// The `deflate` encoding.
    Deflate,

    /// The `compress` encoding.
    Compress,

    /// The `identity` encoding.
    Identity,

    /// The `trailers` encoding.
    Trailers,

    /// The `zstd` encoding.
    Zstd,

    /// Some other encoding that is less common, can be any String.
    EncodingExt(String),
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Chunked => "chunked",
            Brotli => "br",
            Gzip => "gzip",
            Deflate => "deflate",
            Compress => "compress",
            Identity => "identity",
            Trailers => "trailers",
            Zstd => "zstd",
            EncodingExt(ref s) => s.as_ref(),
        })
    }
}

impl str::FromStr for Encoding {
    type Err = crate::error::ParseError;
    fn from_str(s: &str) -> Result<Encoding, crate::error::ParseError> {
        match s {
            "chunked" => Ok(Chunked),
            "br" => Ok(Brotli),
            "deflate" => Ok(Deflate),
            "gzip" => Ok(Gzip),
            "compress" => Ok(Compress),
            "identity" => Ok(Identity),
            "trailers" => Ok(Trailers),
            "zstd" => Ok(Zstd),
            _ => Ok(EncodingExt(s.to_owned())),
        }
    }
}
