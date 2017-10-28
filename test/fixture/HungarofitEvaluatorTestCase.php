<?php


use PHPUnit\Framework\TestCase;

class HungarofitEvaluatorTestCase extends TestCase
{
    protected function _invokeMethod(&$object, $methodName, array $parameters=[])
    {
        $reflection = new \ReflectionClass(get_class($object));
        $method = $reflection->getMethod($methodName);
        $method->setAccessible(true);
        return $method->invokeArgs($object, $parameters);
    }

    protected function _getProperty(&$object, $propertyName, $static=false)
    {
        $reflection = new \ReflectionClass(get_class($object));
        if($static) {
            return $reflection->getStaticPropertyValue($propertyName);
        }
        $property = $reflection->getProperty($propertyName);
        $property->setAccessible(true);
        return $property->getValue($object);
    }
}