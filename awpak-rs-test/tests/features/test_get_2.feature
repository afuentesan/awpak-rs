Feature: Get 2 feature

  Scenario: If we send Some or None receive same Some or None
    Given query=""
    When I call /get_echo_option_point
    Then response=""

    Given query="x=47&y=22"
    When I call /get_echo_option_point
    Then response='{"x":47.0,"y":22.0}'