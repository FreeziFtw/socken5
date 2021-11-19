
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Method {
    NoAuth,
    Gssapi,
    Auth,
    Other(u8),
    NoAcceptable,
}

impl From<u8> for Method {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Self::NoAuth,
            0x01 => Self::Gssapi,
            0x02 => Self::Auth,
            _ => Self::Other(val),
        }
    }
}

impl From<Method> for u8 {
    fn from(method: Method) -> Self {
        match method {
            Method::NoAuth => 0x00,
            Method::Gssapi => 0x01,
            Method::Auth => 0x02,
            Method::Other(val) => val,
            Method::NoAcceptable => 0xFF,
        }
    }
}