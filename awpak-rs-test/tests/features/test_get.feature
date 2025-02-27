Feature: Get feature

  Scenario: If we send Point receive same Point
    Given x=Some(3), y=5
    When I call /get_echo_point
    Then x=Some(3), y=5

    Given x=Some(7), y=9
    When I call /get_echo_point
    Then x=Some(7), y=9

    Given x=None, y=9.5
    When I call /get_echo_point
    Then x=None, y=9.5

  Scenario: Add one to y in get

    Given x=Some(3), y=5
    When I call /add_one_to_y_if_get_or_x_if_post
    Then x=Some(3), y=6
