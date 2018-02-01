// build.rs
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::io;
use std::fs::DirEntry;
extern crate image;
use image::GenericImage;
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    fn is_file_type(e: &fs::DirEntry, ext: &str) -> bool {
        let p = e.path();
        p.is_file() && p.extension().map(|s| s == ext).unwrap_or(false)
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else if is_file_type(&entry, "png") {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    let src = env::current_dir().unwrap().join("src");

    // scan_dir(&src.to_str().unwrap());
    // let paths: Vec<std::fs::DirEntry> = fs::read_dir("./src")
    //     .unwrap()
    //     .filter_map(|e| e.ok())
    //     .collect();
    // panic!("{:?}", paths);
    // fn callback(dir: DirEntry) {
    //     panic!("{:?}", dir);
    // }

    fn callback(dir: &DirEntry) {
        let img = image::open(dir.path()).unwrap().to_rgb();
        let mut path = dir.path();
        path.set_extension("rs");
        let mut file = match fs::File::create(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {:?}", why),
            Ok(file) => file,
        };

        let (x, y) = img.dimensions();
        writeln!(file, "#[allow(dead_code)]");
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        // let first_letter = path.file_name()
        //     .unwrap()
        //     .to_str()
        //     .unwrap()
        //     .chars()
        //     .nth(0)
        //     .unwrap();
        // panic!("{}", first_letter);

        if file_name.starts_with("C") {
            writeln!(file, "pub static {}: [[u8; 4]; {}] = [", file_name, x * y);

            for i in 0..x {
                for j in 0..y {
                    let pixel = img.get_pixel(i, j);
                    write!(file, "[{}, {}, {}, {}], ", pixel[2], pixel[1], pixel[0], 0);
                }
                writeln!(file, "");
            }
            writeln!(file, "];");
        } else if file_name.starts_with("D") {
            writeln!(file, "pub static {}: [u8; {}] = [", file_name, x * y);

            for i in 0..x {
                for j in 0..y {
                    let pixel = img.get_pixel(i, j);
                    write!(file, "{}, ", pixel[0]);
                    // if img.get_pixel(i, j)[3] != 255 {
                    //     // panic!("dimensions {:?}", img.get_pixel(i, j));
                    // }
                }
                writeln!(file, "");
            }
            writeln!(file, "];");
        }

        // panic!("{:?} {:?}", path, img.get_pixel(0, 0));
    }

    // visit_dirs(&src, &callback);

    // f.write_all("{:?}", files);

    // f.write_all(
    //     b"
    //     pub fn message() -> &'static str {
    //         \"Hello, World!\"
    //     }
    // ",
    // ).unwrap();
}
