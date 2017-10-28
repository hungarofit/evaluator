<?php

class FirstEnum extends Hungarofit\Evaluator\Enum
{
    const FOO = 1;
    const BAR = 2;
    const BAZ = 'bax';
    const BAX = 'bax';
}

class SecondEnum extends Hungarofit\Evaluator\Enum
{
    const FOO = 2;
    const BAR = 1;
}

class EnumTest extends HungarofitEvaluatorTestCase
{
    public function testFromName()
    {
        $foo = FirstEnum::fromName('FOO');
        $this->assertEquals('FOO', $foo->getName());
        $this->assertEquals(FirstEnum::FOO, $foo->getValue());
    }

    public function testFromValue()
    {
        $foo = FirstEnum::fromValue(FirstEnum::FOO);
        $this->assertEquals('FOO', $foo->getName());
        $this->assertEquals(FirstEnum::FOO, $foo->getValue());
    }

    public function testFromMagic()
    {
        $foo = FirstEnum::FOO();
        $this->assertEquals('FOO', $foo->getName());
        $this->assertEquals(FirstEnum::FOO, $foo->getValue());
    }

    public function testSingleton()
    {
        $e1 = FirstEnum::FOO();
        $e2 = FirstEnum::fromName('FOO');
        $e3 = FirstEnum::fromValue(FirstEnum::FOO);
        $this->assertTrue($e1 === $e2);
        $this->assertTrue($e1 === $e3);
    }

    public function testInstances()
    {
        $a = FirstEnum::FOO();
        $this->assertEquals(FirstEnum::FOO, $a->getValue());
        $b = SecondEnum::FOO();
        $this->assertEquals(SecondEnum::FOO, $b->getValue());
    }

    public function testAmbiguity()
    {
        $a1 = FirstEnum::BAZ();
        $b2 = FirstEnum::BAX();
        $a3 = FirstEnum::fromName('BAZ');
        $b4 = FirstEnum::fromName('BAX');
        $a5 = FirstEnum::fromValue(FirstEnum::BAZ);
        $a6 = FirstEnum::fromValue(FirstEnum::BAX);
        $this->assertTrue($a1 === $a3);
        $this->assertTrue($a1 === $a5);
        $this->assertTrue($a1 === $a6);
        $this->assertTrue($b2 === $b4);
        $this->assertTrue($a1 !== $b2);
    }
}