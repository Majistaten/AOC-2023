#![allow(dead_code)]
#![allow(unused_variables)]
use crate::day1_1::is_digit;
use crate::helper::Data;

fn validate_cell(pos:(i32,i32), xlimit:i32, ylimit:i32) -> Option<(i32,i32)> {
    if pos.0 >= 0 && pos.1 >= 0 && pos.0 < xlimit as i32 && pos.1 < ylimit as i32 {
         return Some(pos);
     }
    None
}

fn neighbours(pos: (i32, i32), xlimit: i32, ylimit:i32) -> Vec<(i32, i32)> {
    let mut nb: Vec<(i32, i32)> = Vec::new();
    nb.push((pos.0 - 1, pos.1 - 1));
    nb.push((pos.0 - 1, pos.1));
    nb.push((pos.0 - 1, pos.1 + 1));

    nb.push((pos.0 + 1, pos.1 - 1));
    nb.push((pos.0 + 1, pos.1));
    nb.push((pos.0 + 1, pos.1 + 1));

    nb.push((pos.0, pos.1 - 1));
    nb.push((pos.0, pos.1 + 1));

    nb.into_iter()
        .filter(|dolk| validate_cell(*dolk, xlimit, ylimit).is_some())
        .collect()
}

    fn who_dis(data:&Data) -> String {
        let mut symbols:String = String::from("");
        data.input.lines().for_each(|line|{
            line.chars().enumerate().for_each(|c|{
                if is_digit(c.1) || c.1 == '.' {

                }
                else {
                    if !symbols.contains(c.1){
                        symbols = format!("{}{}",symbols,c.1);
                    }
                }
            })
        });
        symbols
    }

pub fn start(){

    let data : Data = Data::new(3,1);
    let symbols = who_dis(&data);
    //let symbols = String::from("*#+$");
    let mut sum:u32 = 0;
    let mut line_num = 0;
    let  lines:Vec<&str> = data.input.lines().collect();
    let lines_count = lines.len() as i32;

    let mut has_pn = false;
    let mut word:String = String::from("");

    data.input.lines().for_each(|line|{
        line.chars().enumerate().for_each(|ch|{
            let c = ch.1;
            let i = ch.0;

            let next = line.chars().nth(i + 1);
            let prev = if i > 0 {
                Some(line.chars().nth(i - 1).unwrap())
            } else {
                None
            };

            ///////////////////////
            if is_digit(c) {
                if prev.is_some(){
                    if !is_digit(prev.unwrap()){
                        word = String::from("");
                        word = format!("{}{}", word, c);
                        has_pn = false;
                    }
                    else if is_digit(prev.unwrap()){
                        word = format!("{}{}", word, c);
                    }
                }
                else if prev.is_none(){
                    word = String::from("");
                    word = format!("{}{}", word, c);
                    has_pn = false;

                }
                // check neighbours
                let coord = (i as i32,line_num);
                let neighbours:Vec<(i32,i32)>  = neighbours(coord,line.len() as i32,lines_count);
                print!("Neighbours: [{}]",neighbours.len());
                neighbours.iter().for_each(|cell|{
                    let ln:&str = lines.get(cell.1 as usize).unwrap();
                    let val:char = ln.chars().enumerate().nth(cell.0 as usize).unwrap().1;
                    if symbols.contains(val){
                        has_pn = true;
                    }
                    print!("{}",val);

                });
                println!();

                let mut calc = false;
                if next.is_some() {
                    if !is_digit(next.unwrap()) {
                        calc = true;
                    }
                }
                else if next.is_none(){
                    calc = true;
                }
                if calc{
                    // has_pn = hasPart;
                    let v = word.parse::<u32>().unwrap();
                    if has_pn {
                        sum += v;
                    }
                    println!("[{}] {}", v, has_pn);
                    println!();
                }
            }
            else{
                word = String::from("");
                has_pn = false;
            }
        });
        line_num += 1;
    });
    println!("SYMBOLS:[{}]",symbols);
    println!("SUM:{}",sum);
}