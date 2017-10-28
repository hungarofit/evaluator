<?php

namespace Hungarofit\Evaluator;


interface LimitAwareInterface
{
    /**
     * @param Gender $gender
     * @return int
     */
    function getMinAge(Gender $gender=null);

    /**
     * @param Gender $gender
     * @param int $age
     * @return float
     */
    function getMinResult(Gender $gender, $age);

    /**
     * @param Gender $gender
     * @param int $age
     * @return float
     */
    function getMinPoints(Gender $gender, $age);
}