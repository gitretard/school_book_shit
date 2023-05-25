#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod lib;
fn main(){
    let mut steve = lib::steve{uidcount:0,records:Vec::new()};
    steve.init_record();
    steve.sync();
    steve.record_register("bob");
    steve.record_register("mike");
}