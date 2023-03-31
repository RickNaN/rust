use rand::{Rng, thread_rng};
use std::env::args;
use std::io::stdin;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut state = [[0i32; 100]; 100];
    // passo da un vettore di stringhe a una matrice di interi
    for i in 0..minefield.len() {
        let tmp = minefield[i];
        for j in 0..minefield[0].len() {
            if (tmp[j..=j].chars().last().unwrap() == '*')
            {state[i][j] = -1}
        }
    }

    // mettendo i numeri. Si poteva fare con match?
    for i in 0..minefield.len() {
        for j in 0..minefield[0].len() {
            if (state[i][j] == -1) {
                // Verticale
                if (i > 0 && state[i-1][j] != -1) {
                    state[i-1][j] += 1;
                }
                if (i < minefield[0].len() && state[i+1][j] != -1) {
                    state[i+1][j] += 1;
                }
                // Orizzontale
                if (j > 0 && state[i][j-1] != -1) {
                    state[i][j-1] += 1;
                }
                if (j < minefield[0].len() && state[i][j+1] != -1) {
                    state[i][j+1] += 1;
                }
                // Obliquo
                if (i > 0 && j > 0 && state[i-1][j-1] != -1) {
                    state[i-1][j-1] += 1;
                }
                if (i < minefield[0].len() && j < minefield[0].len() && state[i+1][j+1] != -1) {
                    state[i+1][j+1] += 1;
                }
                if (i > 0 && j < minefield[0].len() && state[i-1][j+1]!=-1) {
                    state[i-1][j+1] += 1;
                }
                if (i < minefield[0].len() && j > 0 && state[i+1][j-1]!=-1) {
                    state[i+1][j-1] += 1;
                }
            }
        }
    }

    for i in 0..minefield.len() {
        let mut tmp: String = String::new();
        for j in 0..minefield[0].len() {
            if state[i][j] == 0 {
                tmp.push(' ');
            }
            else if state[i][j] == -1 {
                tmp.push('*');
            }
            else {
                tmp.push(char::from_digit(state[i][j] as u32, 32).unwrap());
            }
        } res.push(tmp);
    }

    return res;
}

fn main () {
    let mut support: Vec<&str> = vec![];
    let args: Vec<String> = args().skip(1).collect();
    let mut dim = (0,0);
    let mut field = "";
    if args.len() > 0 {
        // non usare args[index] CANNOT MOVE
        // support.set_len() UNSAFE

        let mut tmp = vec![];
        tmp = args.get(0).unwrap().chars().filter(|x| x.is_numeric()).collect();
        dim.0 = tmp.get(0).unwrap().to_digit(32).unwrap();
        tmp = args.get(1).unwrap().chars().filter(|x| x.is_numeric()).collect();
        dim.1 = tmp.get(0).unwrap().to_digit(32).unwrap();
        field = args.last().unwrap();
    }
    for i in 1..=dim.0 {
        let a = (i - 1) * dim.1;
        let b = i * dim.1;
        let slice = field.get(a as usize..b as usize).unwrap();
        support.push(slice);
    }

    let param = support.as_slice();
    let res = annotate(param);

    for iter in res {
        println!("{}", iter);
    }

}