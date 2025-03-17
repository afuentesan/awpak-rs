Feature: Get 2 feature

  Scenario: If we send Some or None receive same Some or None
    Given query=""
    When I call /get_echo_option_point
    Then response=""

    Given query="x=47&y=22"
    When I call /get_echo_option_point
    Then response='{"x":47.0,"y":22.0}'

    Given query="x=47&y=22"
    When I call /get_replace_query_point_data
    Then response='{"x":48.0,"y":23.0}'

    Given query="y=22"
    When I call /get_replace_query_point_data
    Then response='{"x":null,"y":23.0}'

    Given query="x=1&y=1"
    When I call /get_point_with_context
    Then response='{"point":null,"x":1.0,"y":201.0}'

    Given query='x=1&y=1&point={"x":2,"y":2}'
    When I call /get_point_with_context
    Then response='{"point":{"x":2.0,"y":2.0},"x":1.0,"y":201.0}'

    Given query='x=1&y=1&point={"x":2,"y":2}'
    When I call /get_point_with_context_wrapper
    Then response='{"point":{"point":null,"x":2.0,"y":202.0},"x":201.0,"y":1.0}'

    Given query='x=1&y=1&point={"x":2,"y":2,"point":{"x":3,"y":3}}'
    When I call /get_point_with_context_wrapper
    Then response='{"point":{"point":{"x":3.0,"y":3.0},"x":2.0,"y":202.0},"x":201.0,"y":1.0}'

    Given query='3'
    When I call /get_query_params_from_async_str
    Then response='{"status":203}'