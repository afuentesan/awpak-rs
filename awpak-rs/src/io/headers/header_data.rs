
/// Represents an individual HTTP header.
///
/// This struct contains the name of the header and its value, which can be stored as a string or raw binary data.
///
/// # Fields
///
/// - `name` – The name of the header (e.g., `"Content-Type"`).
/// - `value` *(optional)* – The header value as a `String` if it contains text data.
/// - `value_bytes` – The header value in raw binary form.
///
/// # Notes
///
/// - If the header contains non-textual data, `value` will be `None`, and the raw data should be accessed via `value_bytes`.
/// - If the header contains textual data, `value` will hold the string representation.
#[derive(Clone)]
pub struct HeaderData
{
    pub name : String,
    pub value : Option<String>,
    pub value_bytes : Vec<u8>
}

impl HeaderData
{
    pub fn new( name : String, value_bytes : Vec<u8>, value : Option<String> ) -> Self
    {
        Self
        {
            name,
            value,
            value_bytes
        }
    }
}