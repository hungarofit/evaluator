package evaluator

type Gender uint8

const (
	GenderUnknown Gender = iota
	GenderMale
	GenderFemale
)

func (g Gender) Int() int {
	return int(g)
}

func (g Gender) String() string {
	switch g {
	case GenderMale:
		return "male"
	case GenderFemale:
		return "female"
	}
	return "unknown"
}

func (g Gender) SheetName() string {
	switch g {
	case GenderFemale:
		return "lányok"
	case GenderMale:
		return "fiúk"
	}
	return ""
}

func (g Gender) IsValid() bool {
	return g == GenderUnknown
}
