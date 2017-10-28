<?php

namespace Hungarofit\Evaluator;


/**
 * Class Rating
 * @package Hungarofit\Evaluator
 * @method static VERY_POOR()
 * @method static POOR =()
 * @method static MEDIOCRE()
 * @method static AVERAGE()
 * @method static GOOD()
 * @method static VERY_GOOD()
 * @method static EXCELLENT()
 */
class Rating extends Enum
{
    const VERY_POOR = 1;
    const POOR = 2;
    const MEDIOCRE = 3;
    const AVERAGE = 4;
    const GOOD = 5;
    const VERY_GOOD = 6;
    const EXCELLENT = 7;
}
