<?php

namespace Hungarofit\Evaluator\Exercise;


use Hungarofit\Evaluator\Unit;
use Hungarofit\Evaluator\Exercise;

final class Motor6Throwsingle extends Exercise
{
    /** Unit of exercise */
    const UNIT_EXERCISE = Unit::COUNT;
    
    /** Unit of result */
    const UNIT_RESULT = Unit::METER;
    
    /** Lookup table */
    const TABLE = [
		'f' => [
			'20' => [
				'10.5' => '8.8',
				'10' => '8.65',
				'9.5' => '8.5',
				'9' => '8.35',
				'8.5' => '8.2',
				'8' => '8.05',
				'7.5' => '7.9',
				'7' => '7.75',
				'6.5' => '7.6',
				'6' => '7.45',
				'5.5' => '7.3',
				'5' => '7.15',
				'4.5' => '7',
				'4' => '6.85',
				'3.5' => '6.7',
				'3' => '6.55',
				'2.5' => '6.4',
				'2' => '6.2',
				'1.5' => '6',
				'1' => '5.8',
				'0.5' => '5.6',
			],
			'19' => [
				'10.5' => '8.7',
				'10' => '8.55',
				'9.5' => '8.4',
				'9' => '8.25',
				'8.5' => '8.1',
				'8' => '7.95',
				'7.5' => '7.8',
				'7' => '7.65',
				'6.5' => '7.5',
				'6' => '7.35',
				'5.5' => '7.2',
				'5' => '7.05',
				'4.5' => '6.9',
				'4' => '6.75',
				'3.5' => '6.6',
				'3' => '6.45',
				'2.5' => '6.3',
				'2' => '6.1',
				'1.5' => '5.9',
				'1' => '5.7',
				'0.5' => '5.5',
			],
			'18' => [
				'10.5' => '8.6',
				'10' => '8.45',
				'9.5' => '8.3',
				'9' => '8.15',
				'8.5' => '8',
				'8' => '7.85',
				'7.5' => '7.7',
				'7' => '7.55',
				'6.5' => '7.4',
				'6' => '7.25',
				'5.5' => '7.1',
				'5' => '6.95',
				'4.5' => '6.8',
				'4' => '6.65',
				'3.5' => '6.5',
				'3' => '6.35',
				'2.5' => '6.2',
				'2' => '6',
				'1.5' => '5.8',
				'1' => '5.6',
				'0.5' => '5.4',
			],
			'17' => [
				'10.5' => '8.5',
				'10' => '8.35',
				'9.5' => '8.2',
				'9' => '8.05',
				'8.5' => '7.9',
				'8' => '7.75',
				'7.5' => '7.6',
				'7' => '7.45',
				'6.5' => '7.3',
				'6' => '7.15',
				'5.5' => '7',
				'5' => '6.85',
				'4.5' => '6.7',
				'4' => '6.55',
				'3.5' => '6.4',
				'3' => '6.25',
				'2.5' => '6.1',
				'2' => '5.9',
				'1.5' => '5.7',
				'1' => '5.5',
				'0.5' => '5.3',
			],
			'16' => [
				'10.5' => '8.3',
				'10' => '8.15',
				'9.5' => '8',
				'9' => '7.85',
				'8.5' => '7.7',
				'8' => '7.55',
				'7.5' => '7.4',
				'7' => '7.25',
				'6.5' => '7.1',
				'6' => '6.95',
				'5.5' => '6.8',
				'5' => '6.65',
				'4.5' => '6.5',
				'4' => '6.35',
				'3.5' => '6.2',
				'3' => '6.05',
				'2.5' => '5.9',
				'2' => '5.7',
				'1.5' => '5.5',
				'1' => '5.3',
				'0.5' => '5.1',
			],
			'15' => [
				'10.5' => '8.1',
				'10' => '7.95',
				'9.5' => '7.8',
				'9' => '7.65',
				'8.5' => '7.5',
				'8' => '7.35',
				'7.5' => '7.2',
				'7' => '7.05',
				'6.5' => '6.9',
				'6' => '6.75',
				'5.5' => '6.6',
				'5' => '6.45',
				'4.5' => '6.3',
				'4' => '6.15',
				'3.5' => '6',
				'3' => '5.85',
				'2.5' => '5.7',
				'2' => '5.5',
				'1.5' => '5.3',
				'1' => '5.1',
				'0.5' => '4.9',
			],
			'14' => [
				'10.5' => '7.9',
				'10' => '7.75',
				'9.5' => '7.6',
				'9' => '7.45',
				'8.5' => '7.3',
				'8' => '7.15',
				'7.5' => '7',
				'7' => '6.85',
				'6.5' => '6.7',
				'6' => '6.55',
				'5.5' => '6.4',
				'5' => '6.25',
				'4.5' => '6.1',
				'4' => '5.95',
				'3.5' => '5.8',
				'3' => '5.65',
				'2.5' => '5.5',
				'2' => '5.3',
				'1.5' => '5.1',
				'1' => '4.9',
				'0.5' => '4.7',
			],
			'13' => [
				'10.5' => '7.5',
				'10' => '7.35',
				'9.5' => '7.2',
				'9' => '7.05',
				'8.5' => '6.9',
				'8' => '6.75',
				'7.5' => '6.6',
				'7' => '6.45',
				'6.5' => '6.3',
				'6' => '6.15',
				'5.5' => '6',
				'5' => '5.85',
				'4.5' => '5.7',
				'4' => '5.55',
				'3.5' => '5.4',
				'3' => '5.25',
				'2.5' => '5.1',
				'2' => '4.9',
				'1.5' => '4.7',
				'1' => '4.5',
				'0.5' => '4.3',
			],
			'12' => [
				'10.5' => '7.1',
				'10' => '6.95',
				'9.5' => '6.8',
				'9' => '6.65',
				'8.5' => '6.5',
				'8' => '6.35',
				'7.5' => '6.2',
				'7' => '6.05',
				'6.5' => '5.9',
				'6' => '5.75',
				'5.5' => '5.6',
				'5' => '5.45',
				'4.5' => '5.3',
				'4' => '5.15',
				'3.5' => '5',
				'3' => '4.85',
				'2.5' => '4.7',
				'2' => '4.5',
				'1.5' => '4.3',
				'1' => '4.1',
				'0.5' => '3.9',
			],
			'11' => [
				'10.5' => '6.6',
				'10' => '6.45',
				'9.5' => '6.3',
				'9' => '6.15',
				'8.5' => '6',
				'8' => '5.85',
				'7.5' => '5.7',
				'7' => '5.55',
				'6.5' => '5.4',
				'6' => '5.25',
				'5.5' => '5.1',
				'5' => '4.95',
				'4.5' => '4.8',
				'4' => '4.65',
				'3.5' => '4.5',
				'3' => '4.35',
				'2.5' => '4.2',
				'2' => '4',
				'1.5' => '3.8',
				'1' => '3.6',
				'0.5' => '3.4',
			],
			'10' => [
				'10.5' => '6.1',
				'10' => '5.95',
				'9.5' => '5.8',
				'9' => '5.65',
				'8.5' => '5.5',
				'8' => '5.35',
				'7.5' => '5.2',
				'7' => '5.05',
				'6.5' => '4.9',
				'6' => '4.75',
				'5.5' => '4.6',
				'5' => '4.45',
				'4.5' => '4.3',
				'4' => '4.15',
				'3.5' => '4',
				'3' => '3.85',
				'2.5' => '3.7',
				'2' => '3.5',
				'1.5' => '3.3',
				'1' => '3.1',
				'0.5' => '2.9',
			],
			'9' => [
				'10.5' => '5.5',
				'10' => '5.35',
				'9.5' => '5.2',
				'9' => '5.05',
				'8.5' => '4.9',
				'8' => '4.75',
				'7.5' => '4.6',
				'7' => '4.45',
				'6.5' => '4.3',
				'6' => '4.15',
				'5.5' => '4',
				'5' => '3.85',
				'4.5' => '3.7',
				'4' => '3.55',
				'3.5' => '3.4',
				'3' => '3.25',
				'2.5' => '3.1',
				'2' => '2.9',
				'1.5' => '2.7',
				'1' => '2.5',
				'0.5' => '2.3',
			],
			'8' => [
				'10.5' => '5.1',
				'10' => '4.95',
				'9.5' => '4.8',
				'9' => '4.65',
				'8.5' => '4.5',
				'8' => '4.35',
				'7.5' => '4.2',
				'7' => '4.05',
				'6.5' => '3.9',
				'6' => '3.75',
				'5.5' => '3.6',
				'5' => '3.45',
				'4.5' => '3.3',
				'4' => '3.15',
				'3.5' => '3',
				'3' => '2.85',
				'2.5' => '2.7',
				'2' => '2.5',
				'1.5' => '2.3',
				'1' => '2.1',
				'0.5' => '1.9',
			],
			'7' => [
				'10.5' => '4.4',
				'10' => '4.25',
				'9.5' => '4.1',
				'9' => '3.95',
				'8.5' => '3.8',
				'8' => '3.65',
				'7.5' => '3.5',
				'7' => '3.35',
				'6.5' => '3.2',
				'6' => '3.05',
				'5.5' => '2.9',
				'5' => '2.75',
				'4.5' => '2.6',
				'4' => '2.45',
				'3.5' => '2.3',
				'3' => '2.1',
				'2.5' => '2',
				'2' => '1.8',
				'1.5' => '1.6',
				'1' => '1.4',
				'0.5' => '1.2',
			],
		],
		'm' => [
			'20' => [
				'10.5' => '13.6',
				'10' => '13.35',
				'9.5' => '13.1',
				'9' => '12.85',
				'8.5' => '12.6',
				'8' => '12.35',
				'7.5' => '12.1',
				'7' => '11.85',
				'6.5' => '11.6',
				'6' => '11.35',
				'5.5' => '11.1',
				'5' => '10.8',
				'4.5' => '10.5',
				'4' => '10.2',
				'3.5' => '9.9',
				'3' => '9.6',
				'2.5' => '9.3',
				'2' => '9.05',
				'1.5' => '8.8',
				'1' => '8.55',
				'0.5' => '8.3',
			],
			'19' => [
				'10.5' => '13.3',
				'10' => '13.05',
				'9.5' => '12.8',
				'9' => '12.55',
				'8.5' => '12.3',
				'8' => '12.05',
				'7.5' => '11.8',
				'7' => '11.55',
				'6.5' => '11.3',
				'6' => '11.05',
				'5.5' => '10.8',
				'5' => '10.55',
				'4.5' => '10.3',
				'4' => '10.05',
				'3.5' => '9.8',
				'3' => '9.5',
				'2.5' => '9.2',
				'2' => '8.9',
				'1.5' => '8.6',
				'1' => '8.3',
				'0.5' => '8',
			],
			'18' => [
				'10.5' => '13.1',
				'10' => '12.85',
				'9.5' => '12.6',
				'9' => '12.35',
				'8.5' => '12.1',
				'8' => '11.85',
				'7.5' => '11.6',
				'7' => '11.35',
				'6.5' => '11.1',
				'6' => '10.85',
				'5.5' => '10.6',
				'5' => '10.35',
				'4.5' => '10.1',
				'4' => '9.8',
				'3.5' => '9.5',
				'3' => '9.2',
				'2.5' => '8.9',
				'2' => '8.6',
				'1.5' => '8.3',
				'1' => '8',
				'0.5' => '7.7',
			],
			'17' => [
				'10.5' => '12.7',
				'10' => '12.45',
				'9.5' => '12.2',
				'9' => '11.95',
				'8.5' => '11.7',
				'8' => '11.45',
				'7.5' => '11.2',
				'7' => '10.95',
				'6.5' => '10.7',
				'6' => '10.45',
				'5.5' => '10.2',
				'5' => '9.95',
				'4.5' => '9.7',
				'4' => '9.45',
				'3.5' => '9.2',
				'3' => '8.9',
				'2.5' => '8.6',
				'2' => '8.3',
				'1.5' => '8',
				'1' => '7.7',
				'0.5' => '7.4',
			],
			'16' => [
				'10.5' => '12.3',
				'10' => '12.05',
				'9.5' => '11.8',
				'9' => '11.55',
				'8.5' => '11.3',
				'8' => '11.05',
				'7.5' => '10.8',
				'7' => '10.55',
				'6.5' => '10.3',
				'6' => '10.05',
				'5.5' => '9.8',
				'5' => '9.55',
				'4.5' => '9.3',
				'4' => '9.05',
				'3.5' => '8.8',
				'3' => '8.5',
				'2.5' => '8.2',
				'2' => '7.9',
				'1.5' => '7.6',
				'1' => '7.3',
				'0.5' => '7',
			],
			'15' => [
				'10.5' => '11.8',
				'10' => '11.55',
				'9.5' => '11.3',
				'9' => '11.05',
				'8.5' => '10.8',
				'8' => '10.55',
				'7.5' => '10.3',
				'7' => '10.05',
				'6.5' => '9.8',
				'6' => '9.55',
				'5.5' => '9.3',
				'5' => '9.05',
				'4.5' => '8.8',
				'4' => '8.55',
				'3.5' => '8.3',
				'3' => '8',
				'2.5' => '7.7',
				'2' => '7.4',
				'1.5' => '7.1',
				'1' => '6.8',
				'0.5' => '6.5',
			],
			'14' => [
				'10.5' => '11.2',
				'10' => '10.95',
				'9.5' => '10.7',
				'9' => '10.45',
				'8.5' => '10.2',
				'8' => '9.95',
				'7.5' => '9.7',
				'7' => '9.45',
				'6.5' => '9.2',
				'6' => '8.95',
				'5.5' => '8.7',
				'5' => '8.45',
				'4.5' => '8.2',
				'4' => '7.95',
				'3.5' => '7.7',
				'3' => '7.4',
				'2.5' => '7.1',
				'2' => '6.8',
				'1.5' => '6.5',
				'1' => '6.2',
				'0.5' => '5.9',
			],
			'13' => [
				'10.5' => '10.7',
				'10' => '10.45',
				'9.5' => '10.2',
				'9' => '9.95',
				'8.5' => '9.7',
				'8' => '9.45',
				'7.5' => '9.2',
				'7' => '8.95',
				'6.5' => '8.7',
				'6' => '8.45',
				'5.5' => '8.2',
				'5' => '7.95',
				'4.5' => '7.7',
				'4' => '7.45',
				'3.5' => '7.2',
				'3' => '6.9',
				'2.5' => '6.6',
				'2' => '6.3',
				'1.5' => '6',
				'1' => '5.7',
				'0.5' => '5.4',
			],
			'12' => [
				'10.5' => '10',
				'10' => '9.75',
				'9.5' => '9.5',
				'9' => '9.25',
				'8.5' => '9',
				'8' => '8.75',
				'7.5' => '8.5',
				'7' => '8.25',
				'6.5' => '8',
				'6' => '7.75',
				'5.5' => '7.5',
				'5' => '7.25',
				'4.5' => '7',
				'4' => '6.75',
				'3.5' => '6.5',
				'3' => '6.2',
				'2.5' => '5.9',
				'2' => '5.6',
				'1.5' => '5.3',
				'1' => '5',
				'0.5' => '4.7',
			],
			'11' => [
				'10.5' => '9.2',
				'10' => '8.95',
				'9.5' => '8.7',
				'9' => '8.45',
				'8.5' => '8.2',
				'8' => '7.95',
				'7.5' => '7.7',
				'7' => '7.45',
				'6.5' => '7.2',
				'6' => '6.95',
				'5.5' => '6.7',
				'5' => '6.45',
				'4.5' => '6.2',
				'4' => '5.95',
				'3.5' => '5.7',
				'3' => '5.4',
				'2.5' => '5.1',
				'2' => '4.8',
				'1.5' => '4.5',
				'1' => '4.2',
				'0.5' => '3.9',
			],
			'10' => [
				'10.5' => '8.5',
				'10' => '8.25',
				'9.5' => '8',
				'9' => '7.75',
				'8.5' => '7.5',
				'8' => '7.25',
				'7.5' => '7',
				'7' => '6.75',
				'6.5' => '6.5',
				'6' => '6.25',
				'5.5' => '6',
				'5' => '5.75',
				'4.5' => '5.5',
				'4' => '5.2',
				'3.5' => '4.9',
				'3' => '4.6',
				'2.5' => '4.3',
				'2' => '4',
				'1.5' => '3.7',
				'1' => '3.4',
				'0.5' => '3.1',
			],
			'9' => [
				'10.5' => '7.7',
				'10' => '7.45',
				'9.5' => '7.2',
				'9' => '6.95',
				'8.5' => '6.7',
				'8' => '6.45',
				'7.5' => '6.2',
				'7' => '5.95',
				'6.5' => '5.7',
				'6' => '5.45',
				'5.5' => '5.2',
				'5' => '4.95',
				'4.5' => '4.7',
				'4' => '4.45',
				'3.5' => '4.2',
				'3' => '3.9',
				'2.5' => '3.6',
				'2' => '3.3',
				'1.5' => '3',
				'1' => '2.7',
				'0.5' => '2.4',
			],
			'8' => [
				'10.5' => '6.8',
				'10' => '6.55',
				'9.5' => '6.3',
				'9' => '6.05',
				'8.5' => '5.8',
				'8' => '5.55',
				'7.5' => '5.3',
				'7' => '5.05',
				'6.5' => '4.8',
				'6' => '4.55',
				'5.5' => '4.3',
				'5' => '4.05',
				'4.5' => '3.8',
				'4' => '3.55',
				'3.5' => '3.3',
				'3' => '3',
				'2.5' => '2.7',
				'2' => '2.4',
				'1.5' => '2.1',
				'1' => '1.8',
				'0.5' => '1.5',
			],
			'7' => [
				'10.5' => '5.95',
				'10' => '5.7',
				'9.5' => '5.45',
				'9' => '5.2',
				'8.5' => '4.95',
				'8' => '4.7',
				'7.5' => '4.45',
				'7' => '4.2',
				'6.5' => '3.95',
				'6' => '3.7',
				'5.5' => '3.45',
				'5' => '3.2',
				'4.5' => '2.95',
				'4' => '2.7',
				'3.5' => '2.45',
				'3' => '2.2',
				'2.5' => '1.9',
				'2' => '1.6',
				'1.5' => '1.3',
				'1' => '1',
				'0.5' => '0.7',
			],
		],
	];
}
