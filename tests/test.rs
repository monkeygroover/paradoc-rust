use std::rc::Rc;
// use num::bigint::BigInt;
extern crate paradoc;

fn int(x: i32) -> Rc<paradoc::PdObj> {
    Rc::new(paradoc::PdObj::from(x))
}

fn list(xs: Vec<Rc<paradoc::PdObj>>) -> Rc<paradoc::PdObj> {
    Rc::new(paradoc::PdObj::List(Rc::new(xs)))
}


macro_rules! intvec {
    ($($case:expr),*) => {
        vec![$( int($case), )*];
    }
}

#[test]
fn basic() {
    assert_eq!(paradoc::simple_eval("3 4+"), intvec![7]);
    assert_eq!(paradoc::simple_eval("3:"), intvec![3, 3]);
    assert_eq!(paradoc::simple_eval("11 7%"), intvec![4]);
}

#[test]
fn readme() {
    assert_eq!(paradoc::simple_eval("¹²m"), vec![list(intvec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100])]);
}

#[test]
fn test_list() {
    assert_eq!(paradoc::simple_eval("[3 4]~+"), vec![int(7)]);
}

#[test]
fn map() {
    assert_eq!(paradoc::simple_eval("[3 4])m"), vec![list(intvec![4, 5])]);
    assert_eq!(paradoc::simple_eval("[3 4]{Y+}%"), vec![list(intvec![3, 5])]);
}

#[test]
fn block() {
    assert_eq!(paradoc::simple_eval("3 4{+}~"), vec![int(7)]);
}

#[test]
fn each() {
    assert_eq!(paradoc::simple_eval("[3 4]{:}e"), intvec![3, 3, 4, 4]);
    assert_eq!(paradoc::simple_eval("[3 4]:e"), intvec![3, 3, 4, 4]);
    assert_eq!(paradoc::simple_eval("[3 4]{2*1+}e"), intvec![7, 9]);
}

#[test]
fn stack_manip() {
    assert_eq!(paradoc::simple_eval("3 4:" ), intvec![3, 4, 4]);
    assert_eq!(paradoc::simple_eval("3 4:p"), intvec![3, 4, 3, 4]);
    assert_eq!(paradoc::simple_eval("3 4:a"), intvec![3, 4, 3]);
    assert_eq!(paradoc::simple_eval("3 4\\"), intvec![4, 3]);

    assert_eq!(paradoc::simple_eval("3 4 5\\o"), intvec![4, 5, 3]);
    assert_eq!(paradoc::simple_eval("3 4 5\\i"), intvec![5, 3, 4]);
}

#[test]
fn stack_manip_trailer() {
    assert_eq!(paradoc::simple_eval("3 4 5+u" ), intvec![7, 5]);
    assert_eq!(paradoc::simple_eval("3 4 5+k" ), intvec![3, 4, 5, 9]);
    assert_eq!(paradoc::simple_eval("3 4 5+q" ), intvec![3, 9, 4, 5]);
}
