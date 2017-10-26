<?php

namespace Hungarofit\Evaluator;


interface UnitAwareInterface
{
    /**
     * @return Unit
     */
    function getExerciseUnit();
    /**
     * @return Unit
     */
    function getResultUnit();
}