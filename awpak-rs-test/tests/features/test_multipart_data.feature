Feature: Post multipart data feature

  Scenario: If we send File and data receive file len and data
    
    Given path="tests/files/wallpaper_ini.jpg", filename="wallpaper_ini.jpg", name="img", content_type="image/jpg", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data
    Then response="399268, hello, goodbye"

    Given path="", filename="", name="", content_type="", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_optional_file
    Then response="0, hello, goodbye"

    Given path="tests/files/wallpaper_ini.jpg", filename="wallpaper_ini.jpg", name="img", content_type="image/jpg", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_optional_file
    Then response="399268, hello, goodbye"

    Given path="tests/files/wallpaper_ini.jpg;tests/files/wallpaper.webp", filename="wallpaper_ini.jpg;wallpaper.webp", name="img_1;img_2", content_type="image/jpg;image/webp", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_two_optional_files
    Then response="399268, 80124, hello, goodbye"

    Given path="", filename="", name="", content_type="", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_two_optional_files
    Then response="0, 0, hello, goodbye"

    Given path="tests/files/wallpaper_ini.jpg", filename="wallpaper_ini.jpg", name="img_1", content_type="image/jpg", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_two_optional_files
    Then response="399268, 0, hello, goodbye"

    Given path="tests/files/wallpaper_ini.jpg", filename="wallpaper_ini.jpg", name="img_2", content_type="image/jpg", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_two_optional_files
    Then response="0, 399268, hello, goodbye"

    Given path="tests/files/wallpaper_ini.jpg;tests/files/wallpaper.webp", filename="wallpaper_ini.jpg;wallpaper.webp", name="img;img", content_type="image/jpg;image/webp", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_optional_vec_of_files
    Then response="399268, 80124, hello, goodbye"

    Given path="", filename="", name="", content_type="", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_optional_vec_of_files
    Then response="hello, goodbye"

    Given path="tests/files/wallpaper_ini.jpg", filename="wallpaper_ini.jpg", name="img", content_type="image/jpg", param_1="hello", param_2="goodbye"
    When I call /post_multipart_data_optional_vec_of_files
    Then response="399268, hello, goodbye"

    