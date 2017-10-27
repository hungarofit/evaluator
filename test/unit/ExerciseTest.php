<?php

use PHPUnit\Framework\TestCase;
use Hungarofit\Evaluator\Gender;
use Hungarofit\Evaluator\Unit;
use Hungarofit\Evaluator\Exercise;
use Hungarofit\Evaluator\Exercise\AerobBike12min;

final class FirstExercise12min extends Exercise
{
    const UNIT_EXERCISE = Unit::MINUTE;
    const UNIT_RESULT = Unit::METER;
    const TABLE = [
        'f' => [
            '20' => [
                '5' => 65.5,
                '4' => 55.4,
                '3' => 45.3,
                '2' => 35.2,
                '1' => 25.1,
            ],
            '15' => [
                '5' => 55.5,
                '4' => 45.4,
                '3' => 35.3,
                '2' => 25.2,
                '1' => 15.1,
            ],
            '10' => [
                '5' => 50.5,
                '4' => 40.4,
                '3' => 30.3,
                '2' => 20.2,
                '1' => 10.1,
            ],
        ],
        'm' => [
            '20' => [
                '5' => 65.5,
                '4' => 55.4,
                '3' => 45.3,
                '2' => 35.2,
                '1' => 25.1,
            ],
            '15' => [
                '5' => 55.5,
                '4' => 45.4,
                '3' => 35.3,
                '2' => 25.2,
                '1' => 15.1,
            ],
            '10' => [
                '5' => 50.5,
                '4' => 40.4,
                '3' => 30.3,
                '2' => 20.2,
                '1' => 10.1,
            ],
        ],
    ];
}

final class SecondExercise1km extends Exercise
{
    const UNIT_EXERCISE = Unit::KILOMETER;
    const UNIT_RESULT = Unit::MINUTE;
    const TABLE = [
        'f' => [
            '20' => [
                '5' => 25,
                '4.5' => 30,
                '4' => 35,
                '3.5' => 40,
                '3' => 45,
                '2.5' => 50,
                '2' => 55,
                '1.5' => 60,
                '1' => 65,
            ],
            '10' => [
                '5' => 10,
                '4.5' => 15,
                '4' => 20,
                '3.5' => 25,
                '3' => 30,
                '2.5' => 35,
                '2' => 40,
                '1.5' => 45,
                '1' => 50,
            ],
        ],
        'm' => [
            '20' => [
                '5' => 125,
                '4.5' => 130,
                '4' => 135,
                '3.5' => 140,
                '3' => 145,
                '2.5' => 150,
                '2' => 155,
                '1.5' => 160,
                '1' => 165,
            ],
            '10' => [
                '5' => 110,
                '4.5' => 115,
                '4' => 120,
                '3.5' => 125,
                '3' => 130,
                '2.5' => 135,
                '2' => 140,
                '1.5' => 145,
                '1' => 150,
            ],
        ],
    ];
}



class ExerciseTest extends TestCase
{
    /** @var Exercise */
    static protected $first, $second;

    static function setUpBeforeClass()
    {
        self::$first = FirstExercise12min::get();
        self::$second = FirstExercise12min::get();
    }

    function provideNameData()
    {
        return [
            [self::$first, 'first-exercise-12min'],
            [self::$second, 'second-exercise-1km'],
        ];
    }

    function provideUnitData()
    {
        return [
            [self::$first, Unit::MINUTE, Unit::METER, false, true],
            [self::$second, Unit::KILOMETER, Unit::MINUTE, true, false],
        ];
    }

    function provideEvaluate()
    {
        return [
            [0, Gender::MALE(), 24, 25.099999],
            [1, Gender::MALE(), 24, 25.1],
            [1, Gender::MALE(), 24, 25.199999],
            [2, Gender::MALE(), 24, 35.2],
            [2, Gender::MALE(), 24, 35.299999],
            [3, Gender::MALE(), 24, 45.3],
            [3, Gender::MALE(), 24, 55.399999],
            [4, Gender::MALE(), 24, 55.4],
        ];
    }

    public function testSingleton()
    {
        $a1 = FirstExercise12min::get();
        $a2 = FirstExercise12min::get();
        $b = SecondExercise1km::get();
        $this->assertTrue($a1 instanceof Exercise);
        $this->assertTrue($a2 instanceof Exercise);
        $this->assertTrue($b instanceof Exercise);
        $this->assertEquals($a1, $a2);
        $this->assertNotEquals($a1, $b);
    }

    public function provideLookupClasses()
    {
        $r = [];
        $di = new DirectoryIterator(realpath(__DIR__ . '/../..//src/Hungarofit/Evaluator/Exercise'));
        foreach ($di as $classFile) {
            if ($classFile->getExtension() !== 'php') {
                continue;
            }
            $r[] = [
                '\Hungarofit\Evaluator\Exercise\\' . $classFile->getBasename('.php')
            ];
        }
        return $r;
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

    /**
     * @dataProvider provideEvaluate
     * @param float $points
     * @param Gender $gender
     * @param int $age
     * @param float $result
     */
    public function testEvaluate($points, Gender $gender, $age, $result)
    {
        $this->assertEquals($points, self::$first->evaluate($gender, $age, $result));
    }

    public function testLimit()
    {
        $this->assertEquals(1, self::$first->getMinPoints(Gender::MALE(), 12));
        $this->assertEquals(10.1, self::$first->getMinResult(Gender::MALE(), 12));
        $this->assertEquals(150, self::$second->getMinResult(Gender::MALE(), 12));
        $this->assertEquals(50, self::$second->getMinResult(Gender::FEMALE(), 12));
    }


    public function testAgeArgument()
    {
        $lu = AerobBike12min::get();
        $this->expectException(InvalidArgumentException::class);
        $lu->evaluate(Gender::FEMALE(), 0, 0);
        $this->expectException(InvalidArgumentException::class);
    }

    public function testResultArgument()
    {
        $lu = AerobBike12min::get();
        $this->expectException(InvalidArgumentException::class);
        $lu->evaluate(Gender::FEMALE(), 0, 0);
    }

    public function testMinResultArgument()
    {
        $lu = AerobBike12min::get();
        $this->expectException(InvalidArgumentException::class);
        $lu->getMinResult(Gender::FEMALE(), 0);
    }

    public function testMinKeys()
    {
        $lu = AerobBike12min::get();
        $minAge = $lu->getMinAge();
        $this->assertGreaterThan(0, $minAge);
        $this->assertGreaterThan(0, $lu->getMinResult(Gender::FEMALE(), $minAge));
    }

    /**
     * @dataProvider provideLookupClasses
     * @param $className
     */
    public function testLookupData($className)
    {
        $table = $className::TABLE;
        foreach ($table as $gender => $genderTable) {
            $prevAge = 0;
            foreach ($genderTable as $age => $row) {
                $age = intval($age);
                if ($prevAge > 0) {
                    $this->assertLessThan($prevAge, $age, $className . ' ' . $gender . ':' . $age);
                } else {
                    $this->assertGreaterThan(0, $age, $className . ' ' . $gender . ':' . $age);
                }
                $prevAge = $age;
                $prevPoints = 0;
                $prevResult = 0;
                foreach ($row as $points => $result)  {
                    $msg = $className . ' ' . $gender . ':' . $age . ':' . $points;
                    $points = floatval($points);
                    if ($prevPoints > 0) {
                        $this->assertLessThan($prevPoints, $points, $msg);
                    } else {
                        $this->assertGreaterThan(0, $points, $msg);
                    }
                    $prevPoints = $points;
                    $result = floatval($result);
                    if ($prevResult > 0) {
                        // Order in source is reversed for better lookup, so checking is reversed too!
                        if( ! (Hungarofit\Evaluator\Unit::isAscendingValue($className::UNIT_RESULT))) {
                            $this->assertGreaterThan($prevResult, $result, $msg);
                        }
                        else {
                            $this->assertGreaterThan($result, $prevResult, $msg);
                        }
                    } else {
                        $this->assertGreaterThan(0, $result, $msg);
                    }
                    $prevResult = $result;
                }
            }
        }
    }
}
