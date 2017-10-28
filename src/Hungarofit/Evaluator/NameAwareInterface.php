<?php

namespace Hungarofit\Evaluator;


interface NameAwareInterface
{
    /**
     * @return string
     */
    function getName();

    /**
     * @return string
     */
    function getKey();
}