package evaluator

type ChallengeScore struct {
	Total     Score
	Exercises map[Exercise]Score
	WithAerob bool
}

type Challenge string

const (
	ChallengeUnknown        Challenge = ""
	ChallengeHungarofit     Challenge = "hfit6"
	ChallengeHungarofitMini Challenge = "hfit4"
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
