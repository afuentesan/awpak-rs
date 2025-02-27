Feature: Path Variable feature

  Scenario: If we send path variable receive same path variable
    
    When I call /get_echo/path_variable/string/hello
    Then response="hello"

    When I call /get_echo/37/string_usize/goodbye
    Then response="goodbye 37"

    When I call /get_echo/path_variable/object/33
    Then response='{"x":33}'

    When I call /get_echo/path_variable/object/asdf
    Then response='Server Error'