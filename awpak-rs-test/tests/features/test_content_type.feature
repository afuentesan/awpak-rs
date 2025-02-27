Feature: Content type feature

  Scenario: Content type
    Given accept="application/json;q=0.99, text/plain, text/html;q=0.9, text/plain;q=0.8", content_type="text/plain", data="text"
    When I call /post_echo_text
    Then content_type="text/plain", data="text"

    Given accept="application/json;q=0.99, text/plain, text/html;q=0.9, text/plain;q=0.8", content_type="text/plain", data='{"x":3,"y":5}'
    When I call /post_echo_text
    Then content_type="text/plain", data='{"x":3,"y":5}'

    Given accept="application/json, text/plain;q=0.99, text/html;q=0.9, text/plain;q=0.8", content_type="application/json", data='{"x":3,"y":5}'
    When I call /post_body_echo_point
    Then content_type="application/json", data='{"x":3.0,"y":5.0}'

    Given accept="application/json;q=0.99, text/plain, text/html;q=0.9, text/plain;q=0.8", content_type="application/json", data='{"x":9,"y":7}'
    When I call /post_echo_text
    Then content_type="text/plain", data='{"x":9,"y":7}'

    Given accept="application/json, text/plain;q=0.99, text/html;q=0.9, text/plain;q=0.8", content_type="text/plain", data='{"x":9,"y":7}'
    When I call /post_echo_text
    Then content_type="application/json", data='{"x":9,"y":7}'

    Given accept="application/json, text/plain;q=0.99, text/html;q=0.9, text/plain;q=0.8", content_type="text/plain", data='{"x":13,"y":15}'
    When I call /post_body_echo_point
    Then content_type="application/json", data='{"x":13.0,"y":15.0}'