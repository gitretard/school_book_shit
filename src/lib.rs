use std::{
    fmt, fs,
    io::{BufRead, BufReader, Read, Write},
    path, process, time
};

pub struct borrow<>{
    borrow_when: time::Instant,
    return_after: time::Duration,
    borrow_names: Vec<u32>,
}
pub struct member{
    pub name: String,
    pub mid: u32,
    pub borrow: Option<borrow>,
}

pub trait pretty_unwrap<T, E> {
    fn pretty_unwrap(self, msg: Option<&str>) -> T;
    fn pretty_unwrap_or_else(self, val: T, msg: Option<&str>) -> T;
}
// I fucking love generics
impl<T, E> pretty_unwrap<T, E> for Result<T, E>
where
    E: fmt::Display,
{
    fn pretty_unwrap(self, msg: Option<&str>) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                // Handle the error
                if let Some(msg) = msg {
                    println!("\x1b[31m{} <- {}\x1b[m", msg, err);
                    process::exit(1);
                } else {
                    println!("\x1b[31m{}\x1b[m", err);
                    process::exit(1);
                }
            }
        }
    }
    fn pretty_unwrap_or_else(self, val: T, msg: Option<&str>) -> T {
        match self {
            Ok(v) => v,
            Err(err) => {
                if let Some(msg) = msg {
                    println!("\x1b[31m {} <- {}\x1b[m", msg, err);
                    val
                } else {
                    println!("\x1b[31m{}\x1b[m", err);
                    val
                }
            }
        }
    }
}
// This is Steve. He manages the record file and do stuff
pub struct steve {
    pub midcount: u32,
    pub records: Vec<member>,
}
impl steve {
    pub fn sync(&mut self) {
        let recfile = fs::File::options()
            .read(true)
            .open("rec")
            .pretty_unwrap(Some("Cannot open rec"));
        let mut midcf = fs::File::open("midcf").unwrap();
        let mut midcbuf:[u8;4] = [0;4];
        midcf.read_exact(&mut midcbuf).pretty_unwrap(Some("cant read 4 bytes from midcf"));
        // Load every record name into memory (trust me bro it wont be that resource intensive and slow)
        let reader = BufReader::new(recfile);
        let mut tmpmcount:u32 = 0;
        for i in reader.lines() {
            let i = i.pretty_unwrap(Some("rj809j9wg9j0"));
            let i:Vec<&str> = i.split("::").collect();
            let s:member = member { name: i[0].to_string(), mid:tmpmcount, borrow:None };
            self.records.insert(self.records.len(),s);
            tmpmcount+=1
        }
        self.midcount = tmpmcount
    }
    // Create new record (overwrites existing ones)
    pub fn init_record(&self) {
        fs::File::create("rec").pretty_unwrap(Some("steve: failed to open rec as rw"));
        // init 0 u32
        let mut midcf = fs::File::create("midcf").pretty_unwrap(Some("Failed to create midc"));
        midcf.write(&[0;4]).unwrap();
    }
    pub fn record_register(&mut self, name: &str) {
        // cock
        for i in self.records.iter() {
            let i = i.name.split("::").collect::<Vec<&str>>()[0];
            if name == i{
                println!("steve: Cancelling register due to repeating names {{{name}}}");
                return;
            }
        }
        let p = path::Path::new("rec");
        if !p.exists() {
            panic!("steve: cannot record register! rec not found")
        }
        self.records.insert(self.records.len(),member{name: format!("{}::{}",name,self.midcount),mid:self.midcount,borrow:None});
        let mut file = fs::File::options()
            .append(true)
            .read(true)
            .write(true)
            .open("rec")
            .pretty_unwrap(Some("steve: cannot open rec for register"));
        file.write(format!("{}::{}\n", name, self.midcount).as_bytes())
            .pretty_unwrap(Some("steve: cannot write to rec for register"));
        let mut midcf = fs::File::options().write(true).open("midcf").pretty_unwrap(Some("Failed to open midcf as write-only"));
        self.midcount += 1;
        midcf.write(&self.midcount.to_be_bytes()).pretty_unwrap(Some("failed to write to midcf record_register"));
        
    }
}
