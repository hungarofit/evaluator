package evaluator

type Unit string

const (
	UnitUnknown   Unit = ""
	UnitMile      Unit = "mile"
	UnitMinute    Unit = "min"
	UnitKilometer Unit = "km"
	UnitMeter     Unit = "m"
	UnitCount     Unit = "n"
)

func IsValidUnit(u Unit) bool {
	return u == UnitMile ||
		u == UnitMinute ||
		u == UnitKilometer ||
		u == UnitMeter ||
		u == UnitCount
}

func (u Unit) IsAscending() bool {
	return u == UnitMile ||
		u == UnitKilometer ||
		u == UnitMeter ||
		u == UnitCount
}
