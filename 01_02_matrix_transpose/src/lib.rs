pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut out_matrix = [
        [0,0,0],
        [0,0,0],
        [0,0,0],
    ];

    let mut row_count = 0;
    for row in matrix {
        let mut cell_count = 0;
        for cell in row {
            out_matrix[cell_count][row_count] = cell;
            cell_count += 1
        }
        row_count += 1
    }


    out_matrix
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_test() {
        let matrix = [
            [101, 102, 103], //
            [201, 202, 203],
            [301, 302, 303],
        ];
        let transposed = transpose(matrix);
        assert_eq!(
            transposed,
            [
                [101, 201, 301], //
                [102, 202, 302],
                [103, 203, 303],
            ]
        );
    }
}
