use poem::{
    handler,
    http::{Method, Uri},
    web::RemoteAddr,
};

#[handler]
fn index(remote_addr: &RemoteAddr, method: Method, uri: &Uri) {}
