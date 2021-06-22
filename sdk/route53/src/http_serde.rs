// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_create_health_check_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_hosted_zone_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_key_signing_key_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_query_logging_config_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_reusable_delegation_set_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_traffic_policy_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_traffic_policy_instance_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}

pub fn deser_header_create_traffic_policy_version_location(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<std::string::String>, smithy_http::header::ParseError>
{
    let headers = header_map.get_all("Location").iter();
    smithy_http::header::one_or_none(headers)
}