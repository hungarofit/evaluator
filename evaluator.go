package evaluator

func Evaluate(challenge Challenge, gender Gender, age Age, results map[Exercise]ResultValue) (ChallengeScore, error) {
	cs := ChallengeScore{
		Total:     0,
		TotalMax:  0,
		Exercises: map[Exercise]ChallengeScoreExercise{},
	}
	for ex, rv := range results {
		s, err := TableEvaluate(ex, gender, age, rv)
		if err != nil {
			return cs, err
		}
		m := TableMaxScore(ex, gender, age)
		cs.Exercises[ex] = ChallengeScoreExercise{
			Score:    s,
			ScoreMax: m,
		}
		cs.Total += s
		cs.TotalMax += m
	}
	return cs, nil
}
