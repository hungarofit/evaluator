package evaluator_test

import (
	"strings"
	"testing"

	"github.com/hungarofit/evaluator"
)

func TestEvaluate(t *testing.T) {
	s, err := evaluator.Evaluate(evaluator.ChallengeHungarofitMini, evaluator.GenderFemale, 20, map[evaluator.Exercise]evaluator.ResultValue{
		evaluator.ExerciseAerob_Run12Min: 3000.0,
		evaluator.ExerciseMotor4_Jump:    20.0,
		evaluator.ExerciseMotor4_Pushup:  20.0,
		evaluator.ExerciseMotor4_Situp:   50.0,
		evaluator.ExerciseMotor4_Torso:   70.0,
	})
	if err != nil {
		t.Fatal(err)
	}
	if s.Total != 115.0 {
		t.Errorf("failed to match total sevaluator. expected 105, got %f", s.Total)
	}
}
func TestEvaluateErr(t *testing.T) {
	_, err := evaluator.Evaluate(evaluator.ChallengeHungarofitMini, evaluator.GenderFemale, 20, map[evaluator.Exercise]evaluator.ResultValue{})
	if err == nil || !strings.Contains(err.Error(), "missing result for exercise: ") {
		t.Fatal("error expected")
	}
}
