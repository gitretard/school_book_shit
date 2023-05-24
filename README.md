borrow checking
member id (u32 0..?)
book id
I FUCKING HATE .MD FILES AAAAAAAAAAAAAAAAAAAAA

keep track of time:
'''

use std::fs;
use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let file_path = "last_run.txt";

    // Read the last run timestamp from the file
    let last_run_timestamp = fs::read_to_string(file_path)
        .unwrap_or_else(|_| "0".to_string())
        .parse::<u64>()
        .unwrap_or(0);

    // Calculate the elapsed time since the last run
    let now = Instant::now();
    let elapsed = now.duration_since(Instant::now() - Duration::from_secs(last_run_timestamp));

    println!("Time elapsed since the last run: {:?}", elapsed);

    // Update the last run timestamp in the file
    let mut file = fs::File::create(file_path).expect("Unable to create file");
    file.write_all(now.elapsed().as_secs().to_string().as_bytes())
        .expect("Unable to write data");

    // Sleep for demonstration purposes
    sleep(Duration::from_secs(5));
}

'''



'''

member fields{
 name str
 id u32
 borrowwed book count u32
 borrowed books id str. split with ','
 when borrowed (0) if not borrowing any
 
}

'''

'''
book fields{
 name str
 id u32
}
'''

on every loop look for any members that should be already returnign their borrowed books

read the whole thing into memory,edit,write
'''

use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";
    let search_string = "foo";
    let new_content = "bar";

    // Read the contents of the file into a string
    let mut contents = fs::read_to_string(file_path)?;

    // Split the string into lines and iterate over them
    for line in contents.lines_mut() {
        // Check if the current line contains the search string
        if line.contains(search_string) {
            // Replace the current line with the new content
            *line = new_content;
        }
    }

    // Write the updated contents back to the file
    let mut file = fs::File::create(file_path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

'''

wtf is this shit
'''


use memmap::MmapOptions;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";
    let search_string = "foo";
    let new_content = "bar";

    // Open the file in read-write mode and create a memory map
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)?;
    let mut mmap = unsafe { MmapOptions::new().map_mut(&file)? };

    // Find the line to edit by searching for the search string
    let line_idx = mmap
        .windows(search_string.len())
        .position(|window| window == search_string.as_bytes())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "String not found"))?;

    // Replace the line with the new content
    let line_start = mmap[..line_idx].iter().rposition(|&c| c == b'\n').map(|idx| idx + 1).unwrap_or(0);
    let line_end = mmap[line_idx..].iter().position(|&c| c == b'\n').map(|idx| line_idx + idx).unwrap_or(mmap.len());
    mmap[line_start..line_end].copy_from_slice(new_content.as_bytes());

    Ok(())
}


'''
