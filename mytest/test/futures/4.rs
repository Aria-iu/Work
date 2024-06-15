
/**
 * 一个实例，使用Sink来进行异步写入
 */
use futures::executor::LocalPool;
use futures::task::{LocalSpawnExt, Poll};
use futures::sink::Sink;
use futures::stream::{self, StreamExt};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Waker;

 struct MySink{
    buffer: Vec<String>,
    waker: Option<Waker>,
 }

 impl MySink{
    fn new() -> Self{
        MySink{
            buffer: Vec::new(),
            waker: None,
        }
    }
 }

 impl Sink<String> for MySink{
    type Error = ();
 
    fn poll_ready(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.waker = Some(cx.waker().clone());
        Poll::Ready(Ok(()))
    }
 
    fn start_send(mut self: std::pin::Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
        self.buffer.push(item);
        Ok(())
    }
 
    fn poll_flush(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.buffer.clear();
        if let Some(waker) = self.waker.take(){
            waker.wake();
        }
        Poll::Ready(Ok(()))
    }
 
    fn poll_close(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.buffer.clear();
        Poll::Ready(Ok(()))
    }
 }



fn main(){

    // 创建一个本地任务池
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let sink = Arc::new(Mutex::new(MySink::new()));
    // 模拟数据流
    let stream = stream::iter(vec!["item1".to_string(), "item2".to_string(), "item3".to_string()]);
    
    let _ = spawner.spawn_local(async move  {
        let sink = Arc::clone(&sink);
        let mut stream = stream;
        while let Some(item) = stream.next().await {
            let mut sink = sink.lock().unwrap();
            let mut pin_sink = Pin::new(&mut *sink);

            // 确保Sink已经准备好接受数据
            futures::future::poll_fn(|cx| pin_sink.as_mut().poll_ready(cx)).await.unwrap();

            // 发送数据
            pin_sink.as_mut().start_send(item).unwrap();

            // 刷新Sink
            futures::future::poll_fn(|cx| pin_sink.as_mut().poll_flush(cx)).await.unwrap();
        }

        // 关闭Sink
        let mut sink = sink.lock().unwrap();
        let mut pin_sink = Pin::new(&mut *sink);
        futures::future::poll_fn(|cx| pin_sink.as_mut().poll_close(cx)).await.unwrap();
    });  

    pool.run();

}