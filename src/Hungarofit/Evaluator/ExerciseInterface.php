<?php

namespace Hungarofit\Evaluator;


interface ExerciseInterface extends NameAwareInterface, UnitAwareInterface, LimitAwareInterface
{
    /**
     * @param Gender $gender
     * @param int $age
     * @param float $result
     * @return float|null
     */
    function evaluate(Gender $gender, $age, $result);
}