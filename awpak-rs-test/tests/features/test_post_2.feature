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

  Scenario: Async deserializer

    Given request_body='{"x":1,"y":1}', content_type="application/json"
    When I call /post_point_with_context
    Then response='{"point":null,"x":1.0,"y":201.0}'

    Given request_body='{"x":1,"y":1,"point":{"x":2,"y":2}}', content_type="application/json"
    When I call /post_point_with_context
    Then response='{"point":{"x":2.0,"y":2.0},"x":1.0,"y":201.0}'

    Given request_body='{"x":1,"y":1,"point":{"x":2,"y":2}}', content_type="application/json"
    When I call /post_point_with_context_wrapper
    Then response='{"point":{"point":null,"x":2.0,"y":202.0},"x":201.0,"y":1.0}'

    Given request_body='{"x":1,"y":1,"point":{"x":2,"y":2,"point":{"x":3,"y":3}}}', content_type="application/json"
    When I call /post_point_with_context_wrapper
    Then response='{"point":{"point":{"x":3.0,"y":3.0},"x":2.0,"y":202.0},"x":201.0,"y":1.0}'

    Given request_body='1', content_type="application/json"
    When I call /post_request_body_from_async_str
    Then response='{"status":201}'