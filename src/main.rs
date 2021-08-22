use std::io::{Read, Write};

const BUFFER_SIZE: usize = 128 * 1024 * 1024;

fn encrypt_file(i_filename: &str, o_filename: &str, password: &str, buffer: &mut Vec<u8>) {
    let mut input_file = std::fs::OpenOptions::new().read(true).open(&i_filename).unwrap();
    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(o_filename)
        .unwrap();
    let mut length = input_file.metadata()
        .unwrap()
        .len() as usize;
    while length > 0 {
        length -= if length > BUFFER_SIZE {
            BUFFER_SIZE
        } else {
            buffer.resize(length, 0);
            length
        };
        input_file.read(buffer).unwrap();
        encrypt(buffer, password);
        output_file.write(buffer).unwrap();
    }
}


fn encode(start_dir: &str, output_dir: &str, password: &str) {
    let mut output_file_path;
    let mut buffer= Vec::new();
    let mut files = Vec::new();
    get_files(start_dir, &mut files);
    for (path, filename) in files {
        output_file_path = path.replacen(start_dir, output_dir, 1);
        std::fs::create_dir_all(output_file_path.replacen(&filename, "", 1))
            .unwrap();
        encrypt_file(&path, &output_file_path, password, &mut buffer);
    }
}


fn get_files(start_dir: &str, output: &mut Vec<(String, String)>){
    let mut ui;
    let mut md= std::fs::metadata(start_dir).unwrap();
    if md.is_file() {
        output.push((start_dir.to_string(), String::with_capacity(0)));
        return;
    }
    for i in std::fs::read_dir(start_dir).unwrap() {
        ui = i.unwrap();
        md = ui.metadata().unwrap();
        if md.is_file() {
            if let Some(filename) = ui.file_name().to_str() {
                output.push((ui.path().to_str().unwrap().to_string(), filename.to_string()));
            }
        } else {
            get_files(ui.path().to_str().unwrap(), output);
        }
    }
}


fn encrypt(vector: &mut Vec<u8>, password: &str) {
    let password_len = password.len();
    if password_len == 0 {
        return;
    }
    let mut index;
    for i in 0..vector.len() {
        index = i % password_len;
        vector[i] ^= password[index..index + 1].chars().next().unwrap() as u8;
    }
}


fn main() {
    let flag;
    let args:Vec<String> = std::env::args().collect();
    let sdir = args.get(1).expect("input directory not specified").trim_end_matches('/');
    let odir = match args.get(3) {
        Some(x) => {flag = false; x.trim_end_matches('/').to_string()},
        None => {flag = true; format!("{}/xorencryptiontempfile", std::env::temp_dir().to_str().unwrap())}
    };
    assert_ne!(sdir, odir);
    let password = args.get(2).expect("password not specified");

    let md = std::fs::metadata(sdir).unwrap();
    if md.is_file() {
        let mut buffer = Vec::new();
        if flag {
            encrypt_file(sdir, &odir, password, &mut buffer);
            std::fs::remove_file(sdir).unwrap();
            encrypt_file(&odir, sdir, "", &mut buffer);
            std::fs::remove_file(odir).unwrap();
        } else {
            encrypt_file(sdir, &odir, password, &mut buffer);
        }
    } else if flag {
        encode(&sdir, &odir, password);
        std::fs::remove_dir_all(sdir).unwrap();
        encode(&odir, &sdir, "");
        std::fs::remove_dir_all(odir).unwrap();
    } else {
        encode(&sdir, &odir, password);
    }
}
