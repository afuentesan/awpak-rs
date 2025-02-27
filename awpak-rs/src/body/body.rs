
#[derive(Clone)]
pub struct FileData
{
    pub name : String,
    pub filename : String,
    pub bytes : Vec<u8>,
    pub content_type : String
}

impl FileData
{
    pub fn new( name : String, filename : String, bytes : Vec<u8>, content_type : String ) -> Self
    {
        Self
        {
            name,
            filename,
            bytes,
            content_type
        }
    }
}

/// Represents the body of an HTTP request.
///
/// The body can contain structured JSON data or uploaded files, depending on the request's content type.
pub struct BodyData
{
    /// The JSON payload of the request body, if applicable.
    ///
    /// If the request has a JSON body (e.g., `Content-Type: application/json`), it will be stored here.
    /// You can manipulate this value using the `serde_json` API.
    ///
    /// # Example: Reading JSON data
    /// ```rust
    /// use awpak_rs::body::body::BodyData;
    /// 
    /// let body_data = BodyData { value : None, files : vec![] };
    /// 
    /// if let Some(json_value) = &body_data.value {
    ///     println!("Received JSON: {}", json_value);
    /// }
    /// ```
    ///
    /// # Example: Modifying JSON data
    /// ```rust
    /// use awpak_rs::body::body::BodyData;
    /// 
    /// let mut body_data = BodyData { value : None, files : vec![] };
    /// 
    /// if let Some(json_value) = &mut body_data.value {
    ///     json_value["new_key"] = serde_json::json!("new_value");
    /// }
    /// ```
    pub value : Option<serde_json::Value>,
    pub files : Vec<FileData>
}

impl BodyData
{
    pub fn get_param( &self, name : &str ) -> Option<&serde_json::Value>
    {
        if self.value.is_none()
        {
            return None
        }

        let param = self.value.as_ref().unwrap();

        let param = param.get( name );

        if param.is_none()
        {
            return None
        }

        Some( param.unwrap() )
    }

    pub fn get_file( &self, name : &str ) -> Option<FileData>
    {
        for i in 0..self.files.len()
        {
            if self.files[ i ].name.as_str() == name
            {
                return Some( self.files[ i ].clone() )
            }
        }

        None
    }

    pub fn get_files( &self, name : &str ) -> Option<Vec<FileData>>
    {
        let mut ret : Vec<FileData> = vec![];

        for i in 0..self.files.len()
        {
            if self.files[ i ].name.as_str() == name
            {
                ret.push( self.files[ i ].clone() )
            }
        }

        if ret.len() == 0
        {
            return None
        }
        
        Some( ret )
    }
}

pub trait ToFileData
{
    fn to_file_data( body : &BodyData, name : &str  ) -> Result<Self, ()> where Self: Sized;    
}

impl ToFileData for FileData
{
    fn to_file_data( body : &BodyData, name : &str ) -> Result<Self, ()> where Self: Sized
    {
        match body.get_file( name ) {
            Some( f ) => Ok( f ),
            _ => Err( () )
        }
    }
}

impl ToFileData for Vec<FileData>
{
    fn to_file_data( body : &BodyData, name : &str ) -> Result<Self, ()> where Self: Sized
    {
        match body.get_files( name ) {
            Some( f ) => Ok( f ),
            _ => Err( () )
        }
    }
}

impl<T> ToFileData for Option<T>
where T: ToFileData
{
    fn to_file_data( body : &BodyData, name : &str  ) -> Result<Self, ()> where Self: Sized
    {
        match T::to_file_data( body, name )
        {
            Ok( v ) => Ok( Some( v ) ),
            _ => Ok( None )
        }
    }
}