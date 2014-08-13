# distributed-map

A toy 'distributed' map in Rust. Consists of a server which serves requests to a `Map<String, String>`, and a client which issues requests. Uses [TCP streams](http://doc.rust-lang.org/std/io/net/tcp/index.html) for communication and [Cap'n Proto](https://github.com/dwrensha/capnproto-rust) for request/response message formats.

```bash
$ distributed-map server &

$ distributed-map client put my-key my-value
new key my-key: true

$ distributed-map client contains other-key
contains other-key: false

$distributed-map client get my-key
get my-key: Some(my-value)
```