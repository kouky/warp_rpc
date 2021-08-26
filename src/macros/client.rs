/// Generates service client
///
/// Generates a http client for a service trait, see
/// [warp_rpc_example](https://github.com/kouky/warp_rpc_example) on Github for a detailed example.
///
/// # Example
///
/// Define a service client in a library crate.
///
/// ```rust
/// // `lib.rs` in `media_client` crate
/// use serde::{Deserialize, Serialize};
/// use warp_rpc::error::ServiceError;
/// use warp_rpc::generate_service_client;
///
/// #[async_trait::async_trait]
/// pub trait MediaService {
///     async fn get_image(&self, req: GetImageRequest) -> Result<GetImageResponse, ServiceError>;
///     async fn get_video(&self, req: GetVideoRequest) -> Result<GetVideoResponse, ServiceError>;
/// }
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct GetImageRequest {}
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct GetImageResponse {}
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct GetVideoRequest {}
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct GetVideoResponse {}
///
/// // Generates public `MediaClient` http client which implements `MediaService` trait
/// # #[macro_use] extern crate warp_rpc;
/// # fn main() {
/// generate_service_client!(
///     media { get_image, get_video }
/// );
///
/// # // Compile but don't dispay in docs
/// # let client =  service::MediaClient::new("http://0.0.0.0:8000");
/// # let _ = client.get_image(GetImageRequest{});
/// # let _ = client.get_video(GetVideoRequest{});
/// # }
/// ```
///
/// Use the generated service client from another crate
///
/// ```ignore
/// use media_client::{MediaService, GetImageRequest};
/// use media_client::service::MediaClient;
///
/// #[tokio::main]
/// async fn main() {
///     env_logger::init();
///     let client = MediaClient::new("http://0.0.0.0:8000");
///     let result = client.get_image(GetImageRequest{}).await;
///     match result {
///         Ok(resp) => log::info!("{:?}", resp),
///         Err(e) => log::error!("{}", e),
///     }
/// }
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! generate_service_client {
    ($name:ident { $($method:ident),* }) => {
        paste::paste! {
            pub mod service {
                use warp_rpc::error::ServiceError;
                use std::borrow::Cow;
                use paste::paste;
                use super::[<$name:camel Service>];
                $(
                    use super::{[<$method:camel Request>], [<$method:camel Response>]};
                )*

                #[allow(dead_code)]
                #[derive(Debug)]
                pub struct [<$name:camel Client>]<'a> {
                    url: Cow<'a, str>,
                    client: reqwest::Client,
                }

                #[allow(dead_code)]
                impl<'a> [<$name:camel Client>]<'a> {
                    pub fn new<S>(url: S) -> Self
                        where S: Into<Cow<'a, str>>
                    {
                        Self {
                            url: url.into(),
                            client: reqwest::Client::new(),
                        }
                    }
                }

                #[async_trait::async_trait]
                impl [<$name:camel Service>] for [<$name:camel Client>]<'_> {
                    $(
                        async fn [<$method:snake>](&self, req: [<$method:camel Request>]) -> Result<[<$method:camel Response>], ServiceError> {
                            let url = format!("{}/{}", self.url, stringify!([<$method:snake:lower>]));
                            let response = self.client.post(url)
                                .json(&req)
                                .send()
                                .await?;

                            if response.status().is_success() {
                                let res: [<$method:camel Response>] = response.json().await?;
                                Ok(res)
                            } else {
                                let err: ServiceError = response.json().await?;
                                Err(err)
                            }
                        }
                    )*
                }
            }
        }
    }
}