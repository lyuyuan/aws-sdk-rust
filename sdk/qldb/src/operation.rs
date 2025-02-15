// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Ends a given Amazon QLDB journal stream. Before a stream can be canceled, its current
/// status must be <code>ACTIVE</code>.</p>
/// <p>You can't restart a stream after you cancel it. Canceled QLDB stream resources are
/// subject to a 7-day retention period, so they are automatically deleted after this limit
/// expires.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelJournalKinesisStream {
    _private: (),
}
impl CancelJournalKinesisStream {
    /// Creates a new builder-style object to manufacture [`CancelJournalKinesisStreamInput`](crate::input::CancelJournalKinesisStreamInput)
    pub fn builder() -> crate::input::cancel_journal_kinesis_stream_input::Builder {
        crate::input::cancel_journal_kinesis_stream_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CancelJournalKinesisStream {
    type Output = std::result::Result<
        crate::output::CancelJournalKinesisStreamOutput,
        crate::error::CancelJournalKinesisStreamError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_journal_kinesis_stream_error(response)
        } else {
            crate::operation_deser::parse_cancel_journal_kinesis_stream_response(response)
        }
    }
}

/// <p>Creates a new ledger in your AWS account in the current Region.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateLedger {
    _private: (),
}
impl CreateLedger {
    /// Creates a new builder-style object to manufacture [`CreateLedgerInput`](crate::input::CreateLedgerInput)
    pub fn builder() -> crate::input::create_ledger_input::Builder {
        crate::input::create_ledger_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateLedger {
    type Output =
        std::result::Result<crate::output::CreateLedgerOutput, crate::error::CreateLedgerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_ledger_error(response)
        } else {
            crate::operation_deser::parse_create_ledger_response(response)
        }
    }
}

/// <p>Deletes a ledger and all of its contents. This action is irreversible.</p>
/// <p>If deletion protection is enabled, you must first disable it before you can delete the
/// ledger. You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLedger {
    _private: (),
}
impl DeleteLedger {
    /// Creates a new builder-style object to manufacture [`DeleteLedgerInput`](crate::input::DeleteLedgerInput)
    pub fn builder() -> crate::input::delete_ledger_input::Builder {
        crate::input::delete_ledger_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteLedger {
    type Output =
        std::result::Result<crate::output::DeleteLedgerOutput, crate::error::DeleteLedgerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_ledger_error(response)
        } else {
            crate::operation_deser::parse_delete_ledger_response(response)
        }
    }
}

/// <p>Returns detailed information about a given Amazon QLDB journal stream. The output
/// includes the Amazon Resource Name (ARN), stream name, current status, creation time, and
/// the parameters of the original stream creation request.</p>
/// <p>This action does not return any expired journal streams. For more information, see
/// <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/streams.create.html#streams.create.states.expiration">Expiration for terminal streams</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeJournalKinesisStream {
    _private: (),
}
impl DescribeJournalKinesisStream {
    /// Creates a new builder-style object to manufacture [`DescribeJournalKinesisStreamInput`](crate::input::DescribeJournalKinesisStreamInput)
    pub fn builder() -> crate::input::describe_journal_kinesis_stream_input::Builder {
        crate::input::describe_journal_kinesis_stream_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeJournalKinesisStream {
    type Output = std::result::Result<
        crate::output::DescribeJournalKinesisStreamOutput,
        crate::error::DescribeJournalKinesisStreamError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_journal_kinesis_stream_error(response)
        } else {
            crate::operation_deser::parse_describe_journal_kinesis_stream_response(response)
        }
    }
}

/// <p>Returns information about a journal export job, including the ledger name, export ID,
/// creation time, current status, and the parameters of the original export creation
/// request.</p>
/// <p>This action does not return any expired export jobs. For more information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/export-journal.request.html#export-journal.request.expiration">Export job expiration</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
/// <p>If the export job with the given <code>ExportId</code> doesn't exist, then throws
/// <code>ResourceNotFoundException</code>.</p>
/// <p>If the ledger with the given <code>Name</code> doesn't exist, then throws
/// <code>ResourceNotFoundException</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeJournalS3Export {
    _private: (),
}
impl DescribeJournalS3Export {
    /// Creates a new builder-style object to manufacture [`DescribeJournalS3ExportInput`](crate::input::DescribeJournalS3ExportInput)
    pub fn builder() -> crate::input::describe_journal_s3_export_input::Builder {
        crate::input::describe_journal_s3_export_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeJournalS3Export {
    type Output = std::result::Result<
        crate::output::DescribeJournalS3ExportOutput,
        crate::error::DescribeJournalS3ExportError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_journal_s3_export_error(response)
        } else {
            crate::operation_deser::parse_describe_journal_s3_export_response(response)
        }
    }
}

/// <p>Returns information about a ledger, including its state and when it was created.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeLedger {
    _private: (),
}
impl DescribeLedger {
    /// Creates a new builder-style object to manufacture [`DescribeLedgerInput`](crate::input::DescribeLedgerInput)
    pub fn builder() -> crate::input::describe_ledger_input::Builder {
        crate::input::describe_ledger_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeLedger {
    type Output =
        std::result::Result<crate::output::DescribeLedgerOutput, crate::error::DescribeLedgerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_ledger_error(response)
        } else {
            crate::operation_deser::parse_describe_ledger_response(response)
        }
    }
}

/// <p>Exports journal contents within a date and time range from a ledger into a specified
/// Amazon Simple Storage Service (Amazon S3) bucket. The data is written as files in Amazon Ion format.</p>
/// <p>If the ledger with the given <code>Name</code> doesn't exist, then throws
/// <code>ResourceNotFoundException</code>.</p>
/// <p>If the ledger with the given <code>Name</code> is in <code>CREATING</code> status, then
/// throws <code>ResourcePreconditionNotMetException</code>.</p>
/// <p>You can initiate up to two concurrent journal export requests for each ledger. Beyond
/// this limit, journal export requests throw <code>LimitExceededException</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExportJournalToS3 {
    _private: (),
}
impl ExportJournalToS3 {
    /// Creates a new builder-style object to manufacture [`ExportJournalToS3Input`](crate::input::ExportJournalToS3Input)
    pub fn builder() -> crate::input::export_journal_to_s3_input::Builder {
        crate::input::export_journal_to_s3_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ExportJournalToS3 {
    type Output = std::result::Result<
        crate::output::ExportJournalToS3Output,
        crate::error::ExportJournalToS3Error,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_journal_to_s3_error(response)
        } else {
            crate::operation_deser::parse_export_journal_to_s3_response(response)
        }
    }
}

/// <p>Returns a block object at a specified address in a journal. Also returns a proof of the
/// specified block for verification if <code>DigestTipAddress</code> is provided.</p>
/// <p>For information about the data contents in a block, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/journal-contents.html">Journal contents</a> in the
/// <i>Amazon QLDB Developer Guide</i>.</p>
/// <p>If the specified ledger doesn't exist or is in <code>DELETING</code> status, then throws
/// <code>ResourceNotFoundException</code>.</p>
/// <p>If the specified ledger is in <code>CREATING</code> status, then throws
/// <code>ResourcePreconditionNotMetException</code>.</p>
/// <p>If no block exists with the specified address, then throws
/// <code>InvalidParameterException</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetBlock {
    _private: (),
}
impl GetBlock {
    /// Creates a new builder-style object to manufacture [`GetBlockInput`](crate::input::GetBlockInput)
    pub fn builder() -> crate::input::get_block_input::Builder {
        crate::input::get_block_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetBlock {
    type Output = std::result::Result<crate::output::GetBlockOutput, crate::error::GetBlockError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_block_error(response)
        } else {
            crate::operation_deser::parse_get_block_response(response)
        }
    }
}

/// <p>Returns the digest of a ledger at the latest committed block in the journal. The
/// response includes a 256-bit hash value and a block address.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDigest {
    _private: (),
}
impl GetDigest {
    /// Creates a new builder-style object to manufacture [`GetDigestInput`](crate::input::GetDigestInput)
    pub fn builder() -> crate::input::get_digest_input::Builder {
        crate::input::get_digest_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetDigest {
    type Output = std::result::Result<crate::output::GetDigestOutput, crate::error::GetDigestError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_digest_error(response)
        } else {
            crate::operation_deser::parse_get_digest_response(response)
        }
    }
}

/// <p>Returns a revision data object for a specified document ID and block address. Also
/// returns a proof of the specified revision for verification if <code>DigestTipAddress</code>
/// is provided.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRevision {
    _private: (),
}
impl GetRevision {
    /// Creates a new builder-style object to manufacture [`GetRevisionInput`](crate::input::GetRevisionInput)
    pub fn builder() -> crate::input::get_revision_input::Builder {
        crate::input::get_revision_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetRevision {
    type Output =
        std::result::Result<crate::output::GetRevisionOutput, crate::error::GetRevisionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_revision_error(response)
        } else {
            crate::operation_deser::parse_get_revision_response(response)
        }
    }
}

/// <p>Returns an array of all Amazon QLDB journal stream descriptors for a given ledger. The
/// output of each stream descriptor includes the same details that are returned by
/// <code>DescribeJournalKinesisStream</code>.</p>
/// <p>This action does not return any expired journal streams. For more information, see
/// <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/streams.create.html#streams.create.states.expiration">Expiration for terminal streams</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
/// <p>This action returns a maximum of <code>MaxResults</code> items. It is paginated so that
/// you can retrieve all the items by calling <code>ListJournalKinesisStreamsForLedger</code>
/// multiple times.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJournalKinesisStreamsForLedger {
    _private: (),
}
impl ListJournalKinesisStreamsForLedger {
    /// Creates a new builder-style object to manufacture [`ListJournalKinesisStreamsForLedgerInput`](crate::input::ListJournalKinesisStreamsForLedgerInput)
    pub fn builder() -> crate::input::list_journal_kinesis_streams_for_ledger_input::Builder {
        crate::input::list_journal_kinesis_streams_for_ledger_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListJournalKinesisStreamsForLedger {
    type Output = std::result::Result<
        crate::output::ListJournalKinesisStreamsForLedgerOutput,
        crate::error::ListJournalKinesisStreamsForLedgerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_journal_kinesis_streams_for_ledger_error(response)
        } else {
            crate::operation_deser::parse_list_journal_kinesis_streams_for_ledger_response(response)
        }
    }
}

/// <p>Returns an array of journal export job descriptions for all ledgers that are associated
/// with the current AWS account and Region.</p>
/// <p>This action returns a maximum of <code>MaxResults</code> items, and is paginated so that
/// you can retrieve all the items by calling <code>ListJournalS3Exports</code> multiple
/// times.</p>
/// <p>This action does not return any expired export jobs. For more information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/export-journal.request.html#export-journal.request.expiration">Export job expiration</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJournalS3Exports {
    _private: (),
}
impl ListJournalS3Exports {
    /// Creates a new builder-style object to manufacture [`ListJournalS3ExportsInput`](crate::input::ListJournalS3ExportsInput)
    pub fn builder() -> crate::input::list_journal_s3_exports_input::Builder {
        crate::input::list_journal_s3_exports_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListJournalS3Exports {
    type Output = std::result::Result<
        crate::output::ListJournalS3ExportsOutput,
        crate::error::ListJournalS3ExportsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_journal_s3_exports_error(response)
        } else {
            crate::operation_deser::parse_list_journal_s3_exports_response(response)
        }
    }
}

/// <p>Returns an array of journal export job descriptions for a specified ledger.</p>
/// <p>This action returns a maximum of <code>MaxResults</code> items, and is paginated so that
/// you can retrieve all the items by calling <code>ListJournalS3ExportsForLedger</code>
/// multiple times.</p>
/// <p>This action does not return any expired export jobs. For more information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/export-journal.request.html#export-journal.request.expiration">Export job expiration</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJournalS3ExportsForLedger {
    _private: (),
}
impl ListJournalS3ExportsForLedger {
    /// Creates a new builder-style object to manufacture [`ListJournalS3ExportsForLedgerInput`](crate::input::ListJournalS3ExportsForLedgerInput)
    pub fn builder() -> crate::input::list_journal_s3_exports_for_ledger_input::Builder {
        crate::input::list_journal_s3_exports_for_ledger_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListJournalS3ExportsForLedger {
    type Output = std::result::Result<
        crate::output::ListJournalS3ExportsForLedgerOutput,
        crate::error::ListJournalS3ExportsForLedgerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_journal_s3_exports_for_ledger_error(response)
        } else {
            crate::operation_deser::parse_list_journal_s3_exports_for_ledger_response(response)
        }
    }
}

/// <p>Returns an array of ledger summaries that are associated with the current AWS account
/// and Region.</p>
/// <p>This action returns a maximum of 100 items and is paginated so that you can
/// retrieve all the items by calling <code>ListLedgers</code> multiple times.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLedgers {
    _private: (),
}
impl ListLedgers {
    /// Creates a new builder-style object to manufacture [`ListLedgersInput`](crate::input::ListLedgersInput)
    pub fn builder() -> crate::input::list_ledgers_input::Builder {
        crate::input::list_ledgers_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListLedgers {
    type Output =
        std::result::Result<crate::output::ListLedgersOutput, crate::error::ListLedgersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_ledgers_error(response)
        } else {
            crate::operation_deser::parse_list_ledgers_response(response)
        }
    }
}

/// <p>Returns all tags for a specified Amazon QLDB resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Creates a journal stream for a given Amazon QLDB ledger. The stream captures every
/// document revision that is committed to the ledger's journal and delivers the data to a
/// specified Amazon Kinesis Data Streams resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StreamJournalToKinesis {
    _private: (),
}
impl StreamJournalToKinesis {
    /// Creates a new builder-style object to manufacture [`StreamJournalToKinesisInput`](crate::input::StreamJournalToKinesisInput)
    pub fn builder() -> crate::input::stream_journal_to_kinesis_input::Builder {
        crate::input::stream_journal_to_kinesis_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StreamJournalToKinesis {
    type Output = std::result::Result<
        crate::output::StreamJournalToKinesisOutput,
        crate::error::StreamJournalToKinesisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stream_journal_to_kinesis_error(response)
        } else {
            crate::operation_deser::parse_stream_journal_to_kinesis_response(response)
        }
    }
}

/// <p>Adds one or more tags to a specified Amazon QLDB resource.</p>
/// <p>A resource can have up to 50 tags. If you try to create more than 50 tags for a
/// resource, your request fails and returns an error.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes one or more tags from a specified Amazon QLDB resource. You can specify up to 50
/// tag keys to remove.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <p>Updates properties on a ledger.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateLedger {
    _private: (),
}
impl UpdateLedger {
    /// Creates a new builder-style object to manufacture [`UpdateLedgerInput`](crate::input::UpdateLedgerInput)
    pub fn builder() -> crate::input::update_ledger_input::Builder {
        crate::input::update_ledger_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateLedger {
    type Output =
        std::result::Result<crate::output::UpdateLedgerOutput, crate::error::UpdateLedgerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_ledger_error(response)
        } else {
            crate::operation_deser::parse_update_ledger_response(response)
        }
    }
}

/// <p>Updates the permissions mode of a ledger.</p>
/// <important>
/// <p>Before you switch to the <code>STANDARD</code> permissions mode, you must first
/// create all required IAM policies and table tags to avoid disruption to your users. To
/// learn more, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/ledger-management.basics.html#ledger-mgmt.basics.update-permissions.migrating">Migrating to the standard permissions mode</a> in the <i>Amazon QLDB
/// Developer Guide</i>.</p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateLedgerPermissionsMode {
    _private: (),
}
impl UpdateLedgerPermissionsMode {
    /// Creates a new builder-style object to manufacture [`UpdateLedgerPermissionsModeInput`](crate::input::UpdateLedgerPermissionsModeInput)
    pub fn builder() -> crate::input::update_ledger_permissions_mode_input::Builder {
        crate::input::update_ledger_permissions_mode_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateLedgerPermissionsMode {
    type Output = std::result::Result<
        crate::output::UpdateLedgerPermissionsModeOutput,
        crate::error::UpdateLedgerPermissionsModeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_ledger_permissions_mode_error(response)
        } else {
            crate::operation_deser::parse_update_ledger_permissions_mode_response(response)
        }
    }
}
