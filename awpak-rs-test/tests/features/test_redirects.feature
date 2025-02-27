Feature: Redirects feature

  Scenario: If we send param location redirect to location

    Given location="/" status=""
    When I call /get_redirect_default
    Then location="/" status="301"

    Given location="/hello" status="307"
    When I call /get_redirect_with_status
    Then location="/hello" status="307"