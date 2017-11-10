<?php

namespace Hungarofit\Evaluator;


/**
 * Class Rating
 * @package Hungarofit\Evaluator
 * @method static TERRIBLE()
 * @method static VERY_POOR =()
 * @method static POOR()
 * @method static MEDIOCRE()
 * @method static GOOD()
 * @method static VERY_GOOD()
 * @method static EXCELLENT()
 */
class Rating extends Enum
{
    const TERRIBLE = 1;
    const VERY_POOR = 2;
    const POOR = 3;
    const MEDIOCRE = 4;
    const GOOD = 5;
    const VERY_GOOD = 6;
    const EXCELLENT = 7;
}
