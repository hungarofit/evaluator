<?php

use Hungarofit\Evaluator\Gender;
use Hungarofit\Evaluator\Exercise;
use Hungarofit\Evaluator\Lookup\AerobBike12min;

class LookupTest extends HungarofitEvaluatorTestCase
{
    public function testAgeArgument()
    {
        $lu = new Exercise(AerobBike12min::class);
        $this->expectException(InvalidArgumentException::class);
        $lu->evaluate(Gender::FEMALE(), 0, 0);
        $this->expectException(InvalidArgumentException::class);
    }

    public function testResultArgument()
    {
        $lu = new Exercise(AerobBike12min::class);
        $this->expectException(InvalidArgumentException::class);
        $lu->evaluate(Gender::FEMALE(), 0, 0);
    }

    public function testMinResultArgument()
    {
        $lu = new Exercise(AerobBike12min::class);
        $this->expectException(InvalidArgumentException::class);
        $lu->getMinResult(Gender::FEMALE(), 0);
    }

    public function testMinKeys()
    {
        $lu = new Exercise(AerobBike12min::class);
        $minAge = $lu->getMinAge();
        $this->assertGreaterThan(0, $minAge);
        $this->assertGreaterThan(0, $lu->getMinResult(Gender::FEMALE(), $minAge));
    }

    public function off_testLookup()
    {
        $table = [
            Gender::FEMALE() => [
                '20' => [
                    '3.0' => 60,
                    '2.5' => 50,
                    '2.0' => 40,
                    '1.5' => 30,
                    '1.0' => 20,
                ],
                '11' => [
                    '3.0' => 55,
                    '2.5' => 44,
                    '2.0' => 33,
                    '1.5' => 22,
                    '1.0' => 11,
                ],
                '10' => [
                    '3.0' => 50,
                    '2.5' => 40,
                    '2.0' => 30,
                    '1.5' => 20,
                    '1.0' => 10,
                ],
            ]
        ];
        $lu = new Exercise(AerobBike12min::class);
        $refClass = new ReflectionClass($lu);
        $refClass->setStaticPropertyValue('TABLE', $table);
        $this->assertEquals(null, $lu->evaluate(Gender::FEMALE(), 1, 1));
        $this->assertEquals(0, $lu->evaluate(Gender::FEMALE(), 10, 1));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 10, 10));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 10, 11));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 10, 19));
        $this->assertEquals(1.5, $lu->evaluate(Gender::FEMALE(), 10, 20));
        $this->assertEquals(3.0, $lu->evaluate(Gender::FEMALE(), 10, 50));
        $this->assertEquals(3.0, $lu->evaluate(Gender::FEMALE(), 10, 100));
        $this->assertEquals(0, $lu->evaluate(Gender::FEMALE(), 11, 10));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 11, 11));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 19, 11));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 20, 20));
        $this->assertEquals(1.0, $lu->evaluate(Gender::FEMALE(), 20, 29));
        $this->assertEquals(1.5, $lu->evaluate(Gender::FEMALE(), 20, 30));
        $this->assertEquals(3.0, $lu->evaluate(Gender::FEMALE(), 20, 60));
    }

    public function provideLookupClasses()
    {
        $r = [];
        $di = new DirectoryIterator(realpath(__DIR__ . '/../..//src/Hungarofit/Evaluator/Lookup'));
        foreach ($di as $classFile) {
            if ($classFile->getExtension() !== 'php') {
                continue;
            }
            $r[] = [
                '\Hungarofit\Evaluator\Lookup\\' . $classFile->getBasename('.php')
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
