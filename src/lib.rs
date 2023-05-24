use std::{
    fs,
    io::{prelude::*, BufReader},
    path, time,convert::TryInto,
    collections::HashMap,
};
// Implement types

pub struct MemberBorrow {
    pub borrow_count: u32,
    pub borrow_names: Vec<String>,
    pub borrow_when: time::Instant,
    pub borrow_duration: time::Duration,
}
pub struct Member {
    pub id: u32,
    pub name: String,
    pub Borrow: MemberBorrow,
}
// Bob\00000000\10\cock,balls\2023-05-24 15:30:00.123456789 +0200\10000.0s

pub struct rec {
    pub uidcount: u32,
    pub Member: HashMap<String,Member>,
}

pub fn init() -> rec {
    let path = path::Path::new("rec");
    let mut nit = rec {
        uidcount: 0,
        Member: HashMap::new(),
    };
    if !path.exists() {
        println!("rec.rec not found in current running path!\ncalrecg rec.init()");
        nit.new();
        return nit;
    }
    // loop over rec.rec
    let file = fs::File::open("rec").unwrap();
    let reader1 = BufReader::new(&file);
    nit.uidcount = u32::from_be_bytes(reader1.bytes().take(4).map(|b| b.unwrap()).collect::<Vec<u8>>().try_into().unwrap()); // holy crap
    let reader2 = BufReader::new(file);
    println!("Members count at {}",nit.uidcount);
    for i in reader2.lines().into_iter() {
        let i = i.unwrap();
        let k = i.split('\\');
        
    }
    return nit;
}

impl rec {
    pub fn new(&mut self) {
        // Make file rec
        let mut file = fs::File::create("rec").expect("Failed to create file -> rec.init()");
        // init 0 ucount
        file.write(&[0; 8]).unwrap();
        // line feed
        file.write(&[10; 1]).unwrap();
        self.uidcount = 0
    }
    pub fn register(&mut self,name: &str,) {
        if self.Members.contains_key(name){ // Might be hash conflict. But who tf cares!
            println!("Member already exists. Please remove this member or modify their data!")
        }
    }
}
