package evaluator

type ChallengeScoreExercise struct {
	Score    Score `json:"score"`
	ScoreMax Score `json:"score_max"`
}

type ChallengeScore struct {
	Total     Score                               `json:"total"`
	TotalMax  Score                               `json:"total_max"`
	Exercises map[Exercise]ChallengeScoreExercise `json:"exercises"`
}

type Challenge string

const (
	ChallengeUnknown        Challenge = ""
	ChallengeHungarofit     Challenge = "motor6"
	ChallengeHungarofitMini Challenge = "motor4"
)

func (c Challenge) IsValid() bool {
	return c == ChallengeHungarofit ||
		c == ChallengeHungarofitMini
}

func (c Challenge) GetExercizes() []Exercise {
	switch c {
	case ChallengeHungarofit:
		return []Exercise{
			ExerciseMotor6_Jump,
			ExerciseMotor6_Pushup,
			ExerciseMotor6_Situp,
			ExerciseMotor6_Throwdouble,
			ExerciseMotor6_Throwsingle,
			ExerciseMotor6_Torso,
		}
	case ChallengeHungarofitMini:
		return []Exercise{
			ExerciseMotor4_Jump,
			ExerciseMotor4_Pushup,
			ExerciseMotor4_Situp,
			ExerciseMotor4_Torso,
		}
	}
	return []Exercise{}
}
