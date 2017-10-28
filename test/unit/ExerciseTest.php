<?php

use PHPUnit\Framework\TestCase;
use Hungarofit\Evaluator\Gender;
use Hungarofit\Evaluator\Unit;
use Hungarofit\Evaluator\ExerciseInterface;
use Hungarofit\Evaluator\Exercise\AerobBike12min;
use Hungarofit\Evaluator\Exercise\AerobRun3km;
use Hungarofit\Evaluator\Exercise\Motor4Situp;


class ExerciseTest extends TestCase
{
    public function testSingleton()
    {
        $a1 = AerobBike12min::get();
        $a2 = AerobBike12min::get();
        $b = AerobRun3km::get();
        $this->assertTrue($a1 instanceof ExerciseInterface);
        $this->assertTrue($a2 instanceof ExerciseInterface);
        $this->assertTrue($b instanceof ExerciseInterface);
        $this->assertEquals($a1, $a2);
        $this->assertNotEquals($a1, $b);
    }

    public function testLimit()
    {
        $this->assertEquals(1, AerobBike12min::get()->getMinPoints(Gender::MALE(), 13));
        $this->assertEquals(1, AerobRun3km::get()->getMinPoints(Gender::MALE(), 8));
        $this->assertEquals(3850, AerobBike12min::get()->getMinResult(Gender::MALE(), 13));
        $this->assertEquals(28.34, AerobRun3km::get()->getMinResult(Gender::MALE(), 8));
    }


    public function testAgeArgument()
    {
        $exercise = AerobBike12min::get();
        $this->expectException(InvalidArgumentException::class);
        $exercise->evaluate(Gender::FEMALE(), 0, -1);
        $this->expectException(InvalidArgumentException::class);
    }

    public function testResultArgument()
    {
        $exercise = AerobBike12min::get();
        $this->expectException(InvalidArgumentException::class);
        $exercise->evaluate(Gender::FEMALE(), 1, -1);
    }

    public function testMinResultArgument()
    {
        $exercise = AerobBike12min::get();
        $this->expectException(InvalidArgumentException::class);
        $exercise->getMinResult(Gender::FEMALE(), 0);
    }

    public function testMinKeys()
    {
        $exercise = AerobBike12min::get();
        $minAge = $exercise->getMinAge();
        $this->assertGreaterThan(0, $minAge);
        $this->assertGreaterThan(0, $exercise->getMinResult(Gender::FEMALE(), $minAge));
    }
    
    function testIsAerob()
    {
        $this->assertTrue(AerobBike12min::get()->isAerob());
        $this->assertFalse(Motor4Situp::get()->isAerob());
    }

    function provideNameData()
    {
        return [
            [AerobBike12min::get(), 'aerob-bike-12min', 'aerob-bike-12min'],
            [AerobRun3km::get(), 'aerob-run-3km', 'aerob-run-3km'],
            [Motor4Situp::get(), 'motor4-situp', 'situp'],
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

    function provideUnitData()
    {
        return [
            [AerobBike12min::get(), Unit::MINUTE, Unit::METER, false, true],
            [AerobRun3km::get(), Unit::KILOMETER, Unit::MINUTE, true, false],
            [Motor4Situp::get(), Unit::COUNT, Unit::COUNT, true, true],
        ];
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

    function provideEvaluate()
    {
        return [
            [AerobBike12min::get(), Gender::MALE(), 24, 0, 0],
            [AerobBike12min::get(), Gender::MALE(), 24, 3449, 0],
            [AerobBike12min::get(), Gender::MALE(), 24, 3450, 1],
            [AerobBike12min::get(), Gender::MALE(), 24, 3519, 1],
            [AerobBike12min::get(), Gender::MALE(), 24, 3520, 2],
            [AerobBike12min::get(), Gender::MALE(), 24, 3589, 2],
            [AerobBike12min::get(), Gender::MALE(), 24, 3590, 3],
            [AerobBike12min::get(), Gender::MALE(), 24, 3659, 3],
            [AerobBike12min::get(), Gender::MALE(), 24, 3660, 4],
            [AerobBike12min::get(), Gender::MALE(), 24, 8850, 77],
            [AerobBike12min::get(), Gender::MALE(), 24, 9999, 77],
        ];
    }

    /**
     * @dataProvider provideEvaluate
     * @param ExerciseInterface $exercise
     * @param Gender $gender
     * @param int $age
     * @param float $result
     * @param float $points
     */
    public function testEvaluate(ExerciseInterface $exercise, Gender $gender, $age, $result, $points)
    {
        $this->assertEquals($points, $exercise->evaluate($gender, $age, $result));
    }

    public function provideLookupClasses()
    {
        $r = [];
        $di = new DirectoryIterator(realpath(__DIR__ . '/../../src/Hungarofit/Evaluator/Exercise'));
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
