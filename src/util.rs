pub const ATOM_MATRIX_SIZE: usize = 3;
pub const MATRIX_SIZE: usize = ATOM_MATRIX_SIZE * ATOM_MATRIX_SIZE;
fn is_9_sequence(line: &[u8; MATRIX_SIZE]) -> bool {
    let mut sorted_line = line.clone();
    sorted_line.sort();
    for i in 0..MATRIX_SIZE {
        if sorted_line[i] != (i + 1) as u8 {
            return false;
        }
    }
    return true;
}
fn horizontally_safe(matrix: [[u8; MATRIX_SIZE]; MATRIX_SIZE]) -> bool {
    for line in matrix {
        if !is_9_sequence(&line) {
            return false;
        }
    }
    return true;
}
fn vertically_safe(matrix: [[u8; MATRIX_SIZE]; MATRIX_SIZE]) -> bool {
    for i in 0..MATRIX_SIZE {
        let mut line: [u8; MATRIX_SIZE] = [0; MATRIX_SIZE];
        for j in 0..matrix.len() {
            line[j] = matrix[j][i]
        }
        if !is_9_sequence(&line) {
            return false;
        }
    }
    return true;
}
fn squared_safe(matrix: [[u8; MATRIX_SIZE]; MATRIX_SIZE]) -> bool {
    for i in 0..MATRIX_SIZE {
        let mut line: [u8; MATRIX_SIZE] = [0; MATRIX_SIZE];
        for j in 0..line.len() {
            line[j] = matrix[i.checked_div(ATOM_MATRIX_SIZE).unwrap() * ATOM_MATRIX_SIZE
                + j.checked_div(ATOM_MATRIX_SIZE).unwrap()]
                [(i % ATOM_MATRIX_SIZE) * ATOM_MATRIX_SIZE + (j % ATOM_MATRIX_SIZE)];
        }
        if !is_9_sequence(&line) {
            return false;
        }
    }
    return true;
}
fn is_solved(matrix: [[u8; MATRIX_SIZE]; MATRIX_SIZE]) -> bool {
    return horizontally_safe(matrix) && vertically_safe(matrix) && squared_safe(matrix);
}
fn new_in_line(line: &[u8; MATRIX_SIZE], current_val: u8) -> bool {
    match line.iter().position(|&x| x == current_val) {
        Some(_) => false,
        None => true,
    }
}
fn get_missing_from_pos(matrix: &[[u8; MATRIX_SIZE]; MATRIX_SIZE], pos: u8) -> Vec<u8> {
    // println!("{:?}, {:?}", line, current_val);
    let h_line_number = pos.checked_div(MATRIX_SIZE as u8).unwrap() as usize;
    let v_line_number: usize = pos as usize % MATRIX_SIZE;
    let h_div_atom_size = h_line_number.checked_div(ATOM_MATRIX_SIZE).unwrap();
    let v_div_atom_size = v_line_number.checked_div(ATOM_MATRIX_SIZE).unwrap();
    // square line
    let mut q_line: [u8; MATRIX_SIZE] = [0; MATRIX_SIZE];

    for i in 0..ATOM_MATRIX_SIZE {
        for j in 0..ATOM_MATRIX_SIZE {
            let index = i * ATOM_MATRIX_SIZE + j;
            q_line[index] = matrix[h_div_atom_size * ATOM_MATRIX_SIZE + i]
                [v_div_atom_size * ATOM_MATRIX_SIZE + j];
        }
    }
    // println!("{:?}", q_line);
    let vec_v_line: Vec<u8> = (0..MATRIX_SIZE as u8)
        .into_iter()
        .map(|i| matrix[i as usize][v_line_number])
        .collect();
    let v_line: [u8; MATRIX_SIZE] = vec_v_line.try_into().unwrap();
    (1..MATRIX_SIZE as u8 + 1)
        .into_iter()
        .filter(|&x| {
            new_in_line(&matrix[h_line_number], x)
                && new_in_line(&v_line, x)
                && new_in_line(&q_line, x)
        })
        .collect()
}
pub fn solve(round: u32, matrix: [[u8; MATRIX_SIZE]; MATRIX_SIZE], pos: u8) -> bool {
    let total_size = MATRIX_SIZE * MATRIX_SIZE;
    if pos > total_size as u8 {
        return false;
    }
    if !is_solved(matrix) {
        let mut solved = false;
        let h_line_number = pos.checked_div(MATRIX_SIZE as u8).unwrap() as usize;
        let col = pos % MATRIX_SIZE as u8;
        if matrix[h_line_number][col as usize] == 0 {
            // let vertical_line =
            let missed_vals = get_missing_from_pos(&matrix, pos);
            for j in 0..missed_vals.len() {
                let mut new_matrix = matrix.clone();
                new_matrix[h_line_number][col as usize] = missed_vals[j];
                solved = solve(round + 1, new_matrix, pos + 1);
                if solved {
                    break;
                }
            }
        } else {
            solved = solve(round + 1, matrix, pos + 1);
        }
        return solved;
    } else {
        println!("Solved in {:#} round(s)!", round);
        for i in 0..MATRIX_SIZE {
            println!("{:?}", matrix[i])
        }
        return true;
    }
}
