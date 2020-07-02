use std::io::{Read, Write};

const BUFFER_SIZE:u64 = 128 * 1024 * 1024;


fn encode(start_dir: &str, output_dir: &str, password: &str) {
    for (filename, dir) in get_files(start_dir) {
        let mut file = std::fs::File::open(&filename).unwrap();
        let _ = std::fs::create_dir_all(filename
            .replacen(start_dir, output_dir, 1)
            .replacen(&dir, "", 1));
        let _ = std::fs::File::create(filename
            .replacen(start_dir, output_dir, 1));
        let mut output_file = std::fs::OpenOptions::new()
            .append(true)
            .open(filename
                .replacen(start_dir, output_dir, 1))
            .unwrap();
        let total_len = file.metadata()
            .unwrap()
            .len();
        let mut length = total_len;
        loop {
            let clen = if length < BUFFER_SIZE { length } else { BUFFER_SIZE };
            if clen == 0 {
                break;
            }
            length -= clen;
            let mut buffer = vec![0u8; clen as usize];
            let _ = file.read(&mut buffer);
            let encrypted_data = encrypt(buffer, password);
            let _ = output_file.write(&encrypted_data);
        }
    }
}


fn get_files(start_dir: &str) -> Vec<(String, String)> {
    let mut output:Vec<(String, String)> = Vec::new();
    for i in std::fs::read_dir(start_dir).unwrap() {
        let ui = i.unwrap();
        let md = match ui.metadata() {
            Ok(x) => x,
            Err(_) => continue
        };
        if md.is_file() {
            if let Some(filename) = ui.file_name().to_str() {
                output.push((ui.path().display().to_string(), filename.to_string()));
            }
        } else {
            for j in get_files(&ui.path().display().to_string()) {
                output.push(j);
            }
        }
    }
    output
}


fn encrypt(vector: Vec<u8>, password: &str) -> Vec<u8> {
    if password.len() == 0 {
        return vector;
    }
    let mut output = Vec::new();
    for (i, j) in vector.iter().enumerate() {
        let index = i % password.len();
        let letter = match password[index..index + 1].chars().next() {
            Some(x) => x as u8,
            None => panic!()
        };
        output.push(j ^ letter as u8);
    }
    output
}


fn main() {
    let args:Vec<String> = std::env::args().collect();
    let sdir = args.get(1).expect("starting directory not specified");
    let odir = args.get(2).expect("output directory not specified");
    if sdir == odir {
        println!("cannon use the same directory");
        panic!();
    }
    let pw = args.get(3).expect("password not specified");
    let del_str = match args.get(4) {
        Some(x)  => x as &str,
        _ => ""
    };
    let overwrite = if del_str.contains("-o") {true} else {false};
    let del = if del_str.contains("-d") {true} else {false};
    encode(&sdir, &odir, &pw);
    if del {
        let _ = std::fs::remove_dir_all(sdir);
    } else if overwrite {
        let _ = std::fs::remove_dir_all(sdir);
        encode(&odir, &sdir, "");
        let _ = std::fs::remove_dir_all(odir);
    }
}
