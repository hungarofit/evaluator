<?php

use Hungarofit\Evaluator\Text;
use PHPUnit\Framework\TestCase;

class TextTest extends TestCase
{
    public function provideCamelKebab()
    {
        return [
            ['aerob-run-12km', 'AerobRun12km'],
            ['aerob-run-1mile5', 'AerobRun1mile5'],
            ['motor6-torso', 'Motor6Torso'],
        ];
    }

    /**
     * @dataProvider provideCamelKebab
     * @param $i
     * @param $e
     */
    public function testCamelcase($i, $e)
    {
        $this->assertEquals($e, Text::camelcase($i));
    }

    /**
     * @dataProvider provideCamelKebab
     * @param $e
     * @param $i
     */
    public function testKebabcase($e, $i)
    {
        $this->assertEquals($e, Text::kebabcase($i));
    }
}
