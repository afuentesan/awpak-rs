Feature: Headers feature

  Scenario: If we send Header receive same Header

    Given names="h1", values="5"
    When I call /post_echo_request_header
    Then response="(h1, 5)", response_headers="(h1, 5)"

    Given names="h1,h2", values="5,6"
    When I call /post_echo_request_header_add_one
    Then response="(h1, 6) (h2, 7)", response_headers="(h1, 6) (h2, 7)"

    Given names="h1,h2", values="5,6"
    When I call /post_echo_request_header_add_two
    Then response="Some", response_headers="(h1, 7) (h2, 8)"