Feature: Post 2 feature

  Scenario: If we send Some receive same Some

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /post_body_echo_point
    Then response='{"x":3.0,"y":5.0}'

    Given request_body='[7, 9]', content_type="application/json"
    When I call /post_request_body_vec_u32_echo
    Then response='[7,9]'

    Given request_body='7, 9', content_type="text/plain"
    When I call /post_request_body_vec_u32_echo
    Then response='[7,9]'
  
  Scenario: If we send x add one in both urls. If we send y add one in both urls. Add z if url is /post_add_z_in_middleware

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /post_add_z_in_middleware
    Then response='{"x":4.0,"y":6.0,"z":333}'

    Given request_body='{ "x" : 17, "y" : 23 }', content_type="application/json"
    When I call /post_add_one_to_x_in_middleware
    Then response='{"x":18.0,"y":24.0}'

  Scenario: Add one to x in post

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /add_one_to_y_if_get_or_x_if_post
    Then response='{"x":4.0,"y":5.0}'

  Scenario: Add z and test if z exists

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /add_z_test_order
    Then response='{"has_z":true,"x":3.0,"y":5.0,"z":333}'

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /add_z_test_order_false
    Then response='{"has_z":false,"x":3.0,"y":5.0,"z":333}'

  Scenario: Add z and test pre if z exists

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /add_z_test_pre_order
    Then response='{"has_z":true,"x":3.0,"y":5.0,"z":333.0}'

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /add_z_test_pre_order_false
    Then response='{"has_z":false,"x":3.0,"y":5.0,"z":333.0}'

  Scenario: Context
    
    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /post_echo_context
    Then response='x:78'

    Given request_body='{ "x" : 3, "y" : 5 }', content_type="application/json"
    When I call /post_echo_context_mut
    Then response='x:79'