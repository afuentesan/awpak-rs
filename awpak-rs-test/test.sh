cargo build

cargo run > /tmp/salida_server 2>&1 & 

PID_SERVER=$!

sleep 2

if [ "$1" != "" ]
then
cargo test --test $1
else
cargo test --test test_get
cargo test --test test_post
cargo test --test test_post_params
cargo test --test test_multipart_json
cargo test --test test_multipart_files
cargo test --test test_content_negotiation
cargo test --test test_content_type
cargo test --test test_post_2
cargo test --test test_multipart_data
cargo test --test test_path_variables
cargo test --test test_headers
cargo test --test test_cookies
cargo test --test test_status_code
cargo test --test test_query_param
cargo test --test test_get_2
cargo test --test test_redirects
fi

kill $PID_SERVER