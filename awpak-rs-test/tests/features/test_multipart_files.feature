Feature: Post multipart files feature

  Scenario: If we send File receive len
    
    Given path=tests/files/wallpaper_ini.jpg, filename=wallpaper_ini.jpg, name=img, content_type=image/jpg
    When I call /post_multipart_file_len
    Then len=399268

    Given path=tests/files/wallpaper_ini.jpg;tests/files/wallpaper.webp, filename=wallpaper_ini.jpg;wallpaper.webp, name=img;img, content_type=image/jpg;image/webp
    When I call /post_multipart_files_len
    Then len=479392