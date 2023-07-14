//go:build ignore

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"text/template"

	"github.com/hungarofit/evaluator"
	"github.com/iancoleman/strcase"
	"github.com/xuri/excelize/v2"
)

const (
	sheetExtension   = ".xlsx"
	sheetRelativeDir = "xlsx/"
	tplRelativeDir   = "tpl/"
	outRelativeDir   = ""
)

func processXlsxFile(xlsPath string, gender evaluator.Gender) ([]evaluator.Age, []evaluator.Score, [][]evaluator.ResultValue, error) {
	f, err := excelize.OpenFile(xlsPath)
	if err != nil {
		return nil, nil, nil, err
	}
	defer func() {
		if err := f.Close(); err != nil {
			log.Println(err)
		}
	}()
	rows, err := f.GetRows(gender.SheetName())
	if err != nil {
		return nil, nil, nil, err
	}
	ages := []evaluator.Age{}
	scores := []evaluator.Score{}
	results := [][]evaluator.ResultValue{}
	for ri, row := range rows {
		if ri == 0 {
			for _, c := range row[1:] {
				f, err := strconv.ParseFloat(c, 32)
				if err != nil {
					return nil, nil, nil, err
				}
				ages = append(ages, evaluator.Age(f))
			}
			continue
		}
		f, err := strconv.ParseFloat(row[0], 32)
		if err != nil {
			return nil, nil, nil, err
		}
		scores = append(scores, evaluator.Score(f))
		results = append(results, []evaluator.ResultValue{})
		for _, c := range row[1:] {
			f, _ := strconv.ParseFloat(c, 32)
			results[ri-1] = append(results[ri-1], evaluator.ResultValue(f))
		}
	}
	return ages, scores, results, nil
}

func main() {
	cwd, _ := os.Getwd()
	if len(cwd) > 0 {
		cwd += "/"
	}
	outDir := cwd + outRelativeDir
	tplDir := cwd + tplRelativeDir
	xlsDir := cwd + sheetRelativeDir
	xlsDirItems, err := os.ReadDir(xlsDir)
	if err != nil {
		log.Fatalln(err)
	}
	tableData := map[evaluator.Exercise]map[evaluator.Gender]map[string]interface{}{}
	for _, item := range xlsDirItems {
		n := item.Name()
		if item.IsDir() || !strings.HasSuffix(n, sheetExtension) {
			continue
		}
		xlsPath := xlsDir + n
		fmt.Println(xlsPath)
		baseName := strings.TrimSuffix(n, sheetExtension)
		exercise := evaluator.Exercise(baseName)
		if exercise.GetName() == "" {
			log.Fatalln("invalid exercise: " + baseName)
		}
		tableData[exercise] = map[evaluator.Gender]map[string]interface{}{}
		name := strcase.ToCamel(baseName)
		for _, gender := range []evaluator.Gender{evaluator.GenderFemale, evaluator.GenderMale} {
			log.Printf("%s (%s)\n", name, exercise.GetResultUnit())
			ages, scores, results, err := processXlsxFile(xlsPath, gender)
			if err != nil {
				log.Fatalln(err)
			}
			tableData[exercise][gender] = map[string]interface{}{
				"Name":    name,
				"name":    baseName,
				"unit":    string(exercise.GetResultUnit()),
				"gender":  gender.Int(),
				"ages":    ages,
				"scores":  scores,
				"results": results,
			}
		}
	}
	tpl, err := template.ParseFiles(tplDir + "table.go.tpl")
	if err != nil {
		log.Fatalln(err)
	}
	func() {
		outFile := outDir + "tables.go"
		f, err := os.Create(outFile)
		if err != nil {
			log.Fatalln(err)
		}
		defer f.Close()
		tpl.Execute(f, tableData)
	}()
}
