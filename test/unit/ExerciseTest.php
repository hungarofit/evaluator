<?php

namespace Hungarofit\Evaluator\Lookup {
    use Hungarofit\Evaluator\Lookup;
    use Hungarofit\Evaluator\Unit;

    final class FirstExercise12min extends Lookup
    {
        const UNIT_EXERCISE = Unit::MINUTE;
        const UNIT_RESULT = Unit::METER;
        const TABLE = [
            'f' => [
                '20' => [
                    '1' => 25.1,
                    '2' => 35.2,
                    '3' => 45.3,
                    '4' => 55.4,
                    '5' => 65.5,
                ],
                '15' => [
                    '1' => 15.1,
                    '2' => 25.2,
                    '3' => 35.3,
                    '4' => 45.4,
                    '5' => 55.5,
                ],
                '10' => [
                    '1' => 10.1,
                    '2' => 20.2,
                    '3' => 30.3,
                    '4' => 40.4,
                    '5' => 50.5,
                ],
            ],
            'm' => [
                '10' => [
                    '1' => 10.1,
                    '2' => 20.2,
                    '3' => 30.3,
                    '4' => 40.4,
                    '5' => 50.5,
                ],
                '15' => [
                    '1' => 15.1,
                    '2' => 25.2,
                    '3' => 35.3,
                    '4' => 45.4,
                    '5' => 55.5,
                ],
                '20' => [
                    '1' => 25.1,
                    '2' => 35.2,
                    '3' => 45.3,
                    '4' => 55.4,
                    '5' => 65.5,
                ],
            ],
        ];
    }

    final class SecondExercise1km extends Lookup
    {
        const UNIT_EXERCISE = Unit::KILOMETER;
        const UNIT_RESULT = Unit::MINUTE;
        const TABLE = [
            'f' => [
                '10' => [
                    '1' => 50,
                    '1.5' => 45,
                    '2' => 40,
                    '2.5' => 35,
                    '3' => 30,
                    '3.5' => 25,
                    '4' => 20,
                    '4.5' => 15,
                    '5' => 10,
                ],
                '20' => [
                    '1' => 65,
                    '1.5' => 60,
                    '2' => 55,
                    '2.5' => 50,
                    '3' => 45,
                    '3.5' => 40,
                    '4' => 35,
                    '4.5' => 30,
                    '5' => 25,
                ],
            ],
            'm' => [
                '10' => [
                    '1' => 50,
                    '1.5' => 45,
                    '2' => 40,
                    '2.5' => 35,
                    '3' => 30,
                    '3.5' => 25,
                    '4' => 20,
                    '4.5' => 15,
                    '5' => 10,
                ],
                '20' => [
                    '1' => 65,
                    '1.5' => 60,
                    '2' => 55,
                    '2.5' => 50,
                    '3' => 45,
                    '3.5' => 40,
                    '4' => 35,
                    '4.5' => 30,
                    '5' => 25,
                ],
            ],
        ];
    }
}

namespace {

    use Hungarofit\Evaluator\Exercise;
    use Hungarofit\Evaluator\Gender;
    use Hungarofit\Evaluator\Lookup\FirstExercise12min;
    use Hungarofit\Evaluator\Lookup\SecondExercise1km;
    use PHPUnit\Framework\TestCase;
    use Hungarofit\Evaluator\Unit;

    class ExerciseTest extends TestCase
    {
        function provideNameData()
        {
            return [
                [new Exercise(FirstExercise12min::class), 'first-exercise-12min'],
                [new Exercise(SecondExercise1km::class), 'second-exercise-1km'],
            ];
        }

        function provideUnitData()
        {
            return [
                [new Exercise(FirstExercise12min::class), Unit::MINUTE, Unit::METER, false, true],
                [new Exercise(SecondExercise1km::class), Unit::KILOMETER, Unit::MINUTE, true, false],
            ];
        }

        /**
         * @dataProvider provideNameData
         * @param \Hungarofit\Evaluator\ExerciseInterface $x
         * @param string $n
         */
        public function testName($x, $n)
        {
            $this->assertEquals($n, $x->getName());
        }

        /**
         * @dataProvider provideUnitData
         * @param \Hungarofit\Evaluator\ExerciseInterface $x
         * @param string $a
         * @param string $b
         * @param bool $ba
         * @param bool $bb
         */
        public function testUnit($x, $a, $b, $ba, $bb)
        {
            $this->assertEquals($a, $x->getExerciseUnit()->getValue());
            $this->assertEquals($b, $x->getResultUnit()->getValue());
            $this->assertEquals($ba, $x->getExerciseUnit()->isAscending());
            $this->assertEquals($bb, $x->getResultUnit()->isAscending());
        }

        public function testEvaluate()
        {
            $x = new Exercise(FirstExercise12min::class);
            $this->assertEquals(11, $x->evaluate(Gender::MALE(), 24, 45));
        }
    }

}