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
use Hungarofit\Evaluator\Exercise\Motor6Jump;
use Hungarofit\Evaluator\Exercise\Motor6Pushup;
use Hungarofit\Evaluator\Exercise\Motor6Situp;
use Hungarofit\Evaluator\Exercise\Motor6Throwdouble;
use Hungarofit\Evaluator\Exercise\Motor6Throwsingle;
use Hungarofit\Evaluator\Exercise\Motor6Torso;
use Hungarofit\Evaluator\ExerciseInterface;

/**
 * Class Hungarofit6
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
class Hungarofit6 extends Challenge
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
            Motor6Situp::get(),
            Motor6Jump::get(),
            Motor6Pushup::get(),
            Motor6Torso::get(),
            Motor6Throwdouble::get(),
            Motor6Throwsingle::get(),
        ];
    }
}