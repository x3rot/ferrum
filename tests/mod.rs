mod test_utils;

// Unit tests
mod unit {
    mod ca_tests;
    mod request_interceptor_tests;
    mod response_interceptor_tests;
    mod proxy_server_tests;
}

// Integration tests
mod integration {
    mod proxy_integration_tests;
}
