pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (i, row) in input.iter().enumerate() {
        if let Some(max) = row.iter().max() {
            for (j, cell) in row.iter().enumerate() {
                if cell == max && input.iter().all(|row| cell <= row.get(j).unwrap()) {
                    saddle_points.push((i, j));
                }
            }
        }
    }

    saddle_points
}
