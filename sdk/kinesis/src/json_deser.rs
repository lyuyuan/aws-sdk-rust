// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn describe_limits_deser_operation(
    input: &[u8],
    mut builder: crate::output::describe_limits_output::Builder,
) -> std::result::Result<crate::output::describe_limits_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeLimitsOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_shard_limit(parsed_body.shard_limit);
    builder = builder.set_open_shard_count(parsed_body.open_shard_count);
    Ok(builder)
}

pub fn describe_stream_deser_operation(
    input: &[u8],
    mut builder: crate::output::describe_stream_output::Builder,
) -> std::result::Result<crate::output::describe_stream_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeStreamOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_stream_description(parsed_body.stream_description);
    Ok(builder)
}

pub fn describe_stream_consumer_deser_operation(
    input: &[u8],
    mut builder: crate::output::describe_stream_consumer_output::Builder,
) -> std::result::Result<crate::output::describe_stream_consumer_output::Builder, serde_json::Error>
{
    let parsed_body: crate::serializer::DescribeStreamConsumerOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_consumer_description(parsed_body.consumer_description);
    Ok(builder)
}

pub fn describe_stream_summary_deser_operation(
    input: &[u8],
    mut builder: crate::output::describe_stream_summary_output::Builder,
) -> std::result::Result<crate::output::describe_stream_summary_output::Builder, serde_json::Error>
{
    let parsed_body: crate::serializer::DescribeStreamSummaryOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_stream_description_summary(parsed_body.stream_description_summary);
    Ok(builder)
}

pub fn disable_enhanced_monitoring_deser_operation(
    input: &[u8],
    mut builder: crate::output::disable_enhanced_monitoring_output::Builder,
) -> std::result::Result<
    crate::output::disable_enhanced_monitoring_output::Builder,
    serde_json::Error,
> {
    let parsed_body: crate::serializer::DisableEnhancedMonitoringOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_stream_name(parsed_body.stream_name);
    builder = builder.set_current_shard_level_metrics(parsed_body.current_shard_level_metrics);
    builder = builder.set_desired_shard_level_metrics(parsed_body.desired_shard_level_metrics);
    Ok(builder)
}

pub fn enable_enhanced_monitoring_deser_operation(
    input: &[u8],
    mut builder: crate::output::enable_enhanced_monitoring_output::Builder,
) -> std::result::Result<crate::output::enable_enhanced_monitoring_output::Builder, serde_json::Error>
{
    let parsed_body: crate::serializer::EnableEnhancedMonitoringOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_stream_name(parsed_body.stream_name);
    builder = builder.set_current_shard_level_metrics(parsed_body.current_shard_level_metrics);
    builder = builder.set_desired_shard_level_metrics(parsed_body.desired_shard_level_metrics);
    Ok(builder)
}

pub fn get_records_deser_operation(
    input: &[u8],
    mut builder: crate::output::get_records_output::Builder,
) -> std::result::Result<crate::output::get_records_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetRecordsOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_records(parsed_body.records);
    builder = builder.set_next_shard_iterator(parsed_body.next_shard_iterator);
    builder = builder.set_millis_behind_latest(parsed_body.millis_behind_latest);
    builder = builder.set_child_shards(parsed_body.child_shards);
    Ok(builder)
}

pub fn get_shard_iterator_deser_operation(
    input: &[u8],
    mut builder: crate::output::get_shard_iterator_output::Builder,
) -> std::result::Result<crate::output::get_shard_iterator_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetShardIteratorOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_shard_iterator(parsed_body.shard_iterator);
    Ok(builder)
}

pub fn list_shards_deser_operation(
    input: &[u8],
    mut builder: crate::output::list_shards_output::Builder,
) -> std::result::Result<crate::output::list_shards_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListShardsOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_shards(parsed_body.shards);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_stream_consumers_deser_operation(
    input: &[u8],
    mut builder: crate::output::list_stream_consumers_output::Builder,
) -> std::result::Result<crate::output::list_stream_consumers_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListStreamConsumersOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_consumers(parsed_body.consumers);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_streams_deser_operation(
    input: &[u8],
    mut builder: crate::output::list_streams_output::Builder,
) -> std::result::Result<crate::output::list_streams_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListStreamsOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_stream_names(parsed_body.stream_names);
    builder = builder.set_has_more_streams(parsed_body.has_more_streams);
    Ok(builder)
}

pub fn list_tags_for_stream_deser_operation(
    input: &[u8],
    mut builder: crate::output::list_tags_for_stream_output::Builder,
) -> std::result::Result<crate::output::list_tags_for_stream_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListTagsForStreamOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_tags(parsed_body.tags);
    builder = builder.set_has_more_tags(parsed_body.has_more_tags);
    Ok(builder)
}

pub fn put_record_deser_operation(
    input: &[u8],
    mut builder: crate::output::put_record_output::Builder,
) -> std::result::Result<crate::output::put_record_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::PutRecordOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_shard_id(parsed_body.shard_id);
    builder = builder.set_sequence_number(parsed_body.sequence_number);
    builder = builder.set_encryption_type(parsed_body.encryption_type);
    Ok(builder)
}

pub fn put_records_deser_operation(
    input: &[u8],
    mut builder: crate::output::put_records_output::Builder,
) -> std::result::Result<crate::output::put_records_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::PutRecordsOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_failed_record_count(parsed_body.failed_record_count);
    builder = builder.set_records(parsed_body.records);
    builder = builder.set_encryption_type(parsed_body.encryption_type);
    Ok(builder)
}

pub fn register_stream_consumer_deser_operation(
    input: &[u8],
    mut builder: crate::output::register_stream_consumer_output::Builder,
) -> std::result::Result<crate::output::register_stream_consumer_output::Builder, serde_json::Error>
{
    let parsed_body: crate::serializer::RegisterStreamConsumerOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_consumer(parsed_body.consumer);
    Ok(builder)
}

pub fn update_shard_count_deser_operation(
    input: &[u8],
    mut builder: crate::output::update_shard_count_output::Builder,
) -> std::result::Result<crate::output::update_shard_count_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateShardCountOutputBody = if input.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(input)?
    };
    builder = builder.set_stream_name(parsed_body.stream_name);
    builder = builder.set_current_shard_count(parsed_body.current_shard_count);
    builder = builder.set_target_shard_count(parsed_body.target_shard_count);
    Ok(builder)
}
