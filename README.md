# evaluator

## Hungarofit evaluation logic library for Go

### Generating tables from XLSX files

```bash
# cwd: project root
go run cmd/tables_generate/main.go
```

### Usage

```bash
go get -u github.com/hungarofit/evaluator
```

```go
import "github.com/hungarofit/evaluator"

func main() {
    score, err := evaluator.Evaluate(evaluator.ChallengeHungarofitMini, evaluator.GenderFemale, 20, map[evaluator.Exercise]evaluator.ResultValue{
		evaluator.ExerciseAerob_Run12Min: 3000.0,
		evaluator.ExerciseMotor4_Jump:    20.0,
		evaluator.ExerciseMotor4_Pushup:  20.0,
		evaluator.ExerciseMotor4_Situp:   50.0,
		evaluator.ExerciseMotor4_Torso:   70.0,
	})
	if err != nil {
		log.Fatalln(err)
	}
	log.Println(score.Total)
}
```

_Dr. Ildikó Mérey_, _Lajos Bencz_
