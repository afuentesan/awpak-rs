Feature: Query param feature

  Scenario: If we send param receive same param

    Given query="a=1&b=2"
    When I call /get_echo_params_a_b
    Then response="a=1&b=2"

    Given query='point={"x":33,"y":27}'
    When I call /get_echo_param_point
    Then response="x=33, y=27"

    Given query='point={"y":27}'
    When I call /get_echo_param_point
    Then response="x=0, y=27"

    Given query='a=hello'
    When I call /get_echo_param_string
    Then response="a=hello"

    Given query='a_renamed=hello'
    When I call /get_echo_param_string_change_name
    Then response="a=hello"

    Given query=''
    When I call /get_echo_param_option_string
    Then response="a="

    Given query='a=goodbye'
    When I call /get_echo_param_option_string
    Then response="a=goodbye"

    Given query=''
    When I call /get_echo_param_option_number
    Then response="a="

    Given query='a=3'
    When I call /get_echo_param_option_number
    Then response="a=3"

    Given query='point=1,2'
    When I call /get_echo_param_point_custom_deserializer
    Then response="x=201, y=2"

    Given query='point=1'
    When I call /get_echo_param_point_custom_deserializer
    Then response="x=0, y=201"

    Given query='point_renamed=1'
    When I call /get_echo_param_point_custom_deserializer_change_name
    Then response="x=0, y=201"