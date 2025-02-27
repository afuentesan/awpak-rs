Feature: Cookies feature

  Scenario: If we send Cookie receive same Cookie

    Given names="h1", values="5"
    When I call /post_echo_request_cookies
    Then response="(h1=5)"