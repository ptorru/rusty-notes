# Tar file over gRPC
Trying to send a tar over gRPC using rust!

In essence we have to implement chunking.

Will try the concept using a small file first, but for larger real stuff we will need a bit more work.

# Something unexpected from Rust
Unexpected but make sense... file paths are relative to the location of the Cargo.toml!

See [this question on reddit](https://www.reddit.com/r/rust/comments/73cwro/as_newcomer_with_this_error_trace_how_i_can/)

# Some thoughts
The `tar` library gives us some facilities already, but if you want to use together with `tempfile` there is a bit of complication, because with `tempfile` we get a file handler, not a folder. Without knowing more about `tempfile` we would have to use the example pattern provided by tar, where a loop is used to iterate over every file and get a handler to each. That may be very slow. 

Pending to learn:
* How to get a tempfile path to a folder, so that on the server side we can put the incoming tar there, unpack, and operate from that location.

# References
* [Ciro Costa - Sending files via gRPC](https://ops.tips/blog/sending-files-via-grpc/)
* [SO Answer by Eric Anderson](https://stackoverflow.com/questions/34969446/grpc-image-upload/34982660#34982660)
* [Vinsguru - Implementation example](https://www.vinsguru.com/grpc-file-upload-client-streaming/)
* [Tech School - file in chunks gRPC/Go](https://dev.to/techschoolguru/upload-file-in-chunks-with-client-streaming-grpc-golang-4loc)
* [Tempfile](https://docs.rs/tempfile/3.1.0/tempfile/)
* [Crates.io - Tar](https://crates.io/crates/tar)
