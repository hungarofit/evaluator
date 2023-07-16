package evaluator

import (
	"fmt"
	"strings"
)

func Evaluate(challenge Challenge, gender Gender, age Age, results map[Exercise]ResultValue) (ChallengeScore, error) {
	cs := ChallengeScore{
		Total:     0,
		TotalMax:  0,
		Exercises: map[Exercise]ChallengeScoreExercise{},
	}
	exMiss := []string{}
	for _, ex := range challenge.GetExercizes() {
		if _, ok := results[ex]; !ok {
			exMiss = append(exMiss, string(ex))
		}
	}
	if len(exMiss) > 0 {
		exList := []string{}
		for ex := range results {
			exList = append(exList, string(ex))
		}
		l := strings.Join(exList, ",")
		m := strings.Join(exMiss, ",")
		return cs, fmt.Errorf("not all exercies (%s) have results, missing: %s", l, m)
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
