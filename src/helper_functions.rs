pub fn row_and_col_to_cube_location(row_index: usize, col_index: usize) -> (usize, usize) {
    // returns the cube index, and inner cube index, given the coordinates
    let row_three = (row_index / 3) * 3;
    let col_three = col_index / 3;
    let cube_index = row_three + col_three;
    // now lets get the index inside the cube
    let row_mod_three = row_index % 3;
    let col_mod_three = col_index % 3;
    let inner_cube_index = (row_mod_three * 3) + col_mod_three;

    (cube_index, inner_cube_index)
}

pub fn intersection_of_u32_vectors(vector_a: Vec<u32>, vector_b: Vec<u32>) -> Vec<u32> {
    let mut intersection_vec: Vec<u32> = Vec::new();

    for value in vector_a.iter() {
        if vector_b.contains(value) {
            intersection_vec.push(*value);
        }
    }

    intersection_vec
}

pub fn collapse_three_vectors(
    vector_a: Vec<u32>,
    vector_b: Vec<u32>,
    vector_c: Vec<u32>,
) -> Vec<u32> {
    let mut inter_vec: Vec<u32> = intersection_of_u32_vectors(vector_a, vector_b);
    inter_vec = intersection_of_u32_vectors(inter_vec, vector_c);
    inter_vec
}
