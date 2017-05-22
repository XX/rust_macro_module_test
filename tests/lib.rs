#[macro_use]
extern crate lazy_static;

mod my_module;

#[test]
fn test()
{
    my_module::print();
}
