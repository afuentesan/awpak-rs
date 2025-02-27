
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