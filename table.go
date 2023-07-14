package evaluator

import (
	"fmt"
)

type TableEvaluator interface {
	Evaluate(age Age, result ResultValue) Score
}

type Table struct {
	unit    Unit
	ages    []Age
	scores  []Score
	results [][]ResultValue
}

func (t *Table) GetUnit() Unit {
	return t.unit
}

func (t *Table) AgeCount() int {
	return len(t.ages)
}

func (t *Table) AgeMin() Age {
	return Age(t.ages[0])
}

func (t *Table) AgeMax() Age {
	return Age(t.ages[len(t.ages)-1])
}

func (t *Table) ScoreCount() int {
	return len(t.scores)
}

func (t *Table) ScoreMin() Score {
	return Score(t.scores[0])
}

func (t *Table) ScoreMax() Score {
	return Score(t.scores[len(t.scores)-1])
}

func (t *Table) AgeIndex(age Age) int {
	ageIndex := 0
	for ai, a := range t.ages {
		if a > age {
			break
		}
		ageIndex = ai
	}
	return ageIndex
}

func (t *Table) Evaluate(age Age, result ResultValue) Score {
	ageIndex := t.AgeIndex(age)
	asc := t.unit.IsAscending()
	// if asc {
	// 	if t.results[0][ageIndex] > result {
	// 		return Score(0)
	// 	}
	// } else {
	// 	if t.results[0][ageIndex] > result {
	// 		return Score(0)
	// 	}
	// }
	scoreIndex := -1
	for _, r := range t.results {
		if ageIndex >= len(r) {
			break
		}
		rv := r[ageIndex]
		if rv <= 0 {
			continue
		}
		if asc {
			if rv > result {
				break
			}
		} else {
			if rv < result {
				break
			}
		}
		scoreIndex += 1
	}
	if scoreIndex < 0 {
		return Score(0)
	}
	return t.scores[scoreIndex]
}

func TableEvaluate(exercise Exercise, gender Gender, age Age, result ResultValue) (Score, error) {
	te, ok := tables[exercise]
	if !ok {
		return 0, fmt.Errorf("no such exercise: " + string(exercise))
	}
	teg, ok := te[gender]
	if !ok {
		return 0, fmt.Errorf("no such gender: " + string(gender))
	}
	return teg.Evaluate(age, result), nil
}
