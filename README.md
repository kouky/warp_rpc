# Warp RPC

Build Rust RPC services powered by Warp + Reqwest.

Provides server and client macros to implement the boilerplate for service RPC over
HTTP using JSON payloads. Uses [Warp](https://github.com/seanmonstar/warp) to implement a
server and the [Reqwest](https://github.com/seanmonstar/reqwest) http library to implement the
client.

* [`generate_service_client`] and [`generate_service_server`] macros implement RPC boilerplate
* [`error::ServiceError`] for service response errors

Submit issues and pull requests on the [warp_rpc github](https://github.com/kouky/warp_rpc) repo.

See [warp_rpc_example](https://github.com/kouky/warp_rpc_example) on Github for a detailed
working example.

## License

MIT
