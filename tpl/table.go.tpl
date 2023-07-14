package evaluator

var tables = map[Exercise]map[Gender]*Table{
    {{- range $exercise, $genderTables := . }}
	"{{ $exercise }}": {
        {{- range $gender, $tableData := $genderTables }}
		{{ $tableData.gender }}: &Table{
            unit: Unit("{{ $tableData.unit }}"),
            ages: []Age{
                {{- range $tableData.ages }}
                {{ . }},
                {{- end }}
            },
            scores: []Score{
                {{- range $tableData.scores }}
                {{ . }},
                {{- end }}
            },
            results: [][]ResultValue{
                {{- range $tableData.results }}
                {
                    {{- range . }}
                    {{ . }},
                    {{- end }}
                },
                {{- end }}
            },
		},
        {{- end }}
	},
    {{- end }}
}
