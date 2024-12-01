use std::io::Write;

pub struct Viewer {}

impl Viewer {
    pub fn dump_hex(bytes: Vec<u8>, line_size: usize) {
        let mut i = 0;
        let mut buffer = Vec::with_capacity(bytes.len());
        
        let mut last_is_zero = true;
        while i < bytes.len() {
            let chunk_size = (i + line_size).min(bytes.len()) - i;
    
            if bytes[i..i + chunk_size].iter().any(|&byte| byte != 0x00) {
                write!(buffer, "{:#08X}:", i).unwrap();
                
                for &byte in &bytes[i..i + chunk_size] {
                    write!(buffer, " {:#04X}", byte).unwrap();
                }
    
                writeln!(buffer).unwrap();
                
                last_is_zero = false;
            } else {
                if last_is_zero == false {
                    writeln!(buffer, "*").unwrap();
                }
    
                last_is_zero = true;
            }
    
            
            i += line_size;
        }
    
        std::io::stdout().write_all(&buffer).unwrap();
    }
    
    pub fn dump_hex_a(bytes: Vec<u8>, line_size: usize) {
        let mut i = 0;
        let mut buffer = Vec::with_capacity(bytes.len());
        
        let mut last_is_zero = true;
        while i < bytes.len() {
            let chunk_size = (i + line_size).min(bytes.len()) - i;
    
            if bytes[i..i + chunk_size].iter().any(|&byte| byte != 0x00) {
                write!(buffer, "{:#08X}:", i).unwrap();
                
                let mut bytesi = 0;
                for &byte in &bytes[i..i + chunk_size] {
                    write!(buffer, " {:#04X}", byte).unwrap();
                    bytesi += 1;
                }
    
                write!(buffer, "    ").unwrap();
                let mut result = line_size - bytesi;
                while result > 0 {
                    write!(buffer, "     ").unwrap();
                    result -= 1;
                }
    
                write!(buffer, "|").unwrap();
                for &byte in &bytes[i..i + chunk_size] {
                    if byte.is_ascii_alphabetic() {
                        write!(buffer, " {} ", byte as char).unwrap();
                    } else {
                        write!(buffer, " . ").unwrap();
                    }
                }
                write!(buffer, "|").unwrap();
    
                writeln!(buffer).unwrap();
                
                last_is_zero = false;
            } else {
                if last_is_zero == false {
                    writeln!(buffer, "*").unwrap();
                }
    
                last_is_zero = true;
            }
    
            
            i += line_size;
        }
    
        std::io::stdout().write_all(&buffer).unwrap();
    }
    
    pub fn dump_binary(bytes: Vec<u8>, line_size: usize) {
        let mut i = 0;
        let mut buffer = Vec::with_capacity(bytes.len());
        
        let mut last_is_zero = true;
        while i < bytes.len() {
            let chunk_size = (i + line_size).min(bytes.len()) - i;
    
            if bytes[i..i + chunk_size].iter().any(|&byte| byte != 0x00) {
                write!(buffer, "{:#010b}:", i).unwrap();
                
                for &byte in &bytes[i..i + chunk_size] {
                    write!(buffer, " {:#010b}", byte).unwrap();
                }
    
                writeln!(buffer).unwrap();
                
                last_is_zero = false;
            } else {
                if last_is_zero == false {
                    writeln!(buffer, "*").unwrap();
                }
    
                last_is_zero = true;
            }
    
            
            i += line_size;
        }
    
        std::io::stdout().write_all(&buffer).unwrap();
    }

    pub fn dump_binary_a(bytes: Vec<u8>, line_size: usize) {
        let mut i = 0;
        let mut buffer = Vec::with_capacity(bytes.len());
        
        let mut last_is_zero = true;
        while i < bytes.len() {
            let chunk_size = (i + line_size).min(bytes.len()) - i;
    
            if bytes[i..i + chunk_size].iter().any(|&byte| byte != 0x00) {
                write!(buffer, "{:010b}:", i).unwrap();
                
                let mut bytesi = 0;
                for &byte in &bytes[i..i + chunk_size] {
                    write!(buffer, " {:010b}", byte).unwrap();
                    bytesi += 1;
                }

                write!(buffer, "    ").unwrap();
                let mut result = line_size - bytesi;
                while result > 0 {
                    write!(buffer, "          ").unwrap();
                    result -= 1;
                }

                write!(buffer, "|").unwrap();
                for &byte in &bytes[i..i + chunk_size] {
                    if byte.is_ascii_alphabetic() {
                        write!(buffer, " {} ", byte as char).unwrap();
                    } else {
                        write!(buffer, " . ").unwrap();
                    }
                }
                write!(buffer, "|").unwrap();
    
                writeln!(buffer).unwrap();
                
                last_is_zero = false;
            } else {
                if last_is_zero == false {
                    writeln!(buffer, "*").unwrap();
                }
    
                last_is_zero = true;
            }
    
            
            i += line_size;
        }
    
        std::io::stdout().write_all(&buffer).unwrap();
    }
    
}