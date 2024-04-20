// 引入 std 库中的 boxed 模块
use std::boxed::Box;

// 定义链表节点
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

// 链表结构
struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

// 实现链表的一些方法
impl<T> LinkedList<T> {
    // 创建一个新的空链表
    fn new() -> Self {
        Self { head: None }
    }

    // 在链表头部插入一个新节点
    fn push(&mut self, data: T) {
        let new_node = ListNode {
            data: data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    // 弹出链表头部的节点
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // 遍历打印链表中的所有节点
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

