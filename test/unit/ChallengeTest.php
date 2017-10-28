<?php

use PHPUnit\Framework\TestCase;
use Hungarofit\Evaluator\Gender;
use Hungarofit\Evaluator\Rating;
use Hungarofit\Evaluator\Challenge;
use Hungarofit\Evaluator\Exercise\Motor3Jump;
use Hungarofit\Evaluator\Exercise\Motor3Torso;
use Hungarofit\Evaluator\Exercise\Motor3Situp;
use Hungarofit\Evaluator\Exercise\AerobRun6min;
use Hungarofit\Evaluator\ChallengeInterface;

class ChallengeTest extends TestCase
{
    public function testMagic()
    {
        $x = new Challenge\Hungarofit3(Gender::MALE(), 12);
        $x->setJump(1.56);
        $x->setSitup(34);
        $x->setTorso(34);
        $x->setAerobRun6min(660);
        $this->assertEquals([
            'jump' => 1,
            'situp' => 1,
            'torso' => 1,
            'aerob-run-6min' => 1,
        ], $x->evaluate());
    }

    public function provideEvaluate()
    {
        return [
            [new Challenge\Hungarofit3(Gender::MALE(), 12), Rating::VERY_POOR(), [
                [Motor3Jump::get(), 1.56, 1],
                [Motor3Torso::get(), 34, 1],
                [Motor3Situp::get(), 34, 1],
                [AerobRun6min::get(), 660, 1],
            ]],
            [new Challenge\Hungarofit3(Gender::MALE(), 12), Rating::EXCELLENT(), [
                [Motor3Jump::get(), 2.2, 21],
                [Motor3Torso::get(), 98, 21],
                [Motor3Situp::get(), 98, 21],
                [AerobRun6min::get(), 1475, 77],
            ]],
        ];
    }

    /**
     * @dataProvider provideEvaluate
     * @param ChallengeInterface $c
     * @param int $rating
     * @param $results
     */
    public function testEvaluate(ChallengeInterface $c, $rating, $results)
    {
        $c->clear();
        $points = [];
        /**
         * @var \Hungarofit\Evaluator\ExerciseInterface $x
         * @var float $r
         * @var float $p
         */
        foreach($results as list($x, $r, $p)) {
            $c->setResult($x, $r);
            $points[$x->getKey()] = $p;
        }
        $this->assertEquals($points, $c->evaluate());
        $this->assertEquals($rating, $c->rate());
    }
}