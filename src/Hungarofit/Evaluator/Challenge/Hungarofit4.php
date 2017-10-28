<?php

namespace Hungarofit\Evaluator\Challenge;


use Hungarofit\Evaluator\Challenge;
use Hungarofit\Evaluator\Exercise\AerobBike12min;
use Hungarofit\Evaluator\Exercise\AerobRun12min;
use Hungarofit\Evaluator\Exercise\AerobRun1mile;
use Hungarofit\Evaluator\Exercise\AerobRun1mile5;
use Hungarofit\Evaluator\Exercise\AerobRun2km;
use Hungarofit\Evaluator\Exercise\AerobRun2mile;
use Hungarofit\Evaluator\Exercise\AerobRun6min;
use Hungarofit\Evaluator\Exercise\AerobSwim12min;
use Hungarofit\Evaluator\Exercise\AerobSwim500m;
use Hungarofit\Evaluator\Exercise\Motor4Jump;
use Hungarofit\Evaluator\Exercise\Motor4Pushup;
use Hungarofit\Evaluator\Exercise\Motor4Situp;
use Hungarofit\Evaluator\Exercise\Motor4Torso;


/**
 * Class Hungarofit4
 * @package Hungarofit\Evaluator\Challenge
 * @method $this setAerobBike12min($result)
 * @method $this setAerobRun1mile($result)
 * @method $this setAerobRun1mile5($result)
 * @method $this setAerobRun2km($result)
 * @method $this setAerobRun2mile($result)
 * @method $this setAerobRun6min($result)
 * @method $this setAerobRun12min($result)
 * @method $this setAerobSwim12min($result)
 * @method $this setAerobSwim500m($result)
 * @method $this setSitup($result)
 * @method $this setJump($result)
 * @method $this setPushup($result)
 * @method $this setTorso($result)
 */
class Hungarofit4 extends Challenge
{
    function getRequiredExercises()
    {
        return [
            [
                AerobBike12min::get(),
                AerobRun1mile::get(),
                AerobRun1mile5::get(),
                AerobRun2km::get(),
                AerobRun2mile::get(),
                AerobRun6min::get(),
                AerobRun12min::get(),
                AerobSwim12min::get(),
                AerobSwim500m::get(),
            ],
            Motor4Situp::get(),
            Motor4Jump::get(),
            Motor4Pushup::get(),
            Motor4Torso::get(),
        ];
    }
}