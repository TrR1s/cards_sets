use std::cell::RefCell;
// use std::process::Child;
use std::rc::{ Rc, Weak };
use std::fmt::{self, Debug, Display};
type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> 
where 
T:Display
{
    val: T,
    parent: Option<NodeRef<T>>,
    children: Vec<NodeRef<T>>,
}
impl <T:Display> fmt::Display for  Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    where {
        // Используем `self.номер`, чтобы получить доступ к каждому полю структуры.

        let parent_str:String  = match &self.parent{
            Some(parent) => format!("{}",Rc::clone(&parent).borrow_mut().val),
            _ => "None".to_string(),

        };


        let val_str = format!("{}",self.val);
        // let par_str = self.parent;


        write!(f, "(val_str:{val_str} parent_str: {parent_str})" )
    }
}

impl<T:Display> Node<T> {
    // type T;
    pub fn new(val: T) -> NodeRef<T> {
        Rc::new(
            RefCell::new(Node {
                val: val,
                parent: Option::None,
                children: vec![],
            })
        )
    }
    pub fn do_child(parent_node: NodeRef<T>, child_val: T) -> NodeRef<T> {
        let node_child = Node::new(child_val);
        // Some(Rc::clone(&node_child));
        Rc::clone(&node_child).borrow_mut().parent = Some(Rc::clone(&parent_node));

        Rc::clone(&parent_node).borrow_mut().children.push(Rc::clone(&node_child));
        return Rc::clone(&node_child);
    }
}

#[cfg(test)]
#[test]
fn check_node() {
    use std::borrow::Borrow;

    let leaf_refcell: RefCell<Node<i32>> = RefCell::new(Node {
        val: 3,
        parent: Option::None,
        children: vec![],
    });

    let leaf_rc: Rc<RefCell<Node<i32>>> = Rc::new(leaf_refcell);

    let leaf_refcell2: RefCell<Node<i32>> = RefCell::new(Node {
        val: 4,
        parent: Option::None,
        children: vec![],
    });

    let leaf_rc2: Rc<RefCell<Node<i32>>> = Rc::new(leaf_refcell2);

    let mut branch = Node {
        val: 105,
        parent: None,
        children: vec![Rc::clone(&leaf_rc)],
    };
    let branch_rcref = Rc::new(RefCell::new(branch));

    // Rc::clone(&leaf_rc).borrow_mut().parent = Some(Rc::clone(&branch_rcref));
    // let branch_rcref= Rc::new(RefCell::new(Node {
    //     val: 120,
    //     parent: None,
    //     children: Some(Rc::clone(&leaf_rc)),
    // }));

    // println!("leaf parent = {:?}", leaf.borrow().val);

    // let mut val_mut = leaf.borrow_mut();
    // val_mut.val = 7;
    // drop(val_mut);

    // println!("leaf val = {:?}", branch.children.unwrap().borrow_mut().val);
    Rc::clone(&branch_rcref).borrow_mut().children.push(Rc::clone(&leaf_rc2));
    println!("leaf val = {:?}", Rc::clone(&branch_rcref).borrow_mut().children);
    println!("leaf parent = {:?}", leaf_rc.borrow_mut().val);
    // println!("leaf value = {:?}", leaf.val);
    // println!("leaf children = {:?}", leaf.children);
    Rc::clone(&leaf_rc2).borrow_mut().parent = Some(Rc::clone(&branch_rcref));
    println!("leaf_rc2 parent = {:?}", leaf_rc2.borrow_mut().parent);
    // println!("leaf val = {:?}", leaf.borrow().val);
    // println!("branch children = {:?}", branch.borrow().children);
}

#[test]
fn check_node_new() {
    #[derive(Debug)]
    struct Layout {
        deck: i16,
        game: String,
    }
    impl fmt::Display for  Layout {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        where {
            let deck_str: String=format!("{}",self.deck);
            let game_str: String=format!("{}",self.game);
            write!(f, "(deck:{deck_str}  game: {game_str})" )
        
    }
    }


    let layout = Layout { deck: 16, game: "RP".to_string() };
    let  new_node = Node::new(layout);
    println!(" new_node {}", new_node.clone().borrow());
    let layout_child = Layout { deck: 18, game: "RP shild".to_string() };

    let child =Node::do_child(Rc::clone(&new_node), layout_child);
    // let child =Node::do_child(Rc::clone(&new_node), layout_child);
    
    let child_1 = Rc::clone(&new_node);
    let rc_child = Rc::clone(&(child_1.borrow().children[0]));
    println!(" child {:?}", child.borrow().val);
    child.borrow_mut().val.deck = 988;
    println!(" child {:?}", rc_child.borrow().val);
    println!(" rc_child.borrow {}", rc_child.borrow());
//     println!(" rc_child {:?}", rc_child);
}
