<?php

namespace Hungarofit\Evaluator\Lookup;


use Hungarofit\Evaluator\Lookup;

final class Motor3Situp extends Lookup
{
    /** Unit of exercise */
    const UNIT_EXERCISE = 'n';
    
    /** Unit of result */
    const UNIT_RESULT = 'n';
    
    /** Lookup table */
    const TABLE = [
		'f' => [
			'20' => [
				'21' => '114',
				'20' => '111',
				'19' => '108',
				'18' => '105',
				'17' => '102',
				'16' => '99',
				'15' => '96',
				'14' => '93',
				'13' => '90',
				'12' => '87',
				'11' => '84',
				'10' => '81',
				'9' => '78',
				'8' => '75',
				'7' => '72',
				'6' => '68',
				'5' => '64',
				'4' => '60',
				'3' => '56',
				'2' => '52',
				'1' => '48',
			],
			'19' => [
				'21' => '112',
				'20' => '109',
				'19' => '106',
				'18' => '103',
				'17' => '100',
				'16' => '97',
				'15' => '94',
				'14' => '91',
				'13' => '88',
				'12' => '85',
				'11' => '82',
				'10' => '79',
				'9' => '76',
				'8' => '73',
				'7' => '70',
				'6' => '66',
				'5' => '62',
				'4' => '58',
				'3' => '54',
				'2' => '50',
				'1' => '46',
			],
			'18' => [
				'21' => '110',
				'20' => '107',
				'19' => '104',
				'18' => '101',
				'17' => '98',
				'16' => '95',
				'15' => '92',
				'14' => '89',
				'13' => '86',
				'12' => '83',
				'11' => '80',
				'10' => '77',
				'9' => '74',
				'8' => '71',
				'7' => '68',
				'6' => '64',
				'5' => '60',
				'4' => '56',
				'3' => '52',
				'2' => '48',
				'1' => '44',
			],
			'17' => [
				'21' => '108',
				'20' => '105',
				'19' => '102',
				'18' => '99',
				'17' => '96',
				'16' => '93',
				'15' => '90',
				'14' => '87',
				'13' => '84',
				'12' => '81',
				'11' => '78',
				'10' => '75',
				'9' => '72',
				'8' => '69',
				'7' => '65',
				'6' => '61',
				'5' => '57',
				'4' => '53',
				'3' => '49',
				'2' => '45',
				'1' => '42',
			],
			'16' => [
				'21' => '106',
				'20' => '103',
				'19' => '100',
				'18' => '97',
				'17' => '94',
				'16' => '91',
				'15' => '88',
				'14' => '85',
				'13' => '82',
				'12' => '79',
				'11' => '76',
				'10' => '73',
				'9' => '70',
				'8' => '67',
				'7' => '64',
				'6' => '60',
				'5' => '56',
				'4' => '52',
				'3' => '48',
				'2' => '44',
				'1' => '40',
			],
			'15' => [
				'21' => '104',
				'20' => '101',
				'19' => '98',
				'18' => '95',
				'17' => '92',
				'16' => '89',
				'15' => '86',
				'14' => '83',
				'13' => '80',
				'12' => '77',
				'11' => '74',
				'10' => '71',
				'9' => '68',
				'8' => '65',
				'7' => '62',
				'6' => '58',
				'5' => '54',
				'4' => '50',
				'3' => '46',
				'2' => '42',
				'1' => '38',
			],
			'14' => [
				'21' => '102',
				'20' => '99',
				'19' => '96',
				'18' => '93',
				'17' => '90',
				'16' => '87',
				'15' => '84',
				'14' => '81',
				'13' => '78',
				'12' => '75',
				'11' => '72',
				'10' => '69',
				'9' => '66',
				'8' => '63',
				'7' => '60',
				'6' => '56',
				'5' => '52',
				'4' => '58',
				'3' => '44',
				'2' => '40',
				'1' => '36',
			],
			'13' => [
				'21' => '100',
				'20' => '97',
				'19' => '94',
				'18' => '91',
				'17' => '88',
				'16' => '85',
				'15' => '82',
				'14' => '79',
				'13' => '76',
				'12' => '73',
				'11' => '70',
				'10' => '67',
				'9' => '64',
				'8' => '61',
				'7' => '58',
				'6' => '54',
				'5' => '50',
				'4' => '46',
				'3' => '42',
				'2' => '38',
				'1' => '34',
			],
			'12' => [
				'21' => '98',
				'20' => '95',
				'19' => '92',
				'18' => '89',
				'17' => '86',
				'16' => '83',
				'15' => '80',
				'14' => '77',
				'13' => '74',
				'12' => '71',
				'11' => '68',
				'10' => '65',
				'9' => '62',
				'8' => '59',
				'7' => '56',
				'6' => '52',
				'5' => '48',
				'4' => '44',
				'3' => '40',
				'2' => '36',
				'1' => '32',
			],
			'11' => [
				'21' => '96',
				'20' => '93',
				'19' => '90',
				'18' => '87',
				'17' => '84',
				'16' => '81',
				'15' => '78',
				'14' => '75',
				'13' => '72',
				'12' => '69',
				'11' => '66',
				'10' => '63',
				'9' => '60',
				'8' => '57',
				'7' => '53',
				'6' => '49',
				'5' => '45',
				'4' => '41',
				'3' => '37',
				'2' => '34',
				'1' => '30',
			],
			'10' => [
				'21' => '94',
				'20' => '91',
				'19' => '88',
				'18' => '85',
				'17' => '82',
				'16' => '79',
				'15' => '76',
				'14' => '73',
				'13' => '70',
				'12' => '67',
				'11' => '64',
				'10' => '61',
				'9' => '58',
				'8' => '55',
				'7' => '51',
				'6' => '47',
				'5' => '43',
				'4' => '39',
				'3' => '35',
				'2' => '31',
				'1' => '27',
			],
			'9' => [
				'21' => '91',
				'20' => '88',
				'19' => '85',
				'18' => '82',
				'17' => '79',
				'16' => '76',
				'15' => '73',
				'14' => '70',
				'13' => '67',
				'12' => '64',
				'11' => '61',
				'10' => '58',
				'9' => '55',
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
				'21' => '88',
				'20' => '85',
				'19' => '82',
				'18' => '79',
				'17' => '76',
				'16' => '73',
				'15' => '70',
				'14' => '67',
				'13' => '64',
				'12' => '61',
				'11' => '58',
				'10' => '55',
				'9' => '52',
				'8' => '49',
				'7' => '45',
				'6' => '41',
				'5' => '37',
				'4' => '33',
				'3' => '29',
				'2' => '25',
				'1' => '21',
			],
			'7' => [
				'21' => '85',
				'20' => '82',
				'19' => '79',
				'18' => '76',
				'17' => '73',
				'16' => '70',
				'15' => '67',
				'14' => '64',
				'13' => '61',
				'12' => '58',
				'11' => '55',
				'10' => '52',
				'9' => '49',
				'8' => '46',
				'7' => '42',
				'6' => '38',
				'5' => '34',
				'4' => '30',
				'3' => '26',
				'2' => '22',
				'1' => '18',
			],
			'6' => [
				'21' => '72',
				'20' => '69',
				'19' => '66',
				'18' => '63',
				'17' => '60',
				'16' => '57',
				'15' => '54',
				'14' => '51',
				'13' => '48',
				'12' => '45',
				'11' => '42',
				'10' => '39',
				'9' => '36',
				'8' => '33',
				'7' => '30',
				'6' => '27',
				'5' => '24',
				'4' => '21',
				'3' => '18',
				'2' => '16',
				'1' => '14',
			],
			'5' => [
				'21' => '68',
				'20' => '65',
				'19' => '62',
				'18' => '59',
				'17' => '56',
				'16' => '53',
				'15' => '50',
				'14' => '47',
				'13' => '44',
				'12' => '41',
				'11' => '38',
				'10' => '35',
				'9' => '32',
				'8' => '29',
				'7' => '26',
				'6' => '23',
				'5' => '20',
				'4' => '17',
				'3' => '14',
				'2' => '12',
				'1' => '10',
			],
			'4' => [
				'21' => '49',
				'20' => '47',
				'19' => '45',
				'18' => '43',
				'17' => '41',
				'16' => '39',
				'15' => '37',
				'14' => '35',
				'13' => '33',
				'12' => '31',
				'11' => '29',
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
			'3' => [
				'21' => '46',
				'20' => '44',
				'19' => '42',
				'18' => '40',
				'17' => '38',
				'16' => '36',
				'15' => '34',
				'14' => '32',
				'13' => '30',
				'12' => '28',
				'11' => '26',
				'10' => '24',
				'9' => '22',
				'8' => '20',
				'7' => '18',
				'6' => '16',
				'5' => '14',
				'4' => '12',
				'3' => '10',
				'2' => '8',
				'1' => '6',
			],
		],
		'm' => [
			'20' => [
				'21' => '114',
				'20' => '111',
				'19' => '108',
				'18' => '105',
				'17' => '102',
				'16' => '99',
				'15' => '96',
				'14' => '93',
				'13' => '90',
				'12' => '87',
				'11' => '84',
				'10' => '81',
				'9' => '78',
				'8' => '75',
				'7' => '72',
				'6' => '68',
				'5' => '64',
				'4' => '60',
				'3' => '56',
				'2' => '52',
				'1' => '48',
			],
			'19' => [
				'21' => '112',
				'20' => '109',
				'19' => '106',
				'18' => '103',
				'17' => '100',
				'16' => '97',
				'15' => '94',
				'14' => '91',
				'13' => '88',
				'12' => '85',
				'11' => '82',
				'10' => '79',
				'9' => '76',
				'8' => '73',
				'7' => '70',
				'6' => '66',
				'5' => '62',
				'4' => '58',
				'3' => '54',
				'2' => '50',
				'1' => '46',
			],
			'18' => [
				'21' => '110',
				'20' => '107',
				'19' => '104',
				'18' => '101',
				'17' => '98',
				'16' => '95',
				'15' => '92',
				'14' => '89',
				'13' => '86',
				'12' => '83',
				'11' => '80',
				'10' => '77',
				'9' => '74',
				'8' => '71',
				'7' => '68',
				'6' => '64',
				'5' => '60',
				'4' => '56',
				'3' => '52',
				'2' => '48',
				'1' => '44',
			],
			'17' => [
				'21' => '108',
				'20' => '105',
				'19' => '102',
				'18' => '99',
				'17' => '96',
				'16' => '93',
				'15' => '90',
				'14' => '87',
				'13' => '84',
				'12' => '81',
				'11' => '78',
				'10' => '75',
				'9' => '72',
				'8' => '69',
				'7' => '65',
				'6' => '61',
				'5' => '57',
				'4' => '53',
				'3' => '49',
				'2' => '45',
				'1' => '42',
			],
			'16' => [
				'21' => '106',
				'20' => '103',
				'19' => '100',
				'18' => '97',
				'17' => '94',
				'16' => '91',
				'15' => '88',
				'14' => '85',
				'13' => '82',
				'12' => '79',
				'11' => '76',
				'10' => '73',
				'9' => '70',
				'8' => '67',
				'7' => '64',
				'6' => '60',
				'5' => '56',
				'4' => '52',
				'3' => '48',
				'2' => '44',
				'1' => '40',
			],
			'15' => [
				'21' => '104',
				'20' => '101',
				'19' => '98',
				'18' => '95',
				'17' => '92',
				'16' => '89',
				'15' => '86',
				'14' => '83',
				'13' => '80',
				'12' => '77',
				'11' => '74',
				'10' => '71',
				'9' => '68',
				'8' => '65',
				'7' => '62',
				'6' => '58',
				'5' => '54',
				'4' => '50',
				'3' => '46',
				'2' => '42',
				'1' => '38',
			],
			'14' => [
				'21' => '102',
				'20' => '99',
				'19' => '96',
				'18' => '93',
				'17' => '90',
				'16' => '87',
				'15' => '84',
				'14' => '81',
				'13' => '78',
				'12' => '75',
				'11' => '72',
				'10' => '69',
				'9' => '66',
				'8' => '63',
				'7' => '60',
				'6' => '56',
				'5' => '52',
				'4' => '58',
				'3' => '44',
				'2' => '40',
				'1' => '36',
			],
			'13' => [
				'21' => '100',
				'20' => '97',
				'19' => '94',
				'18' => '91',
				'17' => '88',
				'16' => '85',
				'15' => '82',
				'14' => '79',
				'13' => '76',
				'12' => '73',
				'11' => '70',
				'10' => '67',
				'9' => '64',
				'8' => '61',
				'7' => '58',
				'6' => '54',
				'5' => '50',
				'4' => '46',
				'3' => '42',
				'2' => '38',
				'1' => '34',
			],
			'12' => [
				'21' => '98',
				'20' => '95',
				'19' => '92',
				'18' => '89',
				'17' => '86',
				'16' => '83',
				'15' => '80',
				'14' => '77',
				'13' => '74',
				'12' => '71',
				'11' => '68',
				'10' => '65',
				'9' => '62',
				'8' => '59',
				'7' => '56',
				'6' => '52',
				'5' => '48',
				'4' => '44',
				'3' => '40',
				'2' => '36',
				'1' => '32',
			],
			'11' => [
				'21' => '96',
				'20' => '93',
				'19' => '90',
				'18' => '87',
				'17' => '84',
				'16' => '81',
				'15' => '78',
				'14' => '75',
				'13' => '72',
				'12' => '69',
				'11' => '66',
				'10' => '63',
				'9' => '60',
				'8' => '57',
				'7' => '53',
				'6' => '49',
				'5' => '45',
				'4' => '41',
				'3' => '37',
				'2' => '34',
				'1' => '30',
			],
			'10' => [
				'21' => '94',
				'20' => '91',
				'19' => '88',
				'18' => '85',
				'17' => '82',
				'16' => '79',
				'15' => '76',
				'14' => '73',
				'13' => '70',
				'12' => '67',
				'11' => '64',
				'10' => '61',
				'9' => '58',
				'8' => '55',
				'7' => '51',
				'6' => '47',
				'5' => '43',
				'4' => '39',
				'3' => '35',
				'2' => '31',
				'1' => '27',
			],
			'9' => [
				'21' => '91',
				'20' => '88',
				'19' => '85',
				'18' => '82',
				'17' => '79',
				'16' => '76',
				'15' => '73',
				'14' => '70',
				'13' => '67',
				'12' => '64',
				'11' => '61',
				'10' => '58',
				'9' => '55',
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
				'21' => '88',
				'20' => '85',
				'19' => '82',
				'18' => '79',
				'17' => '76',
				'16' => '73',
				'15' => '70',
				'14' => '67',
				'13' => '64',
				'12' => '61',
				'11' => '58',
				'10' => '55',
				'9' => '52',
				'8' => '49',
				'7' => '45',
				'6' => '41',
				'5' => '37',
				'4' => '33',
				'3' => '29',
				'2' => '25',
				'1' => '21',
			],
			'7' => [
				'21' => '85',
				'20' => '82',
				'19' => '79',
				'18' => '76',
				'17' => '73',
				'16' => '70',
				'15' => '67',
				'14' => '64',
				'13' => '61',
				'12' => '58',
				'11' => '55',
				'10' => '52',
				'9' => '49',
				'8' => '46',
				'7' => '42',
				'6' => '38',
				'5' => '34',
				'4' => '30',
				'3' => '26',
				'2' => '22',
				'1' => '18',
			],
			'6' => [
				'21' => '75',
				'20' => '72',
				'19' => '69',
				'18' => '66',
				'17' => '63',
				'16' => '60',
				'15' => '57',
				'14' => '54',
				'13' => '51',
				'12' => '48',
				'11' => '45',
				'10' => '42',
				'9' => '39',
				'8' => '36',
				'7' => '33',
				'6' => '30',
				'5' => '27',
				'4' => '24',
				'3' => '21',
				'2' => '18',
				'1' => '15',
			],
			'5' => [
				'21' => '73',
				'20' => '70',
				'19' => '67',
				'18' => '64',
				'17' => '61',
				'16' => '58',
				'15' => '55',
				'14' => '52',
				'13' => '49',
				'12' => '46',
				'11' => '43',
				'10' => '40',
				'9' => '37',
				'8' => '34',
				'7' => '31',
				'6' => '28',
				'5' => '25',
				'4' => '22',
				'3' => '19',
				'2' => '16',
				'1' => '13',
			],
			'4' => [
				'21' => '53',
				'20' => '51',
				'19' => '49',
				'18' => '47',
				'17' => '45',
				'16' => '43',
				'15' => '41',
				'14' => '39',
				'13' => '37',
				'12' => '35',
				'11' => '33',
				'10' => '31',
				'9' => '29',
				'8' => '27',
				'7' => '25',
				'6' => '23',
				'5' => '21',
				'4' => '19',
				'3' => '17',
				'2' => '15',
				'1' => '12',
			],
			'3' => [
				'21' => '50',
				'20' => '48',
				'19' => '46',
				'18' => '44',
				'17' => '42',
				'16' => '40',
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
		],
	];
}
