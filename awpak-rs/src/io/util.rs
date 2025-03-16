
#[macro_export]
macro_rules! set_status_code_mcr {
    ( $io : ident, $status : expr ) => {
        $io.response.status = $status;
    };
}

#[macro_export]
macro_rules! redirect_to_mcr {
    ( $io : ident, $rh : ident, $url : expr ) => {

        $io.response.status = 301;

        $rh.replace_header( "Location".into(), $url );
    };
    ( $io : ident, $rh : ident, $url : expr, $status_code : expr ) => {

        $io.response.status = $status_code;

        $rh.replace_header( "Location".into(), $url );
    };
}

#[macro_export]
macro_rules! get_request_body_as {
    ( $io : ident, $ty : ident ) => {
        {
            // let __bytes = $io.request.body.data.clone();
            // let __io_get_request_body_as = std::sync::Arc::new( std::sync::Mutex::new( Some( $io ) ) );

            // let __result = <$ty>::deserialize_with_io( __bytes, __io_get_request_body_as.clone() );

            // let mut $io = __io_get_request_body_as.lock().unwrap().take().unwrap();

            // __result

            let ( __result, __result_io ) = awpak_rs::parse_body_with_io::<$ty>( $io );

            $io = __result_io;

            __result
        }
        
    };
}