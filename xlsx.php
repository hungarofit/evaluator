<?php
/**
 * Generates Hungarofit\Evaluator\Lookup classes from spreadsheets inside xlsx folder
 */

define('XLS_SHEET_FEMALE', 'lányok');
define('XLS_SHEET_MALE', 'fiúk');
define('XLS_SHEET_SOURCE', __DIR__ . '/xlsx');
define('CLASS_NS', 'Hungarofit\Evaluator\Lookup');
define('CLASS_SOURCE', __DIR__ . '/src/Hungarofit/Evaluator/Lookup');

require_once __DIR__ . '/vendor/autoload.php';

if(!function_exists('readline')) {
    function readline($prompt='') {
        echo $prompt;
        return rtrim(stream_get_line(STDIN, 1024, "\n"), "\r");
    }
}

if(!in_array('-c', $argv)) {
    $confirm = readline('Are you sure you want to regenerate the lookup classes? (N/y)' . PHP_EOL);
    if ($confirm !== 'y' && $confirm !== 'Y') {
        echo 'User abort, exiting', PHP_EOL;
        exit(1);
    }
}

function generateSource($className, $dUnit, $rUnit, $table) {
    $tbl = "[\r\n";
    foreach($table as $gender => $sheet) {
        $tbl.= "\t\t'$gender' => [\r\n";
        foreach($sheet as $age => $row) {
            $tbl.= "\t\t\t'$age' => [\r\n";
            foreach($row as $points => $result) {
                $tbl.= "\t\t\t\t'$points' => '$result',\r\n";
            }
            $tbl.= "\t\t\t],\r\n";
        }
        $tbl.= "\t\t],\r\n";
    }
    $tbl.= "\t]";
    return '<?php

namespace '.CLASS_NS.';


use '.CLASS_NS.';

class '.$className.' extends Lookup
{
    /** Unit of exercise */
    const UNIT_EXERCISE = \''.$dUnit.'\';
    
    /** Unit of result */
    const UNIT_RESULT = \''.$rUnit.'\';
    
    /** Lookup table */
    const TABLE = '.$tbl.';
}
';
}


$SRC_PATH = realpath(__DIR__ . '/src/Hungarofit/Evaluator/Lookup') . DIRECTORY_SEPARATOR;

$xlsIterator = new \DirectoryIterator(__DIR__ . '/xlsx/');
foreach($xlsIterator as $xlsPath) {
    if($xlsPath->getExtension() !== 'xlsx') {
        continue;
    }

    $table = [];
    $kebabName = $xlsPath->getBasename('.xlsx');
    $className = Hungarofit\Evaluator\Text::camelcase($kebabName);

    $split = explode('-', $kebabName);
    $t = array_shift($split);
    $x = array_pop($split);
    switch ($t) {
        case 'aerob':
            switch ($x) {
                case '500m':
                    $dUnit = 'm';
                    $rUnit = 'min';
                    break;
                case '12min':
                case '6min':
                    $dUnit = 'min';
                    $rUnit = 'm';
                    break;
                case '1mile':
                case '1mile5':
                case '2mile':
                    $dUnit = 'mile';
                    $rUnit = 'min';
                    break;
                case '2km':
                case '3km':
                    $dUnit = 'km';
                    $rUnit = 'min';
                    break;
                default:
                    throw new RuntimeException('Invalid motor exercise unit: '.$kebabName.' / '.$x);
                    break;
            }
            break;
        case 'motor':
            switch($x) {
                case 'jump':
                case 'throwsingle':
                case 'throwdouble':
                    $dUnit = 'n';
                    $rUnit = 'm';
                    break;
                case 'situp':
                case 'torso':
                case 'pushup':
                    $dUnit = 'n';
                    $rUnit = 'n';
                    break;
                default:
                    throw new RuntimeException('Invalid motor exercise type: '.$kebabName.' / '.$x);
            }
            break;
        default:
            throw new RuntimeException('Invalid exercise xlsx: '.$kebabName);
    }

    $xlsReader = new SpreadsheetReader_XLSX($xlsPath->getPathname());

    // Each sheet is a gender
    foreach($xlsReader->Sheets() as $sheetIndex => $sheetName) {

        // Check sheet name for gender
        switch ($sheetName) {
            case XLS_SHEET_FEMALE:
                $gender = Hungarofit\Evaluator\Gender::FEMALE;
                break;
            case XLS_SHEET_MALE:
                $gender = Hungarofit\Evaluator\Gender::MALE;
                break;
            default:
                throw new RuntimeException('Invalid sheet name: '.$sheetName);
        }
        if(!$xlsReader->ChangeSheet($sheetIndex)) {
            throw new RuntimeException('Failed to change sheet to: '.$sheetName.' #'.$sheetIndex.' in file '.$xlsIterator->getFilename());
        }

        // Fill table for gender
        $genderTable = [];
        $ages = [];
        $colCount = 0;
        foreach($xlsReader as $row) {
            if($colCount < 1) {
                $colCount = count($row);
                array_shift($row);
                $ages = $row;
                continue;
            }
            $points = array_shift($row);
            foreach($row as $ri => $result) {
                if(floatval($result) <= 0) {
                    continue;
                }
                $genderTable["$ages[$ri]"]["$points"] = ''.round($result, 2);
            }
        }

        // Lookup expects data in descending order
        $genderTable = array_reverse($genderTable, true);
        foreach($genderTable as $i => $row) {
            $genderTable[$i] = array_reverse($row, true);
        }

        $table["$gender"] = $genderTable;
    }
    file_put_contents($SRC_PATH . $className . '.php', generateSource($className, $dUnit, $rUnit, $table));
    echo $xlsIterator->getFilename(), ' done.', PHP_EOL;
}