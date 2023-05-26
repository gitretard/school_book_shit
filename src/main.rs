mod lib;
fn main(){
    let mut steve = lib::steve{midcount:0,records:Vec::new()};
    let path = std::path::Path::new("rec");
    if !path.exists(){
        steve.init_record()
    }
    steve.sync();
    steve.record_register("ben");
}