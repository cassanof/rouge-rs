use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::rouge::{RougeLScorer, Scorer};

pub fn rouge_dedup<'a>(scorer: &dyn Scorer, solutions: &[&'a str], threshold: f64) -> Vec<&'a str> {
    let mut keep_mask = vec![true; solutions.len()];

    for i in 0..solutions.len() {
        for j in (i + 1)..solutions.len() {
            if i == j || !keep_mask[j] {
                continue;
            }

            let scores = scorer.score(solutions[i], solutions[j]);
            let rouge_score = scores.fmeasure;

            if rouge_score > threshold {
                keep_mask[j] = false;
            }
        }
    }

    solutions
        .iter()
        .enumerate()
        .filter(|(i, _)| keep_mask[*i])
        .map(|(_, sol)| *sol)
        .collect()
}

pub fn rouge_l_grouped_dedup<'a>(
    solutions: &[&'a str],
    threshold: f64,
    dedup_prob: f64,
    max_group_size: usize,
) -> Vec<&'a str> {
    fn compute_rounds(n: usize, group_size: usize, wanted_prob: f64) -> usize {
        let p = (group_size - 1) as f64 / (n - 1) as f64;
        ((1.0 - wanted_prob).log(1.0 - p)).ceil() as usize
    }
    let scorer = RougeLScorer::default();
    let mut group_size = std::cmp::min(solutions.len(), max_group_size);
    let dedup_rounds = compute_rounds(solutions.len(), group_size, dedup_prob);
    let mut prev_num_sols = solutions.len();
    let mut solutions = solutions.to_vec();
    for rnd in 0..dedup_rounds {
        println!(
            " #### global dedup round {}/{}. current num solutions: {} ####",
            rnd + 1,
            dedup_rounds,
            solutions.len()
        );

        // shuffle solutions
        solutions.shuffle(&mut rand::thread_rng());
        let groups = solutions.chunks(group_size).collect::<Vec<_>>();
        println!(
            "   # deduping {} groups with {} solutions each #",
            groups.len(),
            group_size
        );

        // dedup each group
        let deduped_groups = groups
            .into_par_iter()
            .map(|group| rouge_dedup(&scorer, group, threshold))
            .collect::<Vec<_>>();

        // flatten
        solutions = deduped_groups.into_iter().flatten().collect();
        group_size = std::cmp::min(solutions.len(), max_group_size);
        let dedup_rate = 1.0 - (solutions.len() as f64 / prev_num_sols as f64);
        println!(
            "   # dedup round {} complete. current num solutions: {}. dedup rate: {} #",
            rnd + 1,
            solutions.len(),
            dedup_rate
        );

        prev_num_sols = solutions.len();
    }

    solutions
}
