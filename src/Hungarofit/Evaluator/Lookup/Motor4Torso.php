<?php

namespace Hungarofit\Evaluator\Lookup;


use Hungarofit\Evaluator\Unit;
use Hungarofit\Evaluator\Lookup;

final class Motor4Torso extends Lookup
{
    /** Unit of exercise */
    const UNIT_EXERCISE = Unit::COUNT;
    
    /** Unit of result */
    const UNIT_RESULT = Unit::COUNT;
    
    /** Lookup table */
    const TABLE = [
		'f' => [
			'76' => [
				'15' => '23',
				'14' => '21',
				'13' => '19',
				'12' => '17',
				'11' => '15',
				'10' => '13',
				'9' => '11',
				'8' => '9',
				'7' => '7',
				'6' => '6',
				'5' => '5',
				'4' => '4',
				'3' => '3',
				'2' => '2',
				'1' => '1',
			],
			'66' => [
				'15' => '29',
				'14' => '27',
				'13' => '25',
				'12' => '23',
				'11' => '21',
				'10' => '19',
				'9' => '17',
				'8' => '15',
				'7' => '13',
				'6' => '11',
				'5' => '9',
				'4' => '7',
				'3' => '6',
				'2' => '5',
				'1' => '4',
			],
			'56' => [
				'15' => '36',
				'14' => '34',
				'13' => '32',
				'12' => '30',
				'11' => '28',
				'10' => '26',
				'9' => '24',
				'8' => '22',
				'7' => '20',
				'6' => '18',
				'5' => '16',
				'4' => '14',
				'3' => '12',
				'2' => '10',
				'1' => '8',
			],
			'46' => [
				'15' => '43',
				'14' => '40',
				'13' => '38',
				'12' => '36',
				'11' => '34',
				'10' => '32',
				'9' => '30',
				'8' => '28',
				'7' => '26',
				'6' => '24',
				'5' => '22',
				'4' => '20',
				'3' => '18',
				'2' => '16',
				'1' => '14',
			],
			'36' => [
				'15' => '45',
				'14' => '43',
				'13' => '41',
				'12' => '39',
				'11' => '37',
				'10' => '35',
				'9' => '33',
				'8' => '31',
				'7' => '29',
				'6' => '27',
				'5' => '25',
				'4' => '23',
				'3' => '21',
				'2' => '19',
				'1' => '17',
			],
			'26' => [
				'15' => '49',
				'14' => '47',
				'13' => '45',
				'12' => '43',
				'11' => '41',
				'10' => '39',
				'9' => '37',
				'8' => '35',
				'7' => '33',
				'6' => '31',
				'5' => '29',
				'4' => '27',
				'3' => '25',
				'2' => '23',
				'1' => '21',
			],
			'20' => [
				'14' => '90',
				'13' => '86',
				'12' => '82',
				'11' => '78',
				'10' => '74',
				'9' => '70',
				'8' => '66',
				'7' => '62',
				'6' => '58',
				'5' => '54',
				'4' => '50',
				'3' => '46',
				'2' => '42',
				'1' => '38',
			],
			'19' => [
				'14' => '89',
				'13' => '85',
				'12' => '81',
				'11' => '77',
				'10' => '73',
				'9' => '69',
				'8' => '65',
				'7' => '61',
				'6' => '57',
				'5' => '53',
				'4' => '49',
				'3' => '45',
				'2' => '41',
				'1' => '37',
			],
			'18' => [
				'14' => '88',
				'13' => '84',
				'12' => '80',
				'11' => '76',
				'10' => '72',
				'9' => '68',
				'8' => '64',
				'7' => '60',
				'6' => '56',
				'5' => '52',
				'4' => '48',
				'3' => '44',
				'2' => '40',
				'1' => '36',
			],
			'17' => [
				'14' => '87',
				'13' => '83',
				'12' => '79',
				'11' => '75',
				'10' => '71',
				'9' => '67',
				'8' => '63',
				'7' => '59',
				'6' => '55',
				'5' => '51',
				'4' => '47',
				'3' => '43',
				'2' => '39',
				'1' => '35',
			],
			'16' => [
				'14' => '86',
				'13' => '82',
				'12' => '78',
				'11' => '74',
				'10' => '70',
				'9' => '66',
				'8' => '62',
				'7' => '58',
				'6' => '54',
				'5' => '50',
				'4' => '46',
				'3' => '42',
				'2' => '38',
				'1' => '34',
			],
			'15' => [
				'14' => '85',
				'13' => '81',
				'12' => '77',
				'11' => '73',
				'10' => '69',
				'9' => '65',
				'8' => '61',
				'7' => '57',
				'6' => '53',
				'5' => '49',
				'4' => '45',
				'3' => '41',
				'2' => '37',
				'1' => '33',
			],
			'14' => [
				'14' => '84',
				'13' => '80',
				'12' => '76',
				'11' => '72',
				'10' => '68',
				'9' => '64',
				'8' => '60',
				'7' => '56',
				'6' => '52',
				'5' => '48',
				'4' => '44',
				'3' => '40',
				'2' => '36',
				'1' => '32',
			],
			'13' => [
				'14' => '82',
				'13' => '78',
				'12' => '74',
				'11' => '70',
				'10' => '66',
				'9' => '62',
				'8' => '58',
				'7' => '54',
				'6' => '50',
				'5' => '46',
				'4' => '42',
				'3' => '38',
				'2' => '34',
				'1' => '30',
			],
			'12' => [
				'14' => '82',
				'13' => '77',
				'12' => '73',
				'11' => '69',
				'10' => '65',
				'9' => '61',
				'8' => '57',
				'7' => '53',
				'6' => '49',
				'5' => '45',
				'4' => '41',
				'3' => '37',
				'2' => '33',
				'1' => '29',
			],
			'11' => [
				'14' => '80',
				'13' => '76',
				'12' => '72',
				'11' => '68',
				'10' => '64',
				'9' => '60',
				'8' => '56',
				'7' => '52',
				'6' => '48',
				'5' => '44',
				'4' => '40',
				'3' => '36',
				'2' => '32',
				'1' => '28',
			],
			'10' => [
				'14' => '76',
				'13' => '72',
				'12' => '68',
				'11' => '64',
				'10' => '60',
				'9' => '56',
				'8' => '52',
				'7' => '48',
				'6' => '44',
				'5' => '40',
				'4' => '36',
				'3' => '32',
				'2' => '28',
				'1' => '24',
			],
			'9' => [
				'14' => '74',
				'13' => '70',
				'12' => '66',
				'11' => '62',
				'10' => '58',
				'9' => '56',
				'8' => '50',
				'7' => '46',
				'6' => '42',
				'5' => '38',
				'4' => '34',
				'3' => '30',
				'2' => '26',
				'1' => '22',
			],
			'8' => [
				'14' => '71',
				'13' => '67',
				'12' => '63',
				'11' => '59',
				'10' => '55',
				'9' => '51',
				'8' => '47',
				'7' => '43',
				'6' => '39',
				'5' => '35',
				'4' => '31',
				'3' => '27',
				'2' => '23',
				'1' => '19',
			],
			'7' => [
				'14' => '68',
				'13' => '64',
				'12' => '60',
				'11' => '56',
				'10' => '52',
				'9' => '48',
				'8' => '44',
				'7' => '40',
				'6' => '36',
				'5' => '32',
				'4' => '28',
				'3' => '24',
				'2' => '20',
				'1' => '16',
			],
		],
		'm' => [
			'76' => [
				'15' => '16',
				'14' => '15',
				'13' => '14',
				'12' => '13',
				'11' => '12',
				'10' => '11',
				'9' => '10',
				'8' => '9',
				'7' => '7',
				'6' => '6',
				'5' => '5',
				'4' => '4',
				'3' => '3',
				'2' => '2',
				'1' => '1',
			],
			'66' => [
				'15' => '24',
				'14' => '22',
				'13' => '20',
				'12' => '18',
				'11' => '16',
				'10' => '14',
				'9' => '12',
				'8' => '10',
				'7' => '8',
				'6' => '7',
				'5' => '6',
				'4' => '5',
				'3' => '4',
				'2' => '3',
				'1' => '2',
			],
			'56' => [
				'15' => '38',
				'14' => '36',
				'13' => '34',
				'12' => '32',
				'11' => '30',
				'10' => '28',
				'9' => '26',
				'8' => '24',
				'7' => '22',
				'6' => '20',
				'5' => '18',
				'4' => '16',
				'3' => '14',
				'2' => '12',
				'1' => '10',
			],
			'46' => [
				'15' => '45',
				'14' => '42',
				'13' => '39',
				'12' => '37',
				'11' => '35',
				'10' => '33',
				'9' => '31',
				'8' => '29',
				'7' => '27',
				'6' => '25',
				'5' => '23',
				'4' => '21',
				'3' => '19',
				'2' => '17',
				'1' => '15',
			],
			'36' => [
				'15' => '47',
				'14' => '45',
				'13' => '43',
				'12' => '41',
				'11' => '39',
				'10' => '37',
				'9' => '35',
				'8' => '33',
				'7' => '31',
				'6' => '29',
				'5' => '27',
				'4' => '25',
				'3' => '23',
				'2' => '21',
				'1' => '19',
			],
			'26' => [
				'15' => '51',
				'14' => '49',
				'13' => '47',
				'12' => '45',
				'11' => '43',
				'10' => '41',
				'9' => '39',
				'8' => '37',
				'7' => '35',
				'6' => '33',
				'5' => '31',
				'4' => '29',
				'3' => '27',
				'2' => '25',
				'1' => '23',
			],
			'20' => [
				'14' => '98',
				'13' => '94',
				'12' => '90',
				'11' => '86',
				'10' => '82',
				'9' => '78',
				'8' => '74',
				'7' => '70',
				'6' => '66',
				'5' => '62',
				'4' => '58',
				'3' => '54',
				'2' => '50',
				'1' => '46',
			],
			'19' => [
				'14' => '96',
				'13' => '92',
				'12' => '88',
				'11' => '84',
				'10' => '80',
				'9' => '76',
				'8' => '72',
				'7' => '68',
				'6' => '64',
				'5' => '60',
				'4' => '56',
				'3' => '52',
				'2' => '48',
				'1' => '44',
			],
			'18' => [
				'14' => '94',
				'13' => '90',
				'12' => '86',
				'11' => '82',
				'10' => '78',
				'9' => '74',
				'8' => '70',
				'7' => '66',
				'6' => '62',
				'5' => '58',
				'4' => '54',
				'3' => '50',
				'2' => '46',
				'1' => '42',
			],
			'17' => [
				'14' => '92',
				'13' => '88',
				'12' => '84',
				'11' => '80',
				'10' => '76',
				'9' => '72',
				'8' => '68',
				'7' => '64',
				'6' => '60',
				'5' => '56',
				'4' => '52',
				'3' => '48',
				'2' => '44',
				'1' => '40',
			],
			'16' => [
				'14' => '90',
				'13' => '86',
				'12' => '82',
				'11' => '78',
				'10' => '74',
				'9' => '70',
				'8' => '66',
				'7' => '62',
				'6' => '58',
				'5' => '54',
				'4' => '50',
				'3' => '46',
				'2' => '42',
				'1' => '38',
			],
			'15' => [
				'14' => '88',
				'13' => '84',
				'12' => '80',
				'11' => '76',
				'10' => '72',
				'9' => '68',
				'8' => '64',
				'7' => '60',
				'6' => '56',
				'5' => '52',
				'4' => '48',
				'3' => '44',
				'2' => '40',
				'1' => '36',
			],
			'14' => [
				'14' => '86',
				'13' => '82',
				'12' => '78',
				'11' => '74',
				'10' => '70',
				'9' => '66',
				'8' => '62',
				'7' => '58',
				'6' => '54',
				'5' => '50',
				'4' => '46',
				'3' => '42',
				'2' => '38',
				'1' => '34',
			],
			'13' => [
				'14' => '84',
				'13' => '80',
				'12' => '76',
				'11' => '72',
				'10' => '68',
				'9' => '64',
				'8' => '60',
				'7' => '56',
				'6' => '52',
				'5' => '48',
				'4' => '44',
				'3' => '40',
				'2' => '36',
				'1' => '32',
			],
			'12' => [
				'14' => '82',
				'13' => '78',
				'12' => '74',
				'11' => '70',
				'10' => '66',
				'9' => '62',
				'8' => '58',
				'7' => '54',
				'6' => '50',
				'5' => '46',
				'4' => '42',
				'3' => '38',
				'2' => '34',
				'1' => '30',
			],
			'11' => [
				'14' => '80',
				'13' => '76',
				'12' => '72',
				'11' => '68',
				'10' => '64',
				'9' => '60',
				'8' => '56',
				'7' => '52',
				'6' => '48',
				'5' => '44',
				'4' => '40',
				'3' => '36',
				'2' => '32',
				'1' => '28',
			],
			'10' => [
				'14' => '78',
				'13' => '74',
				'12' => '70',
				'11' => '66',
				'10' => '62',
				'9' => '58',
				'8' => '54',
				'7' => '50',
				'6' => '46',
				'5' => '42',
				'4' => '38',
				'3' => '34',
				'2' => '30',
				'1' => '26',
			],
			'9' => [
				'14' => '76',
				'13' => '72',
				'12' => '68',
				'11' => '64',
				'10' => '60',
				'9' => '56',
				'8' => '52',
				'7' => '48',
				'6' => '44',
				'5' => '40',
				'4' => '36',
				'3' => '32',
				'2' => '28',
				'1' => '24',
			],
			'8' => [
				'14' => '74',
				'13' => '70',
				'12' => '66',
				'11' => '62',
				'10' => '58',
				'9' => '54',
				'8' => '50',
				'7' => '46',
				'6' => '42',
				'5' => '38',
				'4' => '34',
				'3' => '30',
				'2' => '26',
				'1' => '22',
			],
			'7' => [
				'14' => '72',
				'13' => '68',
				'12' => '64',
				'11' => '60',
				'10' => '56',
				'9' => '52',
				'8' => '48',
				'7' => '44',
				'6' => '40',
				'5' => '36',
				'4' => '32',
				'3' => '28',
				'2' => '24',
				'1' => '20',
			],
		],
	];
}
