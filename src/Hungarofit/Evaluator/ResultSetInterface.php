<?php

namespace Hungarofit\Evaluator;


use Traversable;

interface ResultSetInterface extends Traversable
{
    function getChallenge();
    function validate();
    function evaluate();
}