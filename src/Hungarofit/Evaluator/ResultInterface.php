<?php

namespace Hungarofit\Evaluator;


interface ResultInterface
{
    function getExercise();
    function setResult($value);
    function validate();
    function evaluate();
}