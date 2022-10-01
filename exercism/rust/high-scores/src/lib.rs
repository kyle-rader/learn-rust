#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    scores_sorted: Vec<u32>,
}

const TOP_N: usize = 3;

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.into(),
            scores_sorted: HighScores::top_n(scores, TOP_N),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.iter().last().map(|v| *v)
    }

    pub fn personal_best(&self) -> Option<u32> {
        (!self.scores.is_empty()).then_some(self.scores.iter().fold(0, |acc, i| {
            if i > &acc {
                *i
            } else {
                acc
            }
        }))
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores_sorted.iter().take(TOP_N).map(|i| *i).collect()
    }

    fn top_n(scores: &[u32], n: usize) -> Vec<u32> {
        let mut scores: Vec<u32> = scores.clone().into();
        scores.sort();
        scores.iter().rev().take(n).map(|i| *i).collect()
    }
}
