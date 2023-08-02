//borrow的例子
fn say_hello(str:&str){
    println!("{}",str);
}

//borrow进阶：asref泛型中用
fn say_hello_<T:AsRef<str>>(str:T){
    println!("{}",str.as_ref());
}

trait B{
  fn func_in_b(&self);
}

// Trait A继承Trait B
trait A: B{
  fn func_in_a(&self);
}

struct C{}
// C实现Trait A
impl A for C {
  fn func_in_a(&self){
    println!("impl: func_in_a");
  }
}
// C还要实现Trait B
impl B for C {
  fn func_in_b(&self){
    println!("impl: func_in_b");
  }
}

fn main(){
    let words = String::from("Hello world!");
    say_hello(&words);
    say_hello_(&words);
    
    let c = C{};
    c.func_in_b();
    c.func_in_a();
}
