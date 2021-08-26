/// Generates service server
#[macro_export]
#[allow(unused_macros)]
macro_rules! generate_service_server {
    ($name:ident { $($method:ident),* }) => {
        paste::paste! {
            mod server {
                use warp::Filter;
                use super::filters;
                use std::sync::Arc;

                pub(crate) async fn start(service_server: super::[<$name:camel ServiceServer>], port: u16) {
                    let api = filters::mount(Arc::new(service_server));
                    let routes = api.with(warp::log("app"));
                    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
                }
            }

            mod filters {
                use super::handlers;
                use warp::Filter;
                use std::sync::Arc;

                type ServiceServer = Arc<super::[<$name:camel ServiceServer>]>;

                pub(crate) fn mount(service: ServiceServer) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
                    health()
                        $(
                            .or([<$method:snake>](service.clone()))
                        )*
                }

                fn health() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
                    warp::get()
                        .and(warp::path("_health"))
                        .map(|| "Healthy")
                }

                $(
                    fn [<$method:snake>](service: ServiceServer) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
                        warp::post()
                            .and(warp::path(stringify!([<$method:snake>])))
                            .and(warp::body::content_length_limit(1024 * 16))
                            .and(warp::body::json())
                            .and(with_service(service))
                            .and_then(handlers::[<$method:snake>])
                    }
                )*

                fn with_service(service: ServiceServer) -> impl Filter<Extract=(ServiceServer, ), Error=std::convert::Infallible> + Clone {
                    warp::any().map(move || service.clone())
                }
            }

            mod handlers {
                use std::convert::Infallible;
                use std::sync::Arc;
                use [<$name:lower _client>]::[<$name:camel Service>];
                $(
                    use [<$name:lower _client>]::[<$method:camel Request>];
                )*

                type ServiceServer = Arc<super::[<$name:camel ServiceServer>]>;

                $(
                    pub(crate) async fn [<$method:snake>](req: [<$method:camel Request>], service: ServiceServer) -> Result<impl warp::Reply, Infallible> {
                        match service.[<$method:snake>](req).await {
                            Ok(res) => {
                                Ok(warp::reply::with_status(
                                    warp::reply::json(&res),
                                    warp::http::StatusCode::OK)
                                )
                            }
                            Err(e) => {
                                Ok(warp::reply::with_status(
                                    warp::reply::json(&e),
                                    e.status.into())
                                )
                            }
                        }
                    }
                )*
            }
        }
    }
}
