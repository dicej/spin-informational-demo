#![deny(warnings)]

use {
    bindings::spin::http::http::send_informational,
    wasi::http::types::{Headers, IncomingRequest, OutgoingResponse, ResponseOutparam},
};

mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "http-trigger",
        with: {
            "wasi:cli/environment@0.2.0": ::wasi::cli::environment,
            "wasi:cli/exit@0.2.0": ::wasi::cli::exit,
            "wasi:cli/stdin@0.2.0": ::wasi::cli::stdin,
            "wasi:cli/stdout@0.2.0": ::wasi::cli::stdout,
            "wasi:cli/stderr@0.2.0": ::wasi::cli::stderr,
            "wasi:cli/terminal-input@0.2.0": ::wasi::cli::terminal_input,
            "wasi:cli/terminal-output@0.2.0": ::wasi::cli::terminal_output,
            "wasi:cli/terminal-stdin@0.2.0": ::wasi::cli::terminal_stdin,
            "wasi:cli/terminal-stdout@0.2.0": ::wasi::cli::terminal_stdout,
            "wasi:cli/terminal-stderr@0.2.0": ::wasi::cli::terminal_stderr,
            "wasi:clocks/monotonic-clock@0.2.0": ::wasi::clocks::monotonic_clock,
            "wasi:clocks/wall-clock@0.2.0": ::wasi::clocks::wall_clock,
            "wasi:config/store@0.2.0-draft-2024-09-27": generate,
            "wasi:filesystem/types@0.2.0": ::wasi::filesystem::types,
            "wasi:filesystem/preopens@0.2.0": ::wasi::filesystem::preopens,
            "fermyon:spin/llm@2.0.0": generate,
            "fermyon:spin/redis@2.0.0": generate,
            "fermyon:spin/mqtt@2.0.0": generate,
            "fermyon:spin/rdbms-types@2.0.0": generate,
            "fermyon:spin/postgres@2.0.0": generate,
            "fermyon:spin/mysql@2.0.0": generate,
            "fermyon:spin/sqlite@2.0.0": generate,
            "fermyon:spin/key-value@2.0.0": generate,
            "fermyon:spin/variables@2.0.0": generate,
            "spin:postgres/postgres@3.0.0": generate,
            "spin:http/http@3.0.0": generate,
            "wasi:http/incoming-handler@0.2.0": generate,
            "wasi:http/outgoing-handler@0.2.0": ::wasi::http::outgoing_handler,
            "wasi:http/types@0.2.0": ::wasi::http::types,
            "wasi:io/poll@0.2.0": ::wasi::io::poll,
            "wasi:io/error@0.2.0": ::wasi::io::error,
            "wasi:io/streams@0.2.0": ::wasi::io::streams,
            "wasi:keyvalue/store@0.2.0-draft2": generate,
            "wasi:keyvalue/atomics@0.2.0-draft2": generate,
            "wasi:keyvalue/batch@0.2.0-draft2": generate,
            "wasi:sockets/network@0.2.0": ::wasi::sockets::network,
            "wasi:sockets/instance-network@0.2.0": ::wasi::sockets::instance_network,
            "wasi:sockets/udp@0.2.0": ::wasi::sockets::udp,
            "wasi:sockets/udp-create-socket@0.2.0": ::wasi::sockets::udp_create_socket,
            "wasi:sockets/tcp@0.2.0": ::wasi::sockets::tcp,
            "wasi:sockets/tcp-create-socket@0.2.0": ::wasi::sockets::tcp_create_socket,
            "wasi:sockets/ip-name-lookup@0.2.0": ::wasi::sockets::ip_name_lookup,
            "wasi:random/random@0.2.0": ::wasi::random::random,
            "wasi:random/insecure@0.2.0": ::wasi::random::insecure,
            "wasi:random/insecure-seed@0.2.0": ::wasi::random::insecure_seed,
        }
    });

    use super::Component;
    export!(Component);
}

struct Component;

impl bindings::exports::wasi::http0_2_0::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, out: ResponseOutparam) {
        send_informational(
            &out,
            103,
            Headers::from_list(&[(
                "link".to_string(),
                b"<https://cdn.example.com>; \
                  rel=preconnect, <https://cdn.example.com>; \
                  rel=preconnect; crossorigin"
                    .to_vec(),
            )])
            .unwrap(),
        );
        ResponseOutparam::set(out, Ok(OutgoingResponse::new(Headers::new())));
    }
}
