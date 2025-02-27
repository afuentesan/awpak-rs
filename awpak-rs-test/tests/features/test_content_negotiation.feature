Feature: Content negotiation feature

  Scenario: Content negotiation
    Given accept="application/json, text/plain;q=0.99, text/html;q=0.9, text/plain;q=0.8"
    When I call /
    Then content_type="application/json"

    Given accept="text/plain, application/json;q=0.99, text/html;q=0.9, text/plain;q=0.8, */*;q=0.6"
    When I call /
    Then content_type="text/plain"

    Given accept="application/json;q=0.99, text/plain, text/html;q=0.9, text/plain;q=0.8, */*;q=0.6"
    When I call /
    Then content_type="text/plain"

    Given accept="*/json, text/plain;q=0.95, text/html;q=0.9, text/plain;q=0.8"
    When I call /
    Then content_type="application/json"

    Given accept="*/json;q=0.95, text/plain, text/html;q=0.9, text/plain;q=0.8"
    When I call /
    Then content_type="text/plain"