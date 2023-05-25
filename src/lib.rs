use std::{
    fmt, fs,
    io::{BufRead, BufReader, Read, Write,Seek,SeekFrom},
    path, process,
};

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
    pub uidcount: u32,
    pub records: Vec<String>,
}
impl steve {
    pub fn sync(&mut self) {
        let mut ubuf: [u8; 4] = [0; 4];
        let mut file = fs::File::options()
            .read(true)
            .open("rec")
            .pretty_unwrap(Some("Cannot open rec"));
        file.read_exact(&mut ubuf)
            .pretty_unwrap(Some("Cant read form rec. 4 bytes"));
        self.uidcount = u32::from_be_bytes(ubuf);
        // Load every record name into memory (trust me bro it wont be that resource intensive and slow)
        let reader = BufReader::new(file);
        for i in reader.lines() {
            let i: String = i.pretty_unwrap(Some("Shit failed somehow"));
            self.records.insert(self.records.len(), i);
        }
    }
    // Create new record (overwrites existing ones)
    pub fn init_record(&self) {
        let mut file =
            fs::File::create("rec").pretty_unwrap(Some("steve: failed to open rec as rw"));
        // init 0 u32
        file.write_all(&[0; 4])
            .pretty_unwrap(Some("steve: cannot write 4 null bytes to new rec"));
        file.write_all(&[10; 1])
            .pretty_unwrap(Some("steve: cant write LF to new rec"));
    }
    pub fn record_register(&mut self, name: &str) {
        // cock
        for i in self.records.iter() {
            let i = i.split("::").collect::<Vec<&str>>()[0];
            if name == i{
                println!("steve: Cancelling register due to repeating names {{{name}}}");
                return;
            }
        }
        let p = path::Path::new("rec");
        if !p.exists() {
            panic!("steve: cannot record register! rec not found")
        }
        self.records.insert(self.records.len(),format!("{}::{}",name,self.uidcount));
        let mut file = fs::File::options()
            .append(true)
            .read(true)
            .write(true)
            .open("rec")
            .pretty_unwrap(Some("steve: cannot open rec for register"));
        file.write(format!("{}::{}\n", name, self.uidcount).as_bytes())
            .pretty_unwrap(Some("steve: cannot write to rec for register"));
        self.uidcount += 1;
        // wish there was a proper way to MODIFY FUCKING SPECIFIC BYTES
        
    }
}
