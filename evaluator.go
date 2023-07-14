package evaluator

import "fmt"

func Evaluate(challenge Challenge, gender Gender, age Age, results map[Exercise]ResultValue) (ChallengeScore, error) {
	cs := ChallengeScore{
		Total:     0,
		TotalMax:  0,
		Exercises: map[Exercise]ChallengeScoreExercise{},
	}
	for _, ex := range challenge.GetExercizes() {
		if _, ok := results[ex]; !ok {
			return cs, fmt.Errorf("missing result for exercise: " + ex.GetName())
		}
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
