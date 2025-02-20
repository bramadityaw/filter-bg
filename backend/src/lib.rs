use std::io::Error as IoError;
use tokio_native_tls::native_tls::Error as TlsError;
use tokio_tungstenite::tungstenite::Error as WsError;

macro_rules! impl_err_convert {
    ($($x:ident => $y:ident,)*) => {
        $(
        impl From<$x> for Error {
            fn from(value: $x) -> Self {
                Error::$y(value)
            }
        }
        )*
    };
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    TlsErr(TlsError),
    WsErr(WsError),
    IoErr(IoError),
}

impl_err_convert!(
    TlsError => TlsErr,
    WsError => WsErr,
    IoError => IoErr,
);
