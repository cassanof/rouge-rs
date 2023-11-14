use tokenizer::Token;
pub mod rouge;
pub mod tokenizer;

pub fn lcs_table(refe: &[Token], can: &[Token]) -> Vec<Vec<usize>> {
    let rows = refe.len();
    let cols = can.len();

    // Initialize the LCS table with zeros
    let mut lcs_table = vec![vec![0; cols + 1]; rows + 1];

    for i in 1..=rows {
        for j in 1..=cols {
            if refe[i - 1] == can[j - 1] {
                lcs_table[i][j] = lcs_table[i - 1][j - 1] + 1;
            } else {
                lcs_table[i][j] = std::cmp::max(lcs_table[i - 1][j], lcs_table[i][j - 1]);
            }
        }
    }

    lcs_table
}
