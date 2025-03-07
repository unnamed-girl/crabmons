use std::fmt::{Debug, Display};
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

/// **impl_from_either!(T, A, B)** where A:Into\<T> and B: Into\<T>.\
/// Impements From\<Either\<A, B>> for T. 
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

/// **impl_try_from_either!(T, A, B, Error)** where T: TryFrom\<A>, T: TryFrom\<B>.\
/// Implement TryFrom\<Either\<A, B>> for T. The Error type is the given Error type.\
/// 
/// **impl_try_from_either!(T, A, B, ErrorA, ErrorB)** where T: TryFrom\<A>, T: TryFrom\<B>.\
/// Implement TryFrom\<Either\<A, B>> for T. The Error type is Either\<ErrorA, ErrorB>.\
/// If either is Infallible, though, instead the Error type is the noninfallible error.\
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
