package evaluator

import (
	"fmt"
	"strings"
)

func Evaluate(challenge Challenge, gender Gender, age Age, inResults map[Exercise]ResultValue) (ChallengeScore, error) {
	challengeScore := ChallengeScore{
		Total:     0,
		TotalMax:  0,
		Exercises: map[Exercise]ChallengeScoreExercise{},
	}
	results := map[Exercise]ResultValue{}
	challengeName := string(challenge)
	for exercise, v := range inResults {
		exerciseName := string(exercise)
		if !strings.HasPrefix(exerciseName, challengeName+"-") && !strings.HasPrefix(exerciseName, string(ExerciseTypeAerob)+"-") {
			exercise = Exercise(challengeName + "-" + exerciseName)
		}
		results[exercise] = v
	}
	missingExercises := []string{}
	for _, exercise := range challenge.GetExercizes() {
		if _, ok := results[exercise]; !ok {
			missingExercises = append(missingExercises, string(exercise))
		}
	}
	if len(missingExercises) > 0 {
		exerciseList := []string{}
		for ex := range results {
			exerciseList = append(exerciseList, string(ex))
		}
		l := strings.Join(exerciseList, ",")
		m := strings.Join(missingExercises, ",")
		return challengeScore, fmt.Errorf("not all exercies (%s) have results, missing: %s", l, m)
	}
	for exercise, resultValue := range results {
		score, err := TableEvaluate(exercise, gender, age, resultValue)
		if err != nil {
			return challengeScore, err
		}
		scoreMax := TableMaxScore(exercise, gender, age)
		challengeScore.Exercises[exercise] = ChallengeScoreExercise{
			Score:    score,
			ScoreMax: scoreMax,
		}
		challengeScore.Total += score
		challengeScore.TotalMax += scoreMax
	}
	return challengeScore, nil
}
