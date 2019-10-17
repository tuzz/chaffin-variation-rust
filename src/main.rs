use lehmer::Lehmer;
use chrono::Local;

//const N: usize = 4;
//const FACT: [usize; N + 1] = [1, 1, 2, 6, 24];
//const SIZE: usize = (1 + 2 + 6) * 2;

//const N: usize = 5;
//const FACT: [usize; N + 1] = [1, 1, 2, 6, 24, 120];
//const SIZE: usize = (1 + 2 + 6 + 24) * 2;

const N: usize = 6;
const FACT: [usize; N + 1] = [1, 1, 2, 6, 24, 120, 720];
const SIZE: usize = (1 + 2 + 6 + 24 + 120) * 2;

const BENCHMARK: bool = true;

fn main() {
    let mut table = [[0; SIZE]; FACT[N]];
    let mut reverse = [0; FACT[N]];
    let mut best_perms = [99999; FACT[N]];

    for id in 0..FACT[N] {
        let mut row = 0;
        let mut current = Lehmer::from_decimal(id, N).to_permutation();

        for waste in 0..(N - 1) {
            let head = &current[0..=waste];
            let tail = &current[waste + 1..N];

            for i in 0..FACT[head.len()] {
                let perm = Lehmer::from_decimal(i, head.len()).to_permutation();
                let new_tail = perm.iter().map(|p| head[*p as usize]).collect::<Vec<_>>();

                let mut next = tail.to_vec();
                next.extend(new_tail);

                let next_id = Lehmer::from_permutation(&next).to_decimal();

                table[id][row] = next_id;
                table[id][row + 1] = waste;

                row += 2;
            }
        }

        current.reverse();
        reverse[id] = Lehmer::from_permutation(&current).to_decimal();
    }


    let mut seen = [false; FACT[N]];
    let mut max_perms = 0;
    let mut string = [0; FACT[N] * 2];

    for w in 0.. {
        search(&mut seen, 0, &table, &reverse, 0, w, 0, &mut max_perms, &best_perms, &mut string, 0);
        best_perms[w] = max_perms;

        println!("{}", Local::now().format("%Y-%m-%d:%H:%M:%S"));
        println!("{} wasted characters, at most {} permutations", w, max_perms);
        println!();

        if BENCHMARK && w == 52 {
            break;
        }

        if max_perms == FACT[N] / 2 {
            println!("shortest superpermutation: {}", ((N - 1) + FACT[N] / 2 + w) * 2 - 1);
            break;
        }
    }
}

fn search(seen: &mut [bool; FACT[N]], id: usize, table: &[[usize; SIZE]; FACT[N]], reverse: &[usize; FACT[N]], cur_waste: usize, max_waste: usize, perms: usize, max_perms: &mut usize, best_perms: &[usize; FACT[N]], string: &mut [usize; FACT[N] * 2], cursor: usize) {
    seen[id] = true;
    seen[reverse[id]] = true;
    string[cursor] = id;

    if *max_perms == perms {
        *max_perms += 1;
        print_string(string, cursor);
    }

    for chunk in table[id].chunks(2) {
        let next_id = chunk[0];
        let waste = chunk[1];

        let next_waste = cur_waste + waste;

        if next_waste > max_waste {
            continue;
        }

        if seen[next_id] {
            continue;
        }

        if seen[reverse[next_id]] {
            continue;
        }

        let next_perms = perms + 1;
        let possible_perms = next_perms + best_perms[max_waste - next_waste];

        if possible_perms <= *max_perms {
            continue;
        }

        search(seen, next_id, table, reverse, next_waste, max_waste, next_perms, max_perms, best_perms, string, cursor + 1);
    }

    seen[id] = false;
    seen[reverse[id]] = false;
}

fn print_string(string: &[usize; FACT[N] * 2], cursor: usize) {
    println!("{}", Local::now().format("%Y-%m-%d:%H:%M:%S"));

    let mut last = 0;
    print!("{}", last + 1);

    for c in 0..=cursor {
        let perm = Lehmer::from_decimal(string[c], N).to_permutation();

        for p in perm.iter().skip_while(|p| **p != last).skip(1) {
            print!("{}", p + 1);
        }

        last = *perm.last().unwrap();
    }
    println!();
    println!();
}
