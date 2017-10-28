<?php

namespace Hungarofit\Evaluator\Exercise;


use Hungarofit\Evaluator\Unit;
use Hungarofit\Evaluator\Exercise;

final class Motor6Torso extends Exercise
{
    /** Unit of exercise */
    const UNIT_EXERCISE = Unit::COUNT;
    
    /** Unit of result */
    const UNIT_RESULT = Unit::COUNT;
    
    /** Lookup table */
    const TABLE = [
		'f' => [
			'20' => [
				'10.5' => '90',
				'10' => '87',
				'9.5' => '84',
				'9' => '81',
				'8.5' => '78',
				'8' => '75',
				'7.5' => '72',
				'7' => '69',
				'6.5' => '66',
				'6' => '63',
				'5.5' => '61',
				'5' => '58',
				'4.5' => '55',
				'4' => '52',
				'3.5' => '50',
				'3' => '48',
				'2.5' => '46',
				'2' => '44',
				'1.5' => '42',
				'1' => '40',
				'0.5' => '38',
			],
			'19' => [
				'10.5' => '89',
				'10' => '86',
				'9.5' => '83',
				'9' => '80',
				'8.5' => '77',
				'8' => '74',
				'7.5' => '71',
				'7' => '68',
				'6.5' => '65',
				'6' => '62',
				'5.5' => '60',
				'5' => '57',
				'4.5' => '54',
				'4' => '51',
				'3.5' => '49',
				'3' => '47',
				'2.5' => '45',
				'2' => '43',
				'1.5' => '41',
				'1' => '39',
				'0.5' => '37',
			],
			'18' => [
				'10.5' => '88',
				'10' => '85',
				'9.5' => '82',
				'9' => '79',
				'8.5' => '76',
				'8' => '73',
				'7.5' => '70',
				'7' => '67',
				'6.5' => '64',
				'6' => '61',
				'5.5' => '59',
				'5' => '56',
				'4.5' => '53',
				'4' => '50',
				'3.5' => '48',
				'3' => '46',
				'2.5' => '44',
				'2' => '42',
				'1.5' => '40',
				'1' => '38',
				'0.5' => '36',
			],
			'17' => [
				'10.5' => '87',
				'10' => '84',
				'9.5' => '81',
				'9' => '78',
				'8.5' => '75',
				'8' => '72',
				'7.5' => '69',
				'7' => '66',
				'6.5' => '63',
				'6' => '60',
				'5.5' => '58',
				'5' => '55',
				'4.5' => '52',
				'4' => '49',
				'3.5' => '47',
				'3' => '45',
				'2.5' => '43',
				'2' => '41',
				'1.5' => '39',
				'1' => '37',
				'0.5' => '35',
			],
			'16' => [
				'10.5' => '86',
				'10' => '83',
				'9.5' => '80',
				'9' => '77',
				'8.5' => '74',
				'8' => '71',
				'7.5' => '68',
				'7' => '65',
				'6.5' => '62',
				'6' => '59',
				'5.5' => '57',
				'5' => '54',
				'4.5' => '51',
				'4' => '48',
				'3.5' => '46',
				'3' => '44',
				'2.5' => '42',
				'2' => '40',
				'1.5' => '38',
				'1' => '36',
				'0.5' => '34',
			],
			'15' => [
				'10.5' => '85',
				'10' => '82',
				'9.5' => '79',
				'9' => '76',
				'8.5' => '73',
				'8' => '70',
				'7.5' => '67',
				'7' => '64',
				'6.5' => '61',
				'6' => '58',
				'5.5' => '56',
				'5' => '53',
				'4.5' => '50',
				'4' => '47',
				'3.5' => '45',
				'3' => '43',
				'2.5' => '41',
				'2' => '39',
				'1.5' => '37',
				'1' => '35',
				'0.5' => '33',
			],
			'14' => [
				'10.5' => '84',
				'10' => '81',
				'9.5' => '78',
				'9' => '75',
				'8.5' => '72',
				'8' => '69',
				'7.5' => '66',
				'7' => '63',
				'6.5' => '60',
				'6' => '57',
				'5.5' => '55',
				'5' => '52',
				'4.5' => '49',
				'4' => '46',
				'3.5' => '44',
				'3' => '42',
				'2.5' => '40',
				'2' => '38',
				'1.5' => '36',
				'1' => '34',
				'0.5' => '32',
			],
			'13' => [
				'10.5' => '82',
				'10' => '79',
				'9.5' => '76',
				'9' => '73',
				'8.5' => '70',
				'8' => '67',
				'7.5' => '64',
				'7' => '61',
				'6.5' => '58',
				'6' => '55',
				'5.5' => '53',
				'5' => '50',
				'4.5' => '47',
				'4' => '44',
				'3.5' => '42',
				'3' => '40',
				'2.5' => '38',
				'2' => '36',
				'1.5' => '34',
				'1' => '32',
				'0.5' => '30',
			],
			'12' => [
				'10.5' => '80',
				'10' => '77',
				'9.5' => '74',
				'9' => '71',
				'8.5' => '68',
				'8' => '65',
				'7.5' => '62',
				'7' => '59',
				'6.5' => '56',
				'6' => '53',
				'5.5' => '51',
				'5' => '48',
				'4.5' => '45',
				'4' => '42',
				'3.5' => '40',
				'3' => '38',
				'2.5' => '36',
				'2' => '34',
				'1.5' => '32',
				'1' => '30',
				'0.5' => '28',
			],
			'11' => [
				'10.5' => '76',
				'10' => '73',
				'9.5' => '70',
				'9' => '67',
				'8.5' => '64',
				'8' => '61',
				'7.5' => '58',
				'7' => '55',
				'6.5' => '52',
				'6' => '49',
				'5.5' => '46',
				'5' => '43',
				'4.5' => '40',
				'4' => '38',
				'3.5' => '36',
				'3' => '34',
				'2.5' => '32',
				'2' => '30',
				'1.5' => '28',
				'1' => '26',
				'0.5' => '24',
			],
			'10' => [
				'10.5' => '74',
				'10' => '71',
				'9.5' => '68',
				'9' => '65',
				'8.5' => '62',
				'8' => '59',
				'7.5' => '56',
				'7' => '53',
				'6.5' => '50',
				'6' => '47',
				'5.5' => '44',
				'5' => '41',
				'4.5' => '38',
				'4' => '36',
				'3.5' => '34',
				'3' => '32',
				'2.5' => '30',
				'2' => '28',
				'1.5' => '26',
				'1' => '24',
				'0.5' => '22',
			],
			'9' => [
				'10.5' => '73',
				'10' => '70',
				'9.5' => '67',
				'9' => '64',
				'8.5' => '61',
				'8' => '58',
				'7.5' => '55',
				'7' => '52',
				'6.5' => '49',
				'6' => '46',
				'5.5' => '43',
				'5' => '40',
				'4.5' => '37',
				'4' => '35',
				'3.5' => '33',
				'3' => '31',
				'2.5' => '29',
				'2' => '27',
				'1.5' => '25',
				'1' => '23',
				'0.5' => '21',
			],
			'8' => [
				'10.5' => '71',
				'10' => '68',
				'9.5' => '65',
				'9' => '62',
				'8.5' => '59',
				'8' => '56',
				'7.5' => '53',
				'7' => '50',
				'6.5' => '47',
				'6' => '44',
				'5.5' => '41',
				'5' => '38',
				'4.5' => '35',
				'4' => '33',
				'3.5' => '31',
				'3' => '29',
				'2.5' => '27',
				'2' => '25',
				'1.5' => '23',
				'1' => '21',
				'0.5' => '19',
			],
			'7' => [
				'10.5' => '68',
				'10' => '65',
				'9.5' => '62',
				'9' => '59',
				'8.5' => '56',
				'8' => '53',
				'7.5' => '50',
				'7' => '47',
				'6.5' => '44',
				'6' => '41',
				'5.5' => '38',
				'5' => '35',
				'4.5' => '32',
				'4' => '30',
				'3.5' => '28',
				'3' => '26',
				'2.5' => '24',
				'2' => '22',
				'1.5' => '20',
				'1' => '18',
				'0.5' => '16',
			],
		],
		'm' => [
			'20' => [
				'10.5' => '98',
				'10' => '95',
				'9.5' => '92',
				'9' => '89',
				'8.5' => '86',
				'8' => '83',
				'7.5' => '80',
				'7' => '77',
				'6.5' => '74',
				'6' => '71',
				'5.5' => '68',
				'5' => '65',
				'4.5' => '62',
				'4' => '59',
				'3.5' => '56',
				'3' => '53',
				'2.5' => '50',
				'2' => '48',
				'1.5' => '47',
				'1' => '46',
				'0.5' => '45',
			],
			'19' => [
				'10.5' => '96',
				'10' => '93',
				'9.5' => '90',
				'9' => '87',
				'8.5' => '84',
				'8' => '81',
				'7.5' => '78',
				'7' => '75',
				'6.5' => '72',
				'6' => '69',
				'5.5' => '66',
				'5' => '63',
				'4.5' => '60',
				'4' => '57',
				'3.5' => '54',
				'3' => '51',
				'2.5' => '48',
				'2' => '46',
				'1.5' => '45',
				'1' => '44',
				'0.5' => '43',
			],
			'18' => [
				'10.5' => '94',
				'10' => '91',
				'9.5' => '88',
				'9' => '85',
				'8.5' => '82',
				'8' => '79',
				'7.5' => '76',
				'7' => '73',
				'6.5' => '70',
				'6' => '67',
				'5.5' => '64',
				'5' => '61',
				'4.5' => '58',
				'4' => '55',
				'3.5' => '52',
				'3' => '49',
				'2.5' => '46',
				'2' => '45',
				'1.5' => '44',
				'1' => '43',
				'0.5' => '41',
			],
			'17' => [
				'10.5' => '92',
				'10' => '89',
				'9.5' => '86',
				'9' => '83',
				'8.5' => '80',
				'8' => '77',
				'7.5' => '74',
				'7' => '71',
				'6.5' => '68',
				'6' => '65',
				'5.5' => '62',
				'5' => '59',
				'4.5' => '56',
				'4' => '53',
				'3.5' => '50',
				'3' => '47',
				'2.5' => '44',
				'2' => '42',
				'1.5' => '41',
				'1' => '40',
				'0.5' => '39',
			],
			'16' => [
				'10.5' => '90',
				'10' => '87',
				'9.5' => '84',
				'9' => '81',
				'8.5' => '78',
				'8' => '75',
				'7.5' => '72',
				'7' => '69',
				'6.5' => '66',
				'6' => '63',
				'5.5' => '60',
				'5' => '57',
				'4.5' => '54',
				'4' => '51',
				'3.5' => '48',
				'3' => '45',
				'2.5' => '42',
				'2' => '40',
				'1.5' => '39',
				'1' => '38',
				'0.5' => '37',
			],
			'15' => [
				'10.5' => '88',
				'10' => '85',
				'9.5' => '82',
				'9' => '79',
				'8.5' => '76',
				'8' => '73',
				'7.5' => '70',
				'7' => '67',
				'6.5' => '64',
				'6' => '61',
				'5.5' => '58',
				'5' => '55',
				'4.5' => '52',
				'4' => '49',
				'3.5' => '46',
				'3' => '43',
				'2.5' => '40',
				'2' => '38',
				'1.5' => '37',
				'1' => '36',
				'0.5' => '35',
			],
			'14' => [
				'10.5' => '86',
				'10' => '83',
				'9.5' => '80',
				'9' => '77',
				'8.5' => '74',
				'8' => '71',
				'7.5' => '68',
				'7' => '65',
				'6.5' => '62',
				'6' => '59',
				'5.5' => '56',
				'5' => '53',
				'4.5' => '50',
				'4' => '47',
				'3.5' => '44',
				'3' => '41',
				'2.5' => '38',
				'2' => '36',
				'1.5' => '35',
				'1' => '34',
				'0.5' => '33',
			],
			'13' => [
				'10.5' => '84',
				'10' => '81',
				'9.5' => '78',
				'9' => '75',
				'8.5' => '72',
				'8' => '69',
				'7.5' => '66',
				'7' => '63',
				'6.5' => '60',
				'6' => '57',
				'5.5' => '54',
				'5' => '51',
				'4.5' => '48',
				'4' => '45',
				'3.5' => '42',
				'3' => '39',
				'2.5' => '36',
				'2' => '34',
				'1.5' => '33',
				'1' => '32',
				'0.5' => '31',
			],
			'12' => [
				'10.5' => '82',
				'10' => '79',
				'9.5' => '76',
				'9' => '73',
				'8.5' => '70',
				'8' => '67',
				'7.5' => '64',
				'7' => '61',
				'6.5' => '58',
				'6' => '55',
				'5.5' => '52',
				'5' => '49',
				'4.5' => '46',
				'4' => '43',
				'3.5' => '40',
				'3' => '37',
				'2.5' => '34',
				'2' => '33',
				'1.5' => '32',
				'1' => '31',
				'0.5' => '29',
			],
			'11' => [
				'10.5' => '80',
				'10' => '77',
				'9.5' => '74',
				'9' => '71',
				'8.5' => '68',
				'8' => '65',
				'7.5' => '62',
				'7' => '59',
				'6.5' => '56',
				'6' => '53',
				'5.5' => '50',
				'5' => '47',
				'4.5' => '44',
				'4' => '41',
				'3.5' => '38',
				'3' => '35',
				'2.5' => '32',
				'2' => '30',
				'1.5' => '29',
				'1' => '28',
				'0.5' => '27',
			],
			'10' => [
				'10.5' => '78',
				'10' => '75',
				'9.5' => '72',
				'9' => '69',
				'8.5' => '66',
				'8' => '63',
				'7.5' => '60',
				'7' => '57',
				'6.5' => '54',
				'6' => '51',
				'5.5' => '48',
				'5' => '45',
				'4.5' => '42',
				'4' => '39',
				'3.5' => '36',
				'3' => '33',
				'2.5' => '30',
				'2' => '28',
				'1.5' => '27',
				'1' => '26',
				'0.5' => '25',
			],
			'9' => [
				'10.5' => '76',
				'10' => '73',
				'9.5' => '70',
				'9' => '67',
				'8.5' => '64',
				'8' => '61',
				'7.5' => '58',
				'7' => '55',
				'6.5' => '52',
				'6' => '49',
				'5.5' => '46',
				'5' => '43',
				'4.5' => '40',
				'4' => '37',
				'3.5' => '34',
				'3' => '31',
				'2.5' => '28',
				'2' => '26',
				'1.5' => '25',
				'1' => '24',
				'0.5' => '23',
			],
			'8' => [
				'10.5' => '74',
				'10' => '71',
				'9.5' => '68',
				'9' => '65',
				'8.5' => '62',
				'8' => '59',
				'7.5' => '56',
				'7' => '53',
				'6.5' => '50',
				'6' => '47',
				'5.5' => '44',
				'5' => '41',
				'4.5' => '38',
				'4' => '35',
				'3.5' => '32',
				'3' => '29',
				'2.5' => '26',
				'2' => '25',
				'1.5' => '23',
				'1' => '22',
				'0.5' => '21',
			],
			'7' => [
				'10.5' => '72',
				'10' => '69',
				'9.5' => '66',
				'9' => '63',
				'8.5' => '60',
				'8' => '57',
				'7.5' => '54',
				'7' => '51',
				'6.5' => '48',
				'6' => '45',
				'5.5' => '42',
				'5' => '39',
				'4.5' => '36',
				'4' => '33',
				'3.5' => '30',
				'3' => '27',
				'2.5' => '25',
				'2' => '23',
				'1.5' => '22',
				'1' => '20',
				'0.5' => '19',
			],
		],
	];
}