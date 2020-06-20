#[derive(Debug)]
pub struct HighScores<'a> {
    all_scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { all_scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.all_scores.last().cloned()
        //        match self.latest_score {
        //            Some(&score) => Some(score),
        //            None => None,
        //        }
        // we pattern match the reference to the value, then return it copied in a 'Some'
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.all_scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.all_scores.to_vec();
        scores.sort_by(|a, b| b.cmp(a));
        scores.truncate(3);
        scores
    }
}
