use std::fmt::Display;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Either<A,B> {
    A(A),
    B(B)
}
impl<A,B> Display for Either<A, B> where A:Display, B:Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(a) => a.fmt(f),
            Self::B(b) => b.fmt(f),
        }
    }
}

macro_rules! impl_from_either {
    ($structname: ident, $a: ident, $b: ident) => {
          impl From<Either<$a, $b>> for $structname {
              fn from(value: Either<$a, $b>) -> $structname {
                match value {
                    Either::A(a) => a.into(),
                    Either::B(b) => b.into()
                }
              }
          }
    };
}
pub(crate) use impl_from_either;

/// impl_try_from_either!(t, a, b, error_a, error_b) where T: TryFrom\<A> and T: TryFrom\<B>
/// 
/// implements TryFrom\<Either\<a, b>> for t
/// 
/// The error type is Either\<error_a, error_b>. If one is infallible the error type is the non infallible error instead.
macro_rules! impl_try_from_either {
    ($t: ident, $a: ident, $b: ident, $error: ident) => {
        impl TryFrom<Either<$a, $b>> for $t {
              type Error = $error;
              fn try_from(value: Either<$a, $b>) -> Result<$t, Self::Error> {
                  match value {
                      Either::A(a) => a.try_into(),
                      Either::B(b) => b.try_into()
                  }
              }
        }
    };
    ($t: ident, $a: ident, $b: ident, $error: ident, Infallible) => {
          impl TryFrom<Either<$a, $b>> for $t {
                type Error = $error;
                fn try_from(value: Either<$a, $b>) -> Result<$t, Self::Error> {
                    match value {
                        Either::A(a) => a.try_into(),
                        Either::B(b) => Ok(b.into())
                    }
                }
          }
    };
    ($t: ident, $a: ident, $b: ident, Infallible, $error: ident) => {
          impl TryFrom<Either<$a, $b>> for $t {
                type Error = $error;
                fn try_from(value: Either<$a, $b>) -> Result<$t, Self::Error> {
                    match value {
                        Either::A(a) => Ok(a.into()),
                        Either::B(b) => b.try_into()
                    }
                }
          }
    };
    ($t: ident, $a: ident, $b: ident, $error_a: ident, $error_b: ident) => {
        impl TryFrom<Either<$a, $b>> for $t {
              type Error = Either<$error_a, $error_b>;
              fn try_from(value: Either<$a, $b>) -> Result<$t, Self::Error> {
                  match value {
                      Either::A(a) => a.try_into().map_err(Either::A),
                      Either::B(b) => b.try_into().map_err(Either::B)
                  }
              }
        }
  };
}
pub(crate) use impl_try_from_either;

pub struct NotImplemented(pub &'static str);
impl Display for NotImplemented {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn deserialize_via<'de, D, T, Via>(deserializer: D) -> Result<T, D::Error> where D: Deserializer<'de>, Via: Into<T> + Deserialize<'de> {
    let value = Via::deserialize(deserializer)?;
    Ok(value.into())
}
