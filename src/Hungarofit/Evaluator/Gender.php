<?php

namespace Hungarofit\Evaluator;


class Gender
{
    const FEMALE = 'f';
    const MALE = 'm';

    public static function isValid($gender)
    {
        return $gender === self::FEMALE || $gender === self::MALE;
    }
}