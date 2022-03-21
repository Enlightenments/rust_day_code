pub fn main(){
    println!("2022-03-21!");
    is_larger(1,2);
    t_is_larger(1,2);
    t_is_larger(1.2,-2.1);
    t_is_larger('a','b');
}

fn is_larger(a:u8,b:u8)->u8{
    let res = if a>=b {a}else{b};
    println!("lager is {}",res);
    res
}
/// This method params need impl trait std::cmp::PartialOrd+std::fmt::Display.
///
/// # Examples
///
/// ```
/// t_is_larger(1.2,-2.1);
/// assert_eq!(1.2, true);
/// ```
fn t_is_larger<T: std::cmp::PartialOrd+std::fmt::Display>(a:T,b:T)->T{
    let res = if a>=b {a}else{b};
    println!("lager is {}",res);
    res
}
