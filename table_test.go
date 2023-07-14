package evaluator_test

import (
	"testing"

	"github.com/hungarofit/evaluator"
)

func TestTable(t *testing.T) {

	list := []struct {
		evaluator.Exercise
		evaluator.Gender
		evaluator.Age
		evaluator.ResultValue
		evaluator.Score
	}{
		{
			Exercise:    evaluator.ExerciseAerob_Bike12Min,
			Gender:      evaluator.GenderMale,
			Age:         20,
			ResultValue: 1,
			Score:       0.0,
		},
		{
			Exercise:    evaluator.ExerciseAerob_Bike12Min,
			Gender:      evaluator.GenderMale,
			Age:         20,
			ResultValue: 4150,
			Score:       11.0,
		},
		{
			Exercise:    evaluator.ExerciseAerob_Bike12Min,
			Gender:      evaluator.GenderMale,
			Age:         20,
			ResultValue: 10000,
			Score:       77.0,
		},
		{
			Exercise:    evaluator.ExerciseAerob_Run1Mile,
			Gender:      evaluator.GenderFemale,
			Age:         20,
			ResultValue: 14,
			Score:       0.0,
		},
		{
			Exercise:    evaluator.ExerciseAerob_Run1Mile,
			Gender:      evaluator.GenderFemale,
			Age:         20,
			ResultValue: 12.0,
			Score:       10.0,
		},
		{
			Exercise:    evaluator.ExerciseAerob_Run1Mile,
			Gender:      evaluator.GenderFemale,
			Age:         20,
			ResultValue: 6.0,
			Score:       77.0,
		},
		{
			Exercise:    evaluator.ExerciseMotor4_Jump,
			Gender:      evaluator.GenderFemale,
			Age:         20,
			ResultValue: 1.77,
			Score:       6.0,
		},
		{
			Exercise:    evaluator.ExerciseMotor4_Jump,
			Gender:      evaluator.GenderFemale,
			Age:         26,
			ResultValue: 1.77,
			Score:       18.0,
		},
	}

	for _, tv := range list {
		score, err := evaluator.TableEvaluate(tv.Exercise, tv.Gender, tv.Age, tv.ResultValue)
		if err != nil {
			t.Fatal(err)
		}
		if score != tv.Score {
			t.Errorf("scoring mismatch for %#v, got: %f", tv, score)
		}
	}
}
