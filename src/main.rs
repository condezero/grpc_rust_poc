use tonic::{transport::Server, Request, Response, Status};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use bookstore::bookstore_server::{Bookstore, BookstoreServer};
use bookstore::{GetBookRequest, GetBookResponse};


mod bookstore {
    include!("bookstore.rs");

    // Add this
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("greeter_descriptor");
}


#[derive(Default)]
pub struct BookStoreImpl {}

#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = GetBookResponse {
            author: "Rusty".to_owned(),
            title: "Rust".to_owned(),
            price: 20.0,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001);
    let bookstore = BookStoreImpl::default();

    // Add this
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(bookstore::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("Bookstore server listening on {}", addr);

    Server::builder()
        .add_service(BookstoreServer::new(bookstore))
        .add_service(reflection_service) // Add this
        .serve(addr)
        .await?;

    Ok(())
}