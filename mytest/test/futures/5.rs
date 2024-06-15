/**
 * 思考为什么输出为
 * a is : Ok(())
 * flat_map
 * 
 */

use futures::{executor::LocalPool, task::LocalSpawnExt};

fn main(){
    use futures::channel::mpsc;
    use futures::sink::SinkExt;
    use futures::stream::{self, StreamExt};

    // 创建一个本地任务池
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (tx, rx) = mpsc::channel(5);

    let mut tx = tx.with_flat_map(|x| {
        
        stream::iter(vec![Ok(42); x])
    });

    let a = spawner.spawn_local(async move{
        tx.send(5).await.unwrap();
        println!("flat_map");
        drop(tx);
        let received: Vec<i32> = rx.collect().await;
        assert_eq!(received, vec![42,42, 42, 42, 42]);
    });
    
    
    println!("a is : {:?}",a);
    pool.run();
    
}