<?php

namespace Hungarofit\Evaluator;

/**
 * @method static MILE
 * @method static MINUTE
 * @method static KILOMETER
 * @method static METER
 * @method static COUNT
 */
class Unit extends Enum
{
    const MILE = 'mile';
    const MINUTE = 'min';
    const KILOMETER = 'km';
    const METER = 'm';
    const COUNT = 'n';

    public function isAscending()
    {
        return self::isAscendingValue($this->getValue());
    }

    public function isDescending()
    {
        return !$this->isAscending();
    }

    public function isAscendingValue($value)
    {
        switch ($value) {
            case self::MILE:
            case self::KILOMETER:
            case self::METER:
                return false;
                break;
            default:
            case self::MINUTE:
            case self::COUNT:
                return true;
                break;
        }
    }

    public function isDescendingValue($value)
    {
        return !self::isAscendingValue($value);
    }
}