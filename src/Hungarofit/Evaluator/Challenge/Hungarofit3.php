<?php

namespace Hungarofit\Evaluator\Challenge;


use Hungarofit\Evaluator\Challenge;
use Hungarofit\Evaluator\Exercise\AerobRun12min;
use Hungarofit\Evaluator\Exercise\AerobRun6min;
use Hungarofit\Evaluator\Exercise\Motor3Jump;
use Hungarofit\Evaluator\Exercise\Motor3Situp;
use Hungarofit\Evaluator\Exercise\Motor3Torso;

/**
 * Class Hungarofit3
 * @package Hungarofit\Evaluator\ChallengeTorso
 * @method $this setAerobRun6min($result)
 * @method $this setAerobRun12min($result)
 * @method $this setSitup($result)
 * @method $this setTorso($result)
 * @method $this setJump($result)
 */
class Hungarofit3 extends Challenge
{
    function getRequiredExercises()
    {
        return [
            [
                AerobRun6min::get(),
                AerobRun12min::get(),
            ],
            Motor3Situp::get(),
            Motor3Torso::get(),
            Motor3Jump::get(),
        ];
    }
}