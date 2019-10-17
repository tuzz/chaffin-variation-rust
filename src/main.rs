use lehmer::Lehmer;

//const N: usize = 4;
//const FACT: [usize; N + 1] = [1, 1, 2, 6, 24];
//const SIZE: usize = (1 + 2 + 6) * 2;

//const N: usize = 5;
//const FACT: [usize; N + 1] = [1, 1, 2, 6, 24, 120];
//const SIZE: usize = (1 + 2 + 6 + 24) * 2;

const N: usize = 6;
const FACT: [usize; N + 1] = [1, 1, 2, 6, 24, 120, 720];
const SIZE: usize = (1 + 2 + 6 + 24 + 120) * 2;

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

    for w in 0.. {
        search(&mut seen, 0, &table, &reverse, 0, w, 0, &mut max_perms, &best_perms);
        best_perms[w] = max_perms;

        println!("waste {}, max perms {}", w, max_perms);

        if max_perms == FACT[N] / 2 {
            println!("shortest superpermutation: {}", ((N - 1) + FACT[N] / 2 + w) * 2 - 1);
            break;
        }
    }
}

fn search(seen: &mut [bool; FACT[N]], id: usize, table: &[[usize; SIZE]; FACT[N]], reverse: &[usize; FACT[N]], cur_waste: usize, max_waste: usize, perms: usize, max_perms: &mut usize, best_perms: &[usize; FACT[N]]) {
    seen[id] = true;
    seen[reverse[id]] = true;

    if *max_perms == perms {
        *max_perms += 1;
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

        search(seen, next_id, table, reverse, next_waste, max_waste, next_perms, max_perms, best_perms);
    }

    seen[id] = false;
    seen[reverse[id]] = false;
}