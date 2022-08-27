use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

// 单向引用计数链表实现
// 这是一种相当实用的结构体应用形式，每个 append 方法通过将数据添加到头部的方式
// 运行，这意味着我们不必使用引用和实际的列表引用就可以确保不变性。
#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    data: T,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}
impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&self, data: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
        }
    }
}

// 双向列表实现
// 第一个元素prev none
// #[derive(Debug)]
// struct LinkedListNode<T> {
//     prev: Option<Weak<LinkedListNode<T>>>, // 弱引用计数
//     next: Option<Rc<LinkedListNode<T>>>,   // 强引用
//     data: T,
// }
// #[derive(Debug)]
// struct DoubleLinkedList<T> {
//     head: Option<Rc<LinkedListNode<T>>>,
// }

// impl<T> DoubleLinkedList<T> {
//     fn new() -> Self {
//         Self { head: None }
//     }
//
//     fn append(&self, data: T) -> Self {
//         let new_node = Rc::new(LinkedListNode {
//             data: data,
//             next: self.head.clone(),
//             prev: None,
//         });
//
//         match self.head.clone() {
//             Some(node) => {
//                 // 这一行是问题的^^^^^^^^^ cannot assign
//                 node.prev = Some(Rc::downgrade(&new_node));
//             }
//             None => {}
//         }
//
//         Self {
//             head: Some(new_node),
//         }
//     }
// }

// RefCell 可变借用，内部可变
// 将借用检查从编译时移动到运行时，这是通过内部可变性实现的
// Cell 和 RefCell 可以将不可变的
// 内容转换成可变的，允许我们将不可变的结构体中的某个部分定义为可变的
#[derive(Debug)]
struct LinkedListNode<T> {
    prev: RefCell<Option<Weak<LinkedListNode<T>>>>, // 弱引用计数，内部可变
    next: Option<Rc<LinkedListNode<T>>>,            // 强引用
    data: T,
}

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Option<Rc<LinkedListNode<T>>>,
}

impl<T> DoubleLinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    // 让append接收一个指向self的可变引用,但所有的节点的绑定是可变的，让结构体部分可变，用RefCell实现
    // 这里，我们修改了 append 方法以创建新的 RefCell，并通过 RefCell 可变借用更新之前的引用
    fn append(&self, data: T) -> Self {
        let new_node = Rc::new(LinkedListNode {
            data: data,
            next: self.head.clone(),
            prev: RefCell::new(None),
        });

        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut(); // 前一个节点的可变引用

                // 使用 downgrade 方法将一个 Rc<T>类型转换成一个 Weak<T>类型。类似地，可以
                // 使用 upgrade 方法将一个 Weak<T>类型转换成一个 R<T>类型。downgrade 方法将始终有效，
                // 而在弱引用上调用 upgrade 方法时，实际的值可能已经被删除，在这种情况下，你将获得
                // 的值是 None。所以，让我们添加一个指向上一个节点的弱指针
                *prev = Some(Rc::downgrade(&new_node));
            }
            None => {}
        }

        Self {
            head: Some(new_node),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let list = LinkedList::new().append(1).append(2);
    println!("nums:{:?}", list);

    let list2 = LinkedList::new().append("a").append("b");
    println!("list2:{:?}", list2);

    let list_nums = DoubleLinkedList::new().append(1).append(2).append(3);
    println!("list_nums:{:?}", list_nums);
}
