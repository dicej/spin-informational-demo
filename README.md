# spin-informational-demo

This demonstrates sending HTTP 1xx informational responses using forks of
[Spin](https://github.com/fermyon/spin) and
[Hyper](https://github.com/hyperium/hyper).

## Building and Running

First, make sure you have [Rust](https://rustup.rs) installed.  Then run:

```
cargo install --locked --git https://github.com/dicej/spin --branch informational-responses spin-cli
spin build --up
```

Then, in another terminal, run:

```
curl -v localhost:3000
```

...which should output something like this:

```
* Host localhost:3000 was resolved.
* Connected to localhost (127.0.0.1) port 3000
> GET / HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/8.5.0
> Accept: */*
>
< HTTP/1.1 103 <none>
< link: <https://cdn.example.com>; rel=preconnect, <https://cdn.example.com>; rel=preconnect; crossorigin
< date: Tue, 17 Dec 2024 18:49:33 GMT
< HTTP/1.1 200 OK
< content-length: 0
< date: Tue, 17 Dec 2024 18:49:33 GMT
<
* Connection #0 to host localhost left intact
```
