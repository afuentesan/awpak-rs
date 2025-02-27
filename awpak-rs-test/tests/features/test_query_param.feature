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