// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Sends a command to an Amazon QLDB ledger.</p>
/// <note>
/// <p>Instead of interacting directly with this API, we recommend using the QLDB driver
/// or the QLDB shell to execute data transactions on a ledger.</p>
/// <ul>
/// <li>
/// <p>If you are working with an AWS SDK, use the QLDB driver. The driver provides
/// a high-level abstraction layer above this <i>QLDB Session</i> data
/// plane and manages <code>SendCommand</code> API calls for you. For information and
/// a list of supported programming languages, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/getting-started-driver.html">Getting started
/// with the driver</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
/// </li>
/// <li>
/// <p>If you are working with the AWS Command Line Interface (AWS CLI), use the
/// QLDB shell. The shell is a command line interface that uses the QLDB driver to
/// interact with a ledger. For information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/data-shell.html">Accessing Amazon QLDB using the
/// QLDB shell</a>.</p>
/// </li>
/// </ul>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendCommand {
    _private: (),
}
impl SendCommand {
    /// Creates a new builder-style object to manufacture [`SendCommandInput`](crate::input::SendCommandInput)
    pub fn builder() -> crate::input::send_command_input::Builder {
        crate::input::send_command_input::Builder::default()
    }
    #[allow(clippy::unnecessary_wraps)]
    #[allow(dead_code)]
    fn parse_response(
        &self,
        response: &http::response::Response<bytes::Bytes>,
    ) -> std::result::Result<crate::output::SendCommandOutput, crate::error::SendCommandError> {
        if crate::aws_json_errors::is_error(&response) {
            let body = serde_json::from_slice(response.body().as_ref())
                .unwrap_or_else(|_| serde_json::json!({}));
            let generic = crate::aws_json_errors::parse_generic_error(&response, &body);
            let error_code = match generic.code() {
                Some(code) => code,
                None => return Err(crate::error::SendCommandError::unhandled(generic)),
            };
            return Err(match error_code {
                "BadRequestException" => match serde_json::from_value(body) {
                    Ok(body) => crate::error::SendCommandError {
                        kind: crate::error::SendCommandErrorKind::BadRequestError(body),
                        meta: generic,
                    },
                    Err(e) => crate::error::SendCommandError::unhandled(e),
                },
                "CapacityExceededException" => match serde_json::from_value(body) {
                    Ok(body) => crate::error::SendCommandError {
                        kind: crate::error::SendCommandErrorKind::CapacityExceededError(body),
                        meta: generic,
                    },
                    Err(e) => crate::error::SendCommandError::unhandled(e),
                },
                "InvalidSessionException" => match serde_json::from_value(body) {
                    Ok(body) => crate::error::SendCommandError {
                        kind: crate::error::SendCommandErrorKind::InvalidSessionError(body),
                        meta: generic,
                    },
                    Err(e) => crate::error::SendCommandError::unhandled(e),
                },
                "LimitExceededException" => match serde_json::from_value(body) {
                    Ok(body) => crate::error::SendCommandError {
                        kind: crate::error::SendCommandErrorKind::LimitExceededError(body),
                        meta: generic,
                    },
                    Err(e) => crate::error::SendCommandError::unhandled(e),
                },
                "OccConflictException" => match serde_json::from_value(body) {
                    Ok(body) => crate::error::SendCommandError {
                        kind: crate::error::SendCommandErrorKind::OccConflictError(body),
                        meta: generic,
                    },
                    Err(e) => crate::error::SendCommandError::unhandled(e),
                },
                "RateExceededException" => match serde_json::from_value(body) {
                    Ok(body) => crate::error::SendCommandError {
                        kind: crate::error::SendCommandErrorKind::RateExceededError(body),
                        meta: generic,
                    },
                    Err(e) => crate::error::SendCommandError::unhandled(e),
                },
                _ => crate::error::SendCommandError::generic(generic),
            });
        }
        #[allow(unused_mut)]
        let mut builder = crate::output::send_command_output::Builder::default();
        builder =
            crate::json_deser::send_command_deser_operation(response.body().as_ref(), builder)
                .map_err(crate::error::SendCommandError::unhandled)?;
        Ok(builder.build())
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for SendCommand {
    type Output =
        std::result::Result<crate::output::SendCommandOutput, crate::error::SendCommandError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        self.parse_response(response)
    }
}
