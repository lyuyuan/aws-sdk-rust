// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Runs a batch SQL statement over an array of data.</p>
/// <p>You can run bulk update and insert operations for multiple records using a DML
/// statement with different parameter sets. Bulk operations can provide a significant
/// performance improvement over individual insert and update operations.</p>
/// <important>    
/// <p>If a call isn't part of a transaction because it doesn't include the
/// <code>transactionID</code> parameter, changes that result from the call are
/// committed automatically.</p>    
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchExecuteStatement {
    _private: (),
}
impl BatchExecuteStatement {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementInput`](crate::input::BatchExecuteStatementInput)
    pub fn builder() -> crate::input::batch_execute_statement_input::Builder {
        crate::input::batch_execute_statement_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for BatchExecuteStatement {
    type Output = std::result::Result<
        crate::output::BatchExecuteStatementOutput,
        crate::error::BatchExecuteStatementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_execute_statement_error(response)
        } else {
            crate::operation_deser::parse_batch_execute_statement_response(response)
        }
    }
}

/// <p>Starts a SQL transaction.</p>
/// <important>
/// <p>A transaction can run for a maximum of 24 hours. A transaction is terminated and
/// rolled back automatically after 24 hours.</p>
/// <p>A transaction times out if no calls use its transaction ID in three minutes.
/// If a transaction times out before it's committed, it's rolled back
/// automatically.</p>
/// <p>DDL statements inside a transaction cause an implicit commit. We recommend
/// that you run each DDL statement in a separate <code>ExecuteStatement</code> call with
/// <code>continueAfterTimeout</code> enabled.</p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BeginTransaction {
    _private: (),
}
impl BeginTransaction {
    /// Creates a new builder-style object to manufacture [`BeginTransactionInput`](crate::input::BeginTransactionInput)
    pub fn builder() -> crate::input::begin_transaction_input::Builder {
        crate::input::begin_transaction_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for BeginTransaction {
    type Output = std::result::Result<
        crate::output::BeginTransactionOutput,
        crate::error::BeginTransactionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_begin_transaction_error(response)
        } else {
            crate::operation_deser::parse_begin_transaction_response(response)
        }
    }
}

/// <p>Ends a SQL transaction started with the <code>BeginTransaction</code> operation and
/// commits the changes.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CommitTransaction {
    _private: (),
}
impl CommitTransaction {
    /// Creates a new builder-style object to manufacture [`CommitTransactionInput`](crate::input::CommitTransactionInput)
    pub fn builder() -> crate::input::commit_transaction_input::Builder {
        crate::input::commit_transaction_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CommitTransaction {
    type Output = std::result::Result<
        crate::output::CommitTransactionOutput,
        crate::error::CommitTransactionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_commit_transaction_error(response)
        } else {
            crate::operation_deser::parse_commit_transaction_response(response)
        }
    }
}

/// <p>Runs one or more SQL statements.</p>
/// <important>
/// <p>This operation is deprecated. Use the <code>BatchExecuteStatement</code> or
/// <code>ExecuteStatement</code> operation.</p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExecuteSql {
    _private: (),
}
impl ExecuteSql {
    /// Creates a new builder-style object to manufacture [`ExecuteSqlInput`](crate::input::ExecuteSqlInput)
    pub fn builder() -> crate::input::execute_sql_input::Builder {
        crate::input::execute_sql_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ExecuteSql {
    type Output =
        std::result::Result<crate::output::ExecuteSqlOutput, crate::error::ExecuteSqlError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_execute_sql_error(response)
        } else {
            crate::operation_deser::parse_execute_sql_response(response)
        }
    }
}

/// <p>Runs a SQL statement against a database.</p>
/// <important>    
/// <p>If a call isn't part of a transaction because it doesn't include the
/// <code>transactionID</code> parameter, changes that result from the call are
/// committed automatically.</p>    
/// </important>
/// <p>The response size limit is 1 MB. If the call returns more than 1 MB of response data, the call is terminated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExecuteStatement {
    _private: (),
}
impl ExecuteStatement {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementInput`](crate::input::ExecuteStatementInput)
    pub fn builder() -> crate::input::execute_statement_input::Builder {
        crate::input::execute_statement_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ExecuteStatement {
    type Output = std::result::Result<
        crate::output::ExecuteStatementOutput,
        crate::error::ExecuteStatementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_execute_statement_error(response)
        } else {
            crate::operation_deser::parse_execute_statement_response(response)
        }
    }
}

/// <p>Performs a rollback of a transaction. Rolling back a transaction cancels its changes.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RollbackTransaction {
    _private: (),
}
impl RollbackTransaction {
    /// Creates a new builder-style object to manufacture [`RollbackTransactionInput`](crate::input::RollbackTransactionInput)
    pub fn builder() -> crate::input::rollback_transaction_input::Builder {
        crate::input::rollback_transaction_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for RollbackTransaction {
    type Output = std::result::Result<
        crate::output::RollbackTransactionOutput,
        crate::error::RollbackTransactionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_rollback_transaction_error(response)
        } else {
            crate::operation_deser::parse_rollback_transaction_response(response)
        }
    }
}