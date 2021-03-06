<?php

namespace Hungarofit\Evaluator\Exercise;


use Hungarofit\Evaluator\Unit;
use Hungarofit\Evaluator\Exercise;

final class Motor4Situp extends Exercise
{
    /** Unit of exercise */
    const UNIT_EXERCISE = Unit::COUNT;
    
    /** Unit of result */
    const UNIT_RESULT = Unit::COUNT;
    
    /** Lookup table */
    const TABLE = [
		'f' => [
			'76' => [
				'15' => '16',
				'14' => '15',
				'13' => '14',
				'12' => '13',
				'11' => '12',
				'10' => '11',
				'9' => '10',
				'8' => '9',
				'7' => '8',
				'6' => '7',
				'5' => '6',
				'4' => '5',
				'3' => '4',
				'2' => '3',
				'1' => '2',
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
				'15' => '42',
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
				'14' => '115',
				'13' => '110',
				'12' => '105',
				'11' => '100',
				'10' => '95',
				'9' => '90',
				'8' => '85',
				'7' => '80',
				'6' => '75',
				'5' => '70',
				'4' => '65',
				'3' => '60',
				'2' => '55',
				'1' => '50',
			],
			'19' => [
				'14' => '113',
				'13' => '108',
				'12' => '103',
				'11' => '98',
				'10' => '93',
				'9' => '88',
				'8' => '83',
				'7' => '78',
				'6' => '73',
				'5' => '68',
				'4' => '63',
				'3' => '58',
				'2' => '53',
				'1' => '48',
			],
			'18' => [
				'14' => '110',
				'13' => '105',
				'12' => '100',
				'11' => '95',
				'10' => '90',
				'9' => '85',
				'8' => '80',
				'7' => '75',
				'6' => '70',
				'5' => '65',
				'4' => '60',
				'3' => '55',
				'2' => '50',
				'1' => '45',
			],
			'17' => [
				'14' => '108',
				'13' => '103',
				'12' => '98',
				'11' => '93',
				'10' => '88',
				'9' => '83',
				'8' => '78',
				'7' => '73',
				'6' => '68',
				'5' => '63',
				'4' => '58',
				'3' => '53',
				'2' => '48',
				'1' => '43',
			],
			'16' => [
				'14' => '106',
				'13' => '101',
				'12' => '96',
				'11' => '91',
				'10' => '86',
				'9' => '81',
				'8' => '76',
				'7' => '71',
				'6' => '66',
				'5' => '61',
				'4' => '56',
				'3' => '51',
				'2' => '46',
				'1' => '41',
			],
			'15' => [
				'14' => '104',
				'13' => '99',
				'12' => '94',
				'11' => '89',
				'10' => '84',
				'9' => '79',
				'8' => '74',
				'7' => '69',
				'6' => '64',
				'5' => '59',
				'4' => '54',
				'3' => '49',
				'2' => '44',
				'1' => '39',
			],
			'14' => [
				'14' => '102',
				'13' => '97',
				'12' => '92',
				'11' => '87',
				'10' => '82',
				'9' => '77',
				'8' => '72',
				'7' => '67',
				'6' => '62',
				'5' => '57',
				'4' => '52',
				'3' => '47',
				'2' => '42',
				'1' => '37',
			],
			'13' => [
				'14' => '100',
				'13' => '95',
				'12' => '90',
				'11' => '85',
				'10' => '80',
				'9' => '75',
				'8' => '70',
				'7' => '65',
				'6' => '60',
				'5' => '55',
				'4' => '50',
				'3' => '45',
				'2' => '40',
				'1' => '35',
			],
			'12' => [
				'14' => '98',
				'13' => '93',
				'12' => '88',
				'11' => '83',
				'10' => '78',
				'9' => '73',
				'8' => '68',
				'7' => '63',
				'6' => '58',
				'5' => '53',
				'4' => '48',
				'3' => '43',
				'2' => '38',
				'1' => '33',
			],
			'11' => [
				'14' => '96',
				'13' => '91',
				'12' => '86',
				'11' => '81',
				'10' => '76',
				'9' => '71',
				'8' => '66',
				'7' => '61',
				'6' => '56',
				'5' => '51',
				'4' => '46',
				'3' => '41',
				'2' => '36',
				'1' => '32',
			],
			'10' => [
				'14' => '94',
				'13' => '89',
				'12' => '84',
				'11' => '79',
				'10' => '74',
				'9' => '69',
				'8' => '64',
				'7' => '59',
				'6' => '54',
				'5' => '49',
				'4' => '44',
				'3' => '39',
				'2' => '34',
				'1' => '29',
			],
			'9' => [
				'14' => '91',
				'13' => '86',
				'12' => '81',
				'11' => '76',
				'10' => '71',
				'9' => '66',
				'8' => '61',
				'7' => '56',
				'6' => '51',
				'5' => '46',
				'4' => '41',
				'3' => '36',
				'2' => '31',
				'1' => '26',
			],
			'8' => [
				'14' => '88',
				'13' => '83',
				'12' => '78',
				'11' => '73',
				'10' => '68',
				'9' => '63',
				'8' => '58',
				'7' => '53',
				'6' => '48',
				'5' => '43',
				'4' => '38',
				'3' => '33',
				'2' => '28',
				'1' => '23',
			],
			'7' => [
				'14' => '85',
				'13' => '80',
				'12' => '75',
				'11' => '70',
				'10' => '65',
				'9' => '60',
				'8' => '55',
				'7' => '50',
				'6' => '45',
				'5' => '40',
				'4' => '35',
				'3' => '30',
				'2' => '25',
				'1' => '20',
			],
			'6' => [
				'14' => '64',
				'13' => '60',
				'12' => '56',
				'11' => '52',
				'10' => '48',
				'9' => '44',
				'8' => '40',
				'7' => '36',
				'6' => '32',
				'5' => '28',
				'4' => '24',
				'3' => '20',
				'2' => '16',
				'1' => '12',
			],
			'5' => [
				'14' => '60',
				'13' => '56',
				'12' => '52',
				'11' => '48',
				'10' => '44',
				'9' => '40',
				'8' => '36',
				'7' => '32',
				'6' => '28',
				'5' => '24',
				'4' => '20',
				'3' => '16',
				'2' => '12',
				'1' => '8',
			],
			'4' => [
				'14' => '37',
				'13' => '35',
				'12' => '33',
				'11' => '30',
				'10' => '28',
				'9' => '25',
				'8' => '22',
				'7' => '19',
				'6' => '16',
				'5' => '13',
				'4' => '11',
				'3' => '9',
				'2' => '7',
				'1' => '5',
			],
		],
		'm' => [
			'76' => [
				'15' => '30',
				'14' => '28',
				'13' => '26',
				'12' => '24',
				'11' => '22',
				'10' => '20',
				'9' => '18',
				'8' => '16',
				'7' => '14',
				'6' => '12',
				'5' => '10',
				'4' => '8',
				'3' => '6',
				'2' => '4',
				'1' => '2',
			],
			'66' => [
				'15' => '39',
				'14' => '37',
				'13' => '35',
				'12' => '33',
				'11' => '30',
				'10' => '27',
				'9' => '25',
				'8' => '23',
				'7' => '21',
				'6' => '19',
				'5' => '17',
				'4' => '15',
				'3' => '13',
				'2' => '11',
				'1' => '9',
			],
			'56' => [
				'15' => '44',
				'14' => '42',
				'13' => '40',
				'12' => '38',
				'11' => '36',
				'10' => '34',
				'9' => '32',
				'8' => '30',
				'7' => '28',
				'6' => '26',
				'5' => '24',
				'4' => '21',
				'3' => '18',
				'2' => '16',
				'1' => '14',
			],
			'46' => [
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
			'36' => [
				'15' => '54',
				'14' => '52',
				'13' => '50',
				'12' => '48',
				'11' => '46',
				'10' => '44',
				'9' => '42',
				'8' => '40',
				'7' => '38',
				'6' => '36',
				'5' => '34',
				'4' => '32',
				'3' => '30',
				'2' => '28',
				'1' => '26',
			],
			'26' => [
				'15' => '59',
				'14' => '57',
				'13' => '55',
				'12' => '53',
				'11' => '51',
				'10' => '49',
				'9' => '47',
				'8' => '45',
				'7' => '43',
				'6' => '41',
				'5' => '39',
				'4' => '37',
				'3' => '35',
				'2' => '33',
				'1' => '31',
			],
			'20' => [
				'14' => '119',
				'13' => '114',
				'12' => '109',
				'11' => '104',
				'10' => '99',
				'9' => '94',
				'8' => '89',
				'7' => '84',
				'6' => '79',
				'5' => '74',
				'4' => '69',
				'3' => '64',
				'2' => '59',
				'1' => '54',
			],
			'19' => [
				'14' => '117',
				'13' => '112',
				'12' => '107',
				'11' => '102',
				'10' => '97',
				'9' => '92',
				'8' => '87',
				'7' => '82',
				'6' => '77',
				'5' => '72',
				'4' => '67',
				'3' => '62',
				'2' => '57',
				'1' => '52',
			],
			'18' => [
				'14' => '115',
				'13' => '110',
				'12' => '105',
				'11' => '100',
				'10' => '95',
				'9' => '90',
				'8' => '85',
				'7' => '80',
				'6' => '75',
				'5' => '70',
				'4' => '65',
				'3' => '60',
				'2' => '55',
				'1' => '50',
			],
			'17' => [
				'14' => '113',
				'13' => '108',
				'12' => '103',
				'11' => '98',
				'10' => '93',
				'9' => '88',
				'8' => '83',
				'7' => '78',
				'6' => '73',
				'5' => '68',
				'4' => '63',
				'3' => '58',
				'2' => '53',
				'1' => '48',
			],
			'16' => [
				'14' => '111',
				'13' => '106',
				'12' => '101',
				'11' => '96',
				'10' => '91',
				'9' => '86',
				'8' => '81',
				'7' => '76',
				'6' => '71',
				'5' => '66',
				'4' => '61',
				'3' => '56',
				'2' => '51',
				'1' => '46',
			],
			'15' => [
				'14' => '109',
				'13' => '104',
				'12' => '99',
				'11' => '94',
				'10' => '89',
				'9' => '84',
				'8' => '79',
				'7' => '74',
				'6' => '69',
				'5' => '64',
				'4' => '59',
				'3' => '54',
				'2' => '49',
				'1' => '44',
			],
			'14' => [
				'14' => '107',
				'13' => '102',
				'12' => '97',
				'11' => '92',
				'10' => '87',
				'9' => '82',
				'8' => '77',
				'7' => '72',
				'6' => '67',
				'5' => '62',
				'4' => '57',
				'3' => '52',
				'2' => '47',
				'1' => '42',
			],
			'13' => [
				'14' => '105',
				'13' => '100',
				'12' => '95',
				'11' => '90',
				'10' => '85',
				'9' => '80',
				'8' => '75',
				'7' => '70',
				'6' => '65',
				'5' => '60',
				'4' => '55',
				'3' => '50',
				'2' => '45',
				'1' => '40',
			],
			'12' => [
				'14' => '103',
				'13' => '98',
				'12' => '93',
				'11' => '88',
				'10' => '83',
				'9' => '78',
				'8' => '73',
				'7' => '68',
				'6' => '63',
				'5' => '58',
				'4' => '53',
				'3' => '48',
				'2' => '43',
				'1' => '38',
			],
			'11' => [
				'14' => '101',
				'13' => '96',
				'12' => '91',
				'11' => '86',
				'10' => '81',
				'9' => '76',
				'8' => '71',
				'7' => '66',
				'6' => '61',
				'5' => '56',
				'4' => '51',
				'3' => '46',
				'2' => '41',
				'1' => '36',
			],
			'10' => [
				'14' => '99',
				'13' => '94',
				'12' => '89',
				'11' => '84',
				'10' => '79',
				'9' => '74',
				'8' => '69',
				'7' => '64',
				'6' => '59',
				'5' => '54',
				'4' => '49',
				'3' => '44',
				'2' => '39',
				'1' => '34',
			],
			'9' => [
				'14' => '96',
				'13' => '91',
				'12' => '86',
				'11' => '81',
				'10' => '76',
				'9' => '71',
				'8' => '66',
				'7' => '61',
				'6' => '56',
				'5' => '51',
				'4' => '46',
				'3' => '41',
				'2' => '36',
				'1' => '31',
			],
			'8' => [
				'14' => '93',
				'13' => '88',
				'12' => '83',
				'11' => '78',
				'10' => '73',
				'9' => '68',
				'8' => '63',
				'7' => '58',
				'6' => '53',
				'5' => '48',
				'4' => '43',
				'3' => '38',
				'2' => '33',
				'1' => '28',
			],
			'7' => [
				'14' => '90',
				'13' => '85',
				'12' => '80',
				'11' => '75',
				'10' => '70',
				'9' => '65',
				'8' => '60',
				'7' => '55',
				'6' => '50',
				'5' => '45',
				'4' => '40',
				'3' => '35',
				'2' => '30',
				'1' => '25',
			],
			'6' => [
				'14' => '67',
				'13' => '63',
				'12' => '59',
				'11' => '55',
				'10' => '51',
				'9' => '47',
				'8' => '43',
				'7' => '39',
				'6' => '35',
				'5' => '31',
				'4' => '27',
				'3' => '23',
				'2' => '19',
				'1' => '15',
			],
			'5' => [
				'14' => '61',
				'13' => '57',
				'12' => '53',
				'11' => '49',
				'10' => '45',
				'9' => '41',
				'8' => '37',
				'7' => '33',
				'6' => '29',
				'5' => '25',
				'4' => '21',
				'3' => '17',
				'2' => '13',
				'1' => '9',
			],
			'4' => [
				'14' => '39',
				'13' => '37',
				'12' => '35',
				'11' => '33',
				'10' => '30',
				'9' => '28',
				'8' => '25',
				'7' => '22',
				'6' => '19',
				'5' => '16',
				'4' => '13',
				'3' => '11',
				'2' => '9',
				'1' => '7',
			],
		],
	];
}
