Feature: Post multipart json feature

  Scenario: If we send Point receive same Point
    
    Given x=Some(3), y=5
    When I call /post_body_echo_point
    Then x=Some(3), y=5

    Given x=Some(3.794), y=5.951
    When I call /post_body_echo_point
    Then x=Some(3.794), y=5.951