use crate::{io::io::IO, error::error::Error};

pub type EndpointReturnType = std::pin::Pin<std::boxed::Box<
                                    dyn std::future::Future<
                                        Output = std::result::Result<
                                            IO,
                                            Error
                                        >
                                    > 
                                    + std::marker::Send
                                >>;