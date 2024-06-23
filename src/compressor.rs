pub fn diff_text(base_str: &str, target_str: &str) -> String {
    let max_length = base_str.len().max(target_str.len());
    let mut diff_str = String::new();
    
    for i in 0..max_length {
        if i >= target_str.len() {
            break;
        } else if i >= base_str.len() {
            diff_str.push_str(&target_str[i..]);
            break;
        } else {
            if base_str.as_bytes()[i] == target_str.as_bytes()[i] {
                diff_str.push(31 as char);
            } else {
                diff_str.push(target_str.as_bytes()[i] as char);
            }
        }
    }
    
    diff_str
}

pub fn replace_at_symbols(input_string: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    
    for char in input_string.chars() {
        if char == 31 as char {
            count += 1;
        } else {
            if count > 0 {
                if count > 2 {
                    result.push(31 as char);
                    result.push_str(&count.to_string());
                    result.push(31 as char);
                } else {
                    for _ in 0..count {
                        result.push(31 as char);
                    }
                }
                count = 0;
            }
            result.push(char);
        }
    }
    
    if count > 0 {
        if count > 2 {
            result.push(31 as char);
            result.push_str(&count.to_string());
            result.push(31 as char);
        } else {
            for _ in 0..count {
                result.push(31 as char);
            }
        }
    }
    
    result
}

pub fn convert_dna_to_bits(dna_sequence: &str) -> Vec<u8> {
    let mut bit_pattern = String::new();
    for base in dna_sequence.chars() {
        match base {
            'A' => bit_pattern.push_str("00"),
            'T' => bit_pattern.push_str("01"),
            'C' => bit_pattern.push_str("10"),
            'G' => bit_pattern.push_str("11"),
            _ => {} 
        }
    }

    
    while bit_pattern.len() % 8 != 0 {
        bit_pattern.push('0');
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut byte: u8 = 0;
    let mut bit_count = 0;

    for bit in bit_pattern.chars() {
        if bit == '1' {
            byte |= 1 << bit_count;
        }
        bit_count += 1;

        if bit_count == 8 {
            bytes.push(byte);
            byte = 0;
            bit_count = 0;
        }
    }

    bytes
}