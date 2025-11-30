use std::fmt;

use crate::exercise::{ChallengeType, Exercise};

#[derive(Debug, Clone, Copy)]
pub struct Challenge {
    pub challenge_type: ChallengeType,
    pub aerob_exercise: Exercise,
}

impl Challenge {
    pub fn new(challenge_type: ChallengeType, aerob_exercise: Exercise) -> Self {
        Self {
            challenge_type,
            aerob_exercise,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Concerning,
    Weak,
    Mediocre,
    Average,
    Good,
    Excellent,
    Outstanding,
}

impl Classification {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s < 20.5 => Self::Concerning,
            s if s < 40.5 => Self::Weak,
            s if s < 60.5 => Self::Mediocre,
            s if s < 80.5 => Self::Average,
            s if s < 100.5 => Self::Good,
            s if s < 120.5 => Self::Excellent,
            _ => Self::Outstanding,
        }
    }

    pub fn score_range(&self) -> (f32, f32) {
        match self {
            Self::Concerning => (0.0, 20.499),
            Self::Weak => (20.5, 40.499),
            Self::Mediocre => (40.5, 60.499),
            Self::Average => (60.5, 80.499),
            Self::Good => (80.5, 100.499),
            Self::Excellent => (100.5, 120.499),
            Self::Outstanding => (120.5, 140.0),
        }
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Concerning => "Concerning",
            Self::Weak => "Weak",
            Self::Mediocre => "Mediocre",
            Self::Average => "Average",
            Self::Good => "Good",
            Self::Excellent => "Excellent",
            Self::Outstanding => "Outstanding",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChallengeScore {
    pub exercise: Exercise,
    pub score: f32,
}

#[derive(Debug)]
pub struct ChallengeResult {
    pub total_score: f32,
    pub classification: Classification,
    pub motor_scores: Vec<ChallengeScore>,
    pub aerob_score: ChallengeScore,
}

impl ChallengeResult {
    pub fn new(motor_scores: Vec<ChallengeScore>, aerob_score: ChallengeScore) -> Self {
        let total_score = motor_scores.iter().map(|s| s.score).sum::<f32>() + aerob_score.score;
        let classification = Classification::from_score(total_score);

        Self {
            total_score,
            classification,
            motor_scores,
            aerob_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_from_score() {
        assert_eq!(Classification::from_score(0.0), Classification::Concerning);
        assert_eq!(Classification::from_score(20.49), Classification::Concerning);
        assert_eq!(Classification::from_score(20.5), Classification::Weak);
        assert_eq!(Classification::from_score(40.49), Classification::Weak);
        assert_eq!(Classification::from_score(40.5), Classification::Mediocre);
        assert_eq!(Classification::from_score(60.49), Classification::Mediocre);
        assert_eq!(Classification::from_score(60.5), Classification::Average);
        assert_eq!(Classification::from_score(80.49), Classification::Average);
        assert_eq!(Classification::from_score(80.5), Classification::Good);
        assert_eq!(Classification::from_score(100.49), Classification::Good);
        assert_eq!(Classification::from_score(100.5), Classification::Excellent);
        assert_eq!(Classification::from_score(120.49), Classification::Excellent);
        assert_eq!(Classification::from_score(120.5), Classification::Outstanding);
        assert_eq!(Classification::from_score(140.0), Classification::Outstanding);
    }

    #[test]
    fn test_classification_ordering() {
        assert!(Classification::Concerning < Classification::Weak);
        assert!(Classification::Weak < Classification::Mediocre);
        assert!(Classification::Mediocre < Classification::Average);
        assert!(Classification::Average < Classification::Good);
        assert!(Classification::Good < Classification::Excellent);
        assert!(Classification::Excellent < Classification::Outstanding);
    }

    #[test]
    fn test_challenge_result_new() {
        let motor_scores = vec![
            ChallengeScore { exercise: Exercise::Jump, score: 10.0 },
            ChallengeScore { exercise: Exercise::Pushup, score: 15.0 },
        ];
        let aerob_score = ChallengeScore { exercise: Exercise::AerobRun2Km, score: 12.0 };

        let result = ChallengeResult::new(motor_scores, aerob_score);
        assert_eq!(result.total_score, 37.0);
        assert_eq!(result.classification, Classification::Weak);
    }
}
