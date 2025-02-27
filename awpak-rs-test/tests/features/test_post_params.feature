Feature: Post params feature

    Scenario: If we send param receive same param

    Given param=Some(64)
    When I call param /post_body_param_number_echo
    Then param=Some(64)

    Given param=Some("asdf")
    When I call param /post_body_param_string_echo
    Then param=Some(asdf)

    Given param=Some(128)
    When I call param /post_body_param_string_echo
    Then param=Some(128)

    Given param=Some("128,33,28,17")
    When I call param /post_body_param_vec_i16_echo
    Then param=Some([128,33,28,17])

    Given param=Some([128,33,28,18])
    When I call param /post_body_param_vec_i16_echo
    Then param=Some([128,33,28,18])

    Given param=Some(["hello","goodbye"])
    When I call param /post_body_param_vec_string_echo
    Then param=Some(["hello","goodbye"])

    Given param=Some("")
    When I call param /post_body_param_option_string_echo
    Then param=Some()

    Given param=Some("asdf")
    When I call param /post_body_param_option_string_echo
    Then param=Some(asdf)