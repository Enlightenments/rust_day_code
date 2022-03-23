use std::rc::Rc;

pub fn main(){
    println!("2022-03-21!");
    let s1 = "hello world";
    let mut s2 = String::from("hello world");
    {
        let s3 = String::from("hello s3");
        s2 = s3;
        // println!("{}",s3); error
    }
    println!("{}",s2);

    let s = bigstr("ab","ac");
    println!("{}",s);

    let list = List::Cons(0,Box::new(List::Cons(1,Box::new(List::Cons(2,Box::new(List::Nil))))));
    println!("list:{:?}",list);

    let l4 = Rc::new(RcList::Cons(4,Rc::new(RcList::Nil)));
    let rc_list1 = RcList::Cons(0,Rc::new(RcList::Cons(1,l4.clone())));
    let rc_list2 = RcList::Cons(1,Rc::new(RcList::Cons(2,l4)));
    println!("rclist:{:?},{:?}",rc_list1,rc_list2);
}

fn bigstr<'a>(s1:&'a str,s2:&'a str)->&'a str{
    let s = if s1>s2 {s1}else{s2};
    s
}

struct Person<'a>{
    name:&'a str,
}

#[derive(Debug)]
enum List{
    Cons(i32,Box<List>),
    Nil,
}

#[derive(Debug)]
enum RcList{
    Cons(i32,Rc<RcList>),
    Nil,
}