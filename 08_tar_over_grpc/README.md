# Tar file over gRPC
Trying to send a tar over gRPC using rust!

In essence we have to implement chunking.

Will try the concept using a small file first, but for larger real stuff we will need a bit more work.


# References
* [Ciro Costa - Sending files via gRPC](https://ops.tips/blog/sending-files-via-grpc/)
* [SO Answer by Eric Anderson](https://stackoverflow.com/questions/34969446/grpc-image-upload/34982660#34982660)
* [Vinsguru - Implementation example](https://www.vinsguru.com/grpc-file-upload-client-streaming/)
* [Tech School - file in chunks gRPC/Go](https://dev.to/techschoolguru/upload-file-in-chunks-with-client-streaming-grpc-golang-4loc)
