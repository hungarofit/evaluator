<?php

namespace Hungarofit\Evaluator;


interface NameAwareInterface
{
    /**
     * Returns name (jump)
     * @return string
     */
    function getName();

    /**
     * Returns key (motor4-jump)
     * @return string
     */
    function getKey();
}