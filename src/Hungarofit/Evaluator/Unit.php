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

    public static function isAscendingValue($value)
    {
        switch ($value) {
            case self::MILE:
            case self::KILOMETER:
            case self::METER:
            case self::COUNT:
                return true;
                break;
            case self::MINUTE:
                return false;
                break;
            default:
                throw new \InvalidArgumentException('Invalid Unit value: '.$value);
        }
    }

    public static function isDescendingValue($value)
    {
        return !self::isAscendingValue($value);
    }
}