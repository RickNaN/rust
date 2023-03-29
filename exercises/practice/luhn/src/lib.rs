use std::env::args;
use std::io::stdin;

/// Check a Luhn checksum.

pub fn is_format(code: &str) -> bool {
    let mut count = (0,0);
    for i in code.split(" ") {
        count.0 = count.0 + 1;
        for j in i.chars() {
            if j.is_numeric() == false {return false;}
            count.1 = count.1 + 1;
        }
    }
    if count.0 != 4 || count.1 != 16 {return false;}

    return true;
}

pub fn is_compliant(code: &str) -> bool {
    let mut flag = false;
    let tmp = String::from(code).len();
    if tmp == 1 {return false;}

    for j in code.chars() {
        if (flag == false) && j.is_numeric() == false {flag = true; return false;} // primo carattere numerico
        flag = true;
        if j.is_ascii() == false {return false;}
        if j.is_numeric() == false && ((j == ' ')  == false) {return false;}
    }
    return true;
}

pub fn is_valid(code: &str) -> bool {
    let mut tmp = String::new();
    let mut sum = 0;
    let mut count = 0;

    if is_compliant(code) == false {return false}

    for i in code.split(" ") {
        tmp.push_str(i);
    }
    for j in tmp.chars().rev(){
        if (count%2) == 0 { // vanno tenuti come sono
            sum = sum + j.to_digit(32).unwrap();
            //println!("{} {}", sum, count);
        }
        else { // vanno raddoppiati
            let tmp = j.to_digit(32).unwrap();
            if 2*tmp > 9
            { sum = sum + 2*tmp-9;}
            else {sum = sum + 2*tmp;}
        }
        count = count+1;
    }
    sum%10 == 0
}

fn main() {
    let flag = false;
    if flag == false {
        let args: Vec<String> = args().skip(1).collect();
        if args.len() > 0 {
            if is_format(&args[0]) == true
            {
                if is_valid(&args[0]) == true {println!("Valid checksum")}
                else {println!("Invalid checksum")}
            }
            else {println!("Invalid format");}
        }
    }
}