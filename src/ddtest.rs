
trait TScene {
    fn Do(&self);
}

struct TMenu1 {
}

impl TScene for TMenu1 {
    fn Do(&self) {
        println!("1 do");
    }
}
struct TMenu2 {
}

impl TScene for TMenu2 {
    fn Do(&self) {
        println!("2 do");
    }
}

/*
 Options: 
  - trait objects
  - enum for storage, monomorphization for dispatch, bit of boilerplate\
  - scene is just function pointers, explicit dynamic dispatch, oh but whatever state it wants to have as well
    ok you can measure the vtable for me thanks
 */

#[test]
fn test_dd() {

    let mut v:Vec<Box<dyn TScene>> = Vec::new();
    v.push(Box::new(TMenu1{}));
    v[v.len()-1].Do();
    v.push(Box::new(TMenu2{}));
    v[v.len()-1].Do();

}