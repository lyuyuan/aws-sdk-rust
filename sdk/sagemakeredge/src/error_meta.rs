// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    InternalServiceError(crate::error::InternalServiceError),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServiceError(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetDeviceRegistrationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetDeviceRegistrationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDeviceRegistrationErrorKind::InternalServiceError(inner) => {
                    Error::InternalServiceError(inner)
                }
                crate::error::GetDeviceRegistrationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SendHeartbeatError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SendHeartbeatError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendHeartbeatErrorKind::InternalServiceError(inner) => {
                    Error::InternalServiceError(inner)
                }
                crate::error::SendHeartbeatErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}