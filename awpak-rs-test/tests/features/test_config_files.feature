Feature: Config files feature

  Scenario: Request for contents of config file
    When I call /get_db_config_file_user_pass
    Then response="user:pass"

    When I call /get_db_config_file_user_pass_value
    Then response="user2:pass2"