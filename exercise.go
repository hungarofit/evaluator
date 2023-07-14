package evaluator

import (
	"strings"
)

type ExerciseType string

const (
	ExerciseTypeUnknown ExerciseType = ""
	ExerciseTypeAerob   ExerciseType = "aerob"
	ExerciseTypeMotor4  ExerciseType = "motor4"
	ExerciseTypeMotor6  ExerciseType = "motor6"
)

type Exercise string

var (
	ExerciseAerob_Unknown   Exercise = ""
	ExerciseAerob_Bike12Min Exercise = "aerob-bike-12min"
	ExerciseAerob_Run1Mile  Exercise = "aerob-run-1mile"
	ExerciseAerob_Run1Mile5 Exercise = "aerob-run-1mile5"
	ExerciseAerob_Run2Km    Exercise = "aerob-run-2km"
	ExerciseAerob_Run2Mile  Exercise = "aerob-run-2mile"
	ExerciseAerob_Run3Km    Exercise = "aerob-run-3km"
	ExerciseAerob_Run6Min   Exercise = "aerob-run-6min"
	ExerciseAerob_Run12Min  Exercise = "aerob-run-12min"
	ExerciseAerob_Swim12Min Exercise = "aerob-swim-12min"
	ExerciseAerob_Swim500m  Exercise = "aerob-swim-500m"
)

var (
	ExerciseMotor4_Unknown Exercise = ""
	ExerciseMotor4_Jump    Exercise = "motor4-jump"
	ExerciseMotor4_Pushup  Exercise = "motor4-pushup"
	ExerciseMotor4_Situp   Exercise = "motor4-situp"
	ExerciseMotor4_Torso   Exercise = "motor4-torso"
)

var (
	ExerciseMotor6_Unknown     Exercise = ""
	ExerciseMotor6_Jump        Exercise = "motor6-jump"
	ExerciseMotor6_Pushup      Exercise = "motor6-pushup"
	ExerciseMotor6_Situp       Exercise = "motor6-situp"
	ExerciseMotor6_Torso       Exercise = "motor6-torso"
	ExerciseMotor6_Throwdouble Exercise = "motor6-throwdouble"
	ExerciseMotor6_Throwsingle Exercise = "motor6-throwsingle"
)

type ExerciseEvaluator interface {
	GetType() string
	GetName() string
	Evaluate(g Gender, a Age, r ResultValue) (Score, error)
}

func (e Exercise) GetType() ExerciseType {
	switch strings.Split(string(e), "-")[0] {
	case string(ExerciseTypeAerob):
		return ExerciseTypeAerob
	case string(ExerciseTypeMotor4):
		return ExerciseTypeMotor4
	case string(ExerciseTypeMotor6):
		return ExerciseTypeMotor6
	}
	return ExerciseTypeUnknown
}

func (e Exercise) GetName() string {
	l := strings.SplitN(string(e), "-", 2)
	if len(l) < 2 {
		return ""
	}
	return l[1]
}

func (e Exercise) GetResultUnit() Unit {
	et := e.GetType()
	if et == "" {
		return UnitUnknown
	}
	if et == ExerciseTypeAerob {
		switch e {
		case ExerciseAerob_Bike12Min:
			return UnitMeter
		case ExerciseAerob_Run12Min:
			return UnitMeter
		case ExerciseAerob_Run6Min:
			return UnitMeter
		case ExerciseAerob_Swim12Min:
			return UnitMeter
		case ExerciseAerob_Run1Mile:
			return UnitMinute
		case ExerciseAerob_Run1Mile5:
			return UnitMinute
		case ExerciseAerob_Run2Mile:
			return UnitMinute
		case ExerciseAerob_Run2Km:
			return UnitMinute
		case ExerciseAerob_Run3Km:
			return UnitMinute
		case ExerciseAerob_Swim500m:
			return UnitMinute
		}
	}
	if e == ExerciseMotor6_Throwsingle ||
		e == ExerciseMotor6_Throwdouble {
		return UnitMeter
	}
	return UnitCount
}
