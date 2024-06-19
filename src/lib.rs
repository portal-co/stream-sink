use futures_core::Stream;
use futures_sink::Sink;

#[pin_project::pin_project]
pub struct StreamSink<S,Y>{
    #[pin]
    pub stream: S,
    #[pin]
    pub sink: Y
}
impl<S: Stream,Y> Stream for StreamSink<S,Y>{
    type Item = S::Item;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        return self.project().stream.poll_next(cx);
    }
}
impl<S,Y: Sink<T>,T> Sink<T> for StreamSink<S,Y>{
    type Error = Y::Error;

    fn poll_ready(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().sink.poll_ready(cx)
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.project().sink.start_send(item)
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().sink.poll_flush(cx)
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().sink.poll_close(cx)
    }
}