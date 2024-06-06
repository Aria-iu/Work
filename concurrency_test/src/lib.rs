#![allow(unused)]

extern crate arc_swap;
extern crate cuckoofilter;
extern crate dashmap;
extern crate evmap;
extern crate tokio;

mod ticker;

use ticker::Ticker;

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use std::sync::{Arc, Mutex};
    use std::thread::{self, yield_now};
    use std::time::Duration;

    use crate::ticker::Ticker;

    #[test]
    fn dashmap_test() {
        let map = Arc::new(DashMap::new());

        let map1 = Arc::clone(&map);
        let handle1 = thread::spawn(move || {
            map1.insert("key1", "value1");
        });

        let map2 = Arc::clone(&map);
        let handle2 = thread::spawn(move || {
            map2.insert("key2", "value2");
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        println!("{:?}", map.get("key1"));
        println!("{:?}", map.get("key2"))
    }

    #[test]
    fn cuckoofilter_test() {
        let value: &str = "hello world";

        // Create cuckoo filter with default max capacity of 1000000 items
        let mut cf = cuckoofilter::CuckooFilter::new();
        // Add data to the filter
        let success = cf.add(value).unwrap();
        // success ==> Ok(())

        // Lookup if data is in the filter
        let success = cf.contains(value);
        // success ==> true

        // Test and add to the filter (if data does not exists then add)
        let success = cf.test_and_add(value).unwrap();
        // success ==> Ok(false)

        // Remove data from the filter.
        let success = cf.delete(value);
    }

    #[test]
    fn evmap_test() {
        let (book_r, book_w) = evmap::new();
        let w = Arc::new(Mutex::new(book_w));
        let writer: Vec<_> = (0..4)
            .map(|i| {
                let w = w.clone();
                std::thread::spawn(move || {
                    let mut w = w.lock().unwrap();
                    w.insert(i, true);
                    w.refresh();
                })
            })
            .collect();

        while book_r.len() < 4 {
            yield_now();
        }
        for w in writer.into_iter() {
            assert!(w.join().is_ok());
        }
    }

    // 在多线程环境中，若需要大量的读取数据，少量的写入数据，引入Arc::Mutex<T>，进行🔓的操作是不划算的
    // 可以使用类似于Mutex<Arx<T>>的结构，来实现引用共享（只有一个引用）
    #[test]
    fn arc_swap_test() {
        use arc_swap::ArcSwap;

        let data = ArcSwap::new(1i32.into());

        println!("Initial Value: {}", data.load());

        data.store(Arc::new(2));

        println!("New Value: {}", data.load());
    }

    #[test]
    fn timer_test() {
        println!(" begin");
        // 使用标准库中的 std::thread::sleep 来创建一个简单的定时器
        let _ret = thread::sleep(Duration::from_secs(5));
        println!("5 s end");
    }

    #[test]
    fn ticker_test() {
        let ticker = Ticker::new(0..10, Duration::from_secs(1));

        for value in ticker {
            println!("Value: {}", value);
        }
    }

    fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
        if v.len() > 1 {
            let mid = partition(v);
            let (lo, hi) = v.split_at_mut(mid);
            rayon::join(|| quick_sort(lo), || quick_sort(hi));
        }
    }

    fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
        let pivot = v.len() - 1;
        let mut i = 0;
        for j in 0..pivot {
            if v[j] <= v[pivot] {
                v.swap(i, j);
                i += 1;
            }
        }
        v.swap(i, pivot);
        i
    }
    // 使用rayon库完成并发的排序
    #[test]
    fn rayon_test() {
        let mut v = vec![5, 1, 8, 22, 0, 44];
        quick_sort(&mut v);
        assert_eq!(v, vec![0, 1, 5, 8, 22, 44]);
    }
}
