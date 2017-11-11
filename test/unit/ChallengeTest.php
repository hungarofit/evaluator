<?php

use PHPUnit\Framework\TestCase;
use Hungarofit\Evaluator\Gender;
use Hungarofit\Evaluator\Rating;
use Hungarofit\Evaluator\Challenge;
use Hungarofit\Evaluator\Exercise\Motor4Jump;
use Hungarofit\Evaluator\Exercise\Motor4Pushup;
use Hungarofit\Evaluator\Exercise\Motor4Situp;
use Hungarofit\Evaluator\Exercise\Motor4Torso;
use Hungarofit\Evaluator\Exercise\AerobRun6min;
use Hungarofit\Evaluator\ChallengeInterface;

class ChallengeTest extends TestCase
{
    public function testMagic()
    {
        $x = new Challenge\Hungarofit4(Gender::MALE(), 12);
        $x->setJump(1.56);
        $x->setSitup(34);
        $x->setTorso(34);
        $x->setPushup(34);
        $x->setAerobRun6min(660);
        $this->assertEquals([
            'motor4-jump' => 1,
            'motor4-situp' => 0,
            'motor4-torso' => 2,
            'motor4-pushup' => 10,
            'aerob-run-6min' => 1,
        ], $x->evaluate());
    }

    public function provideEvaluate()
    {
        return [
            [new Challenge\Hungarofit4(Gender::MALE(), 12), Rating::TERRIBLE(), [
                [Motor4Jump::get(), 1.56, 1],
                [Motor4Torso::get(), 34, 2],
                [Motor4Situp::get(), 34, 0],
                [Motor4Pushup::get(), 34, 10],
                [AerobRun6min::get(), 660, 1],
            ]],
            [new Challenge\Hungarofit4(Gender::MALE(), 12), Rating::EXCELLENT(), [
                [Motor4Jump::get(), 2.2, 21],
                [Motor4Torso::get(), 98, 14],
                [Motor4Situp::get(), 98, 13],
                [Motor4Pushup::get(), 34, 10],
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

    public function testOptionalAerob()
    {
        $x = new Challenge\Hungarofit4(Gender::MALE(), 12);
        $x->setPushup(40);
        $x->setTorso(60);
        $x->setSitup(50);
        $x->setJump(1.92);
        $this->assertEquals('GOOD', $x->rate()->getName());
        $x->setAerobRun6min(1200);
        $this->assertEquals('GOOD', $x->rate()->getName());
    }
}
