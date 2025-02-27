Feature: Status code feature

  Scenario: If we send status code receive same status code

    Given status="202"
    When I call /post_echo_status_code
    Then response="202"

    Given status="301"
    When I call /post_echo_status_code
    Then response="301"

    Given status="200"
    When I call /url_not_found
    Then response="404"