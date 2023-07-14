package evaluator

func Evaluate(challenge Challenge, gender Gender, age Age, results map[Exercise]ResultValue) (ChallengeScore, error) {
	cs := ChallengeScore{
		Total:     0,
		WithAerob: false,
		Exercises: map[Exercise]Score{},
	}
	for ex, rv := range results {
		s, err := TableEvaluate(ex, gender, age, rv)
		if err != nil {
			return cs, err
		}
		cs.Exercises[ex] = s
		cs.Total += s
		if ex.GetType() == ExerciseTypeAerob {
			cs.WithAerob = true
		}
	}
	return cs, nil
}
