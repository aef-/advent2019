use std::fs;


fn try_noun_verb(noun: usize, verb: usize) -> usize {
    let filename = "src/input";
    let file = fs::read_to_string(filename).unwrap();
    let mut op_codes: Vec<usize> = file.split(",").into_iter().map({|i|
        i.trim().parse::<usize>().unwrap()
    }).collect();
    op_codes[1] = noun;
    op_codes[2] = verb;

    let mut index = 0;
    loop {
        let op_code = op_codes[index];
        let index_to_change = op_codes[index + 3];
        let operand1 = op_codes[index + 1];
        let operand2 = op_codes[index + 2];

        match op_code {
            1 => {
                op_codes[index_to_change] = op_codes[operand1] + op_codes[operand2];
                index += 4
            
            },
            2 => {
                op_codes[index_to_change] = op_codes[operand1] * op_codes[operand2];
                index += 4
            },
            99 => {
                break
            },
            _ => panic!("Unknown opcode")
        }
    } 

    op_codes[0]
}
fn main() {
    for x in 0..100 {
        for y in 0..100 {
            if try_noun_verb(x, y) == 19690720 {
                println!("{}", 100 * x + y);
                break
            }
        }
    }

}
