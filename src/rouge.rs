use crate::tokenizer::{RougeTokenizer, Token, Tokenizer};

pub struct Score {
    pub precision: f64,
    pub recall: f64,
    pub fmeasure: f64,
}

pub trait Scorer {
    fn score(&self, refe: &str, can: &str) -> Score;
}

pub struct RougeLScorer {
    tokenizer: Box<dyn Tokenizer>,
}

impl RougeLScorer {
    pub fn new(tokenizer: Box<dyn Tokenizer>) -> Self {
        RougeLScorer { tokenizer }
    }
}

impl Default for RougeLScorer {
    fn default() -> Self {
        RougeLScorer::new(Box::new(RougeTokenizer))
    }
}

impl Scorer for RougeLScorer {
    fn score(&self, refe: &str, can: &str) -> Score {
        let mut mut_refe = String::from(refe);
        let mut mut_can = String::from(can);

        let refe_tokens = self.tokenizer.tokenize(&mut mut_refe);
        let can_tokens = self.tokenizer.tokenize(&mut mut_can);

        if refe_tokens.is_empty() || can_tokens.is_empty() {
            return Score {
                precision: 0.0,
                recall: 0.0,
                fmeasure: 0.0,
            };
        }

        let lcs_table = lcs_table(&refe_tokens, &can_tokens);
        let lcs_len = lcs_table[refe_tokens.len()][can_tokens.len()] as f64;

        let precision = lcs_len / can_tokens.len() as f64;
        let recall = lcs_len / refe_tokens.len() as f64;

        Score {
            precision,
            recall,
            fmeasure: fmeasure(precision, recall),
        }
    }
}

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

pub fn fmeasure(precision: f64, recall: f64) -> f64 {
    if precision == 0.0 && recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let refe = ["a", "b", "c"];
        let can = ["a", "c", "d", "b"];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[3][4], 2);
    }

    #[test]
    fn empty_reference() {
        let refe: [Token; 0] = [];
        let can = ["a", "b"];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[0][2], 0);
    }

    #[test]
    fn empty_candidate() {
        let refe = ["a", "b"];
        let can: [Token; 0] = [];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[2][0], 0);
    }

    #[test]
    fn both_empty() {
        let refe: [Token; 0] = [];
        let can: [Token; 0] = [];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[0][0], 0);
    }

    #[test]
    fn identical_arrays() {
        let refe = ["a", "b", "c"];
        let can = ["a", "b", "c"];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[3][3], 3);
    }

    #[test]
    fn no_common_tokens() {
        let refe = ["a", "b"];
        let can = ["c", "d"];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[2][2], 0);
    }

    #[test]
    fn single_common_token() {
        let refe = ["a", "b"];
        let can = ["b", "c"];
        let table = lcs_table(&refe, &can);
        assert_eq!(table[2][2], 1);
    }
}
