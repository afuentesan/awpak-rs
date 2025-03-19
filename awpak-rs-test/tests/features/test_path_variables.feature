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

    When I call /get_echo/path_variable/object/custom_deserializer/33
    Then response='{"x":233}'

    When I call /get_echo/path_variable/vec/u32/1,2,3
    Then response='[1,2,3]'

    When I call /get_echo/path_variable/vec/objects/1,2,3
    Then response='[{"x":1},{"x":2},{"x":3}]'

    When I call /get_echo/path_variable_renamed/vec/u32/1,2,3
    Then response='[1,2,3]'