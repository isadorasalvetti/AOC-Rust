use std::{collections::HashMap};

const TRANSMISSION: &str = "C20D59802D2B0B6713C6B4D1600ACE7E3C179BFE391E546CC017F004A4F513C9D973A1B2F32C3004E6F9546D005840188C51DA298803F1863C42160068E5E37759BC4908C0109E76B00425E2C530DE40233CA9DE8022200EC618B10DC001098EF0A63910010D3843350C6D9A252805D2D7D7BAE1257FD95A6E928214B66DBE691E0E9005F7C00BC4BD22D733B0399979DA7E34A6850802809A1F9C4A947B91579C063005B001CF95B77504896A884F73D7EBB900641400E7CDFD56573E941E67EABC600B4C014C829802D400BCC9FA3A339B1C9A671005E35477200A0A551E8015591F93C8FC9E4D188018692429B0F930630070401B8A90663100021313E1C47900042A2B46C840600A580213681368726DEA008CEDAD8DD5A6181801460070801CE0068014602005A011ECA0069801C200718010C0302300AA2C02538007E2C01A100052AC00F210026AC0041492F4ADEFEF7337AAF2003AB360B23B3398F009005113B25FD004E5A32369C068C72B0C8AA804F0AE7E36519F6296D76509DE70D8C2801134F84015560034931C8044C7201F02A2A180258010D4D4E347D92AF6B35B93E6B9D7D0013B4C01D8611960E9803F0FA2145320043608C4284C4016CE802F2988D8725311B0D443700AA7A9A399EFD33CD5082484272BC9E67C984CF639A4D600BDE79EA462B5372871166AB33E001682557E5B74A0C49E25AACE76D074E7C5A6FD5CE697DC195C01993DCFC1D2A032BAA5C84C012B004C001098FD1FE2D00021B0821A45397350007F66F021291E8E4B89C118FE40180F802935CC12CD730492D5E2B180250F7401791B18CCFBBCD818007CB08A664C7373CEEF9FD05A73B98D7892402405802E000854788B91BC0010A861092124C2198023C0198880371222FC3E100662B45B8DB236C0F080172DD1C300820BCD1F4C24C8AAB0015F33D280";
const TEST: &str = "C200B40A82";

use std::panic;

fn main() {
    let HEXCODE: HashMap<char, &str> = HashMap::from([('0', "0000"), ('1', "0001"), ('2', "0010"), ('3', "0011"), ('4', "0100"), ('5', "0101"), ('6', "0110"), ('7', "0111"), 
    ('8', "1000"), ('9', "1001"), ('A', "1010"), ('B', "1011"), ('C', "1100"), ('D', "1101"), ('E', "1110"), ('F', "1111"),]);

    let mut code= String::new();
    for char in TRANSMISSION.chars() {
        if !HEXCODE.contains_key(&char) { code.push(char); }
        else { code.push_str(HEXCODE[&char]); }
    }

    let mut scan = 0;
    let mut version_acc = 0;

    //println!("totl code len: {}", code.len());
    let result = decode_pkg(&mut scan, &code, &mut version_acc);
    println!("Part1: {}", version_acc);
    println!("Part2: {}", &result);
}

fn decode_pkg(scan: &mut usize, code: &str, version_acc: &mut i32) -> u64 {
    // Header
    let version = i32::from_str_radix(&code[*scan..*scan+3], 2).unwrap();
    *version_acc += version;

    let id = i32::from_str_radix(&code[*scan+3..*scan+6], 2).unwrap();
    *scan += 6;

    println!("Header! v: {}, id: {}, ", &version, &id);

    if id == 4 {
        return get_literal_value(scan, code);
    }
    else { 
        let literal_values = split_sub_packages(scan, code, version_acc);

        // Operation
        println!("Operating! id: {} - {:?}", &id, &literal_values);
        let result: u64;
        if id == 0 { result = literal_values.iter().sum(); }
        else if id == 1 { 
            if literal_values.contains(&0) { result = 0; }
            else { result = literal_values.iter().product(); }
        }
        else if id == 2 { result = *literal_values.iter().min().unwrap(); }
        else if id == 3 { result = *literal_values.iter().max().unwrap(); }
        else if id == 5 { 
            if literal_values[0] > literal_values[1] { result = 1; } 
            else { result = 0; } 
        }
        else if id == 6 { 
            if literal_values[0] < literal_values[1] { result = 1; } 
            else { result = 0 } 
        }
        else if id == 7 { 
            if literal_values[0] == literal_values[1] { result = 1; } 
            else { result = 0; } 
        }
        else { panic!("Bad ID"); }
        return result;
    }
}

fn get_literal_value(scan: &mut usize, code: &str) -> u64 {
    let mut val = String::new();
    let mut group_id = code.chars().nth(*scan).unwrap();
    val.push_str(&code[*scan+1..*scan+5]);
    *scan += 5;
    while group_id == '1' {
        group_id = code.chars().nth(*scan).unwrap();
        val.push_str(&code[*scan+1..*scan+5]);
        *scan += 5;
    }
    let lit = u64::from_str_radix(&val, 2).unwrap_or_default();
    println!("Lit: {}", &lit);
    return lit;
}

fn split_sub_packages(scan: &mut usize, code: &str, version_acc: &mut i32) -> Vec<u64> {
    let mut ret = Vec::new();
    if code.chars().nth(*scan).unwrap() == '1' {
        let spkg_num = usize::from_str_radix(&code[*scan+1..*scan+12], 2).unwrap();
        *scan += 12;
        println!("1: scan: {}, spkgs: {}", &scan, &spkg_num);
        let mut pkg_scan = 0;
        for _ in 0..spkg_num { 
            ret.push(decode_pkg(&mut pkg_scan, &code[*scan..], version_acc));
        }
        *scan += pkg_scan
    }
    else {
        let spkgs_len = usize::from_str_radix(&code[*scan+1..*scan+16], 2).unwrap();
        *scan += 16;
        println!("0: scan: {}, spkgs: {}", &scan, &spkgs_len);
        let mut pkg_scan = 0;
        while pkg_scan < spkgs_len { 
            ret.push(decode_pkg(&mut pkg_scan, &code[*scan..*scan + spkgs_len], version_acc)); 
        }
        *scan += pkg_scan
    }
    return ret;
}