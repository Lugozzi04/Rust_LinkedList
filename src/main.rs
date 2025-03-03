#[derive(Debug)]
struct Node{
    value:i32,
    next:Option<Box<Node>>
}
#[derive(Debug)]
struct LinkedList{
    head:Option<Box<Node>>
}
trait List {
    fn new()->LinkedList;
    fn add(&mut self,value:i32);
    fn remove(&mut self,value:i32)->bool;
    fn print(&self);
    fn len(&self)->usize;
}
impl List for LinkedList {
    fn new()->LinkedList {
        LinkedList{
            head:None,
        }
    }

    fn add(&mut self, value: i32) {
        
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        
    }

    fn remove(&mut self, value: i32) -> bool {
        unsafe {
            let mut current: *mut Option<Box<Node>> = &mut self.head;
            
            while let Some(node) = &mut *current {
                if node.value == value {
                    let next = node.next.take();
                    *current = next;
                    return true;
                }
                current = &mut (*current).as_mut().unwrap().next;
            }
        }
        false
    }
        

    fn print(&self) {
        let mut cur=&self.head;
        loop{
            match cur{
                Some(node)=>{
                    print!("{} ",node.value);
                    cur = &node.next;
                }
                None=>break,
            }
        }
    }

    fn len(&self)->usize {
        let mut size:usize = 0;
        let mut cur = &self.head;
        while let Some(node)=cur{
            size+=1;
            cur = &node.next;
        }
        size
    }
}
fn main() {
    let mut list = LinkedList::new();
    list.add(10);
    list.add(20);
    list.add(30);
    list.remove(20);
    list.print();
    println!("size: {}",list.len());
}
