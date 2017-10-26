<?php

namespace Hungarofit\Evaluator;


use RuntimeException;
use InvalidArgumentException;

class Exercise implements ExerciseInterface
{
    private $_name;
    private $_exerciseUnit;
    private $_resultUnit;
    protected $_lookupClass;

    public function __construct($lookupClass)
    {
        $this->setLookupClass($lookupClass);
    }

    public function setLookupClass($lookupClass)
    {
        if(!class_exists($lookupClass)) {
            throw new InvalidArgumentException('No such Lookup class: '.$lookupClass);
        }
        $this->_lookupClass = $lookupClass;

        $nameSplit = explode('\\', $lookupClass);
        $nameClass = array_pop($nameSplit);
        $this->_name = Text::kebabcase($nameClass);

        $this->_exerciseUnit = Unit::fromValue($lookupClass::UNIT_EXERCISE);
        $this->_resultUnit = Unit::fromValue($lookupClass::UNIT_RESULT);

        return $this;
    }

    /**
     * @return string
     */
    public function getName()
    {
        return $this->_name;
    }

    public function getExerciseUnit()
    {
        return $this->_exerciseUnit;
    }

    public function getResultUnit()
    {
        return $this->_resultUnit;
    }

    function getLookupClass()
    {
        return $this->_lookupClass;
    }

    /**
     * @inheritdoc
     * @throws RuntimeException
     * @throws InvalidArgumentException
     */
    function getMinAge(Gender $gender = null)
    {
        if(!$gender) {
            $gender = Gender::FEMALE();
        }
        $lu = $this->_lookupClass;
        $table = $lu::TABLE[$gender->getValue()];
        end($table);
        $a = key($table);
        if($a === null) {
            throw new RuntimeException('static::$TABLE is not populated');
        }
        return intval($a);
    }


    /**
     * @inheritdoc
     * @throws RuntimeException
     * @throws InvalidArgumentException
     */
    function getMinResult(Gender $gender, $age)
    {
        if($age < 1) {
            throw new InvalidArgumentException('Invalid age: '.$age);
        }
        $lu = $this->_lookupClass;
        $table = $lu::TABLE[$gender->getValue()];
        foreach($table as $a => $v) {
            $a = intval($a);
            if($age < $a) {
                continue;
            }
            end($v);
            $p = key($v);
            if($p === null) {
                break;
            }
            return floatval($p);
        }
        throw new RuntimeException('Failed to find rows for age: '.$age);
    }

    /**
     * @inheritdoc
     * @throws InvalidArgumentException
     */
    public function evaluate(Gender $gender, $age, $result)
    {
        if($age < 1) {
            throw new InvalidArgumentException('Invalid age: '.$age);
        }
        if($result <= 0) {
            throw new InvalidArgumentException('Invalid result: '.$result);
        }
        $lu = $this->_lookupClass;
        $table = $lu::TABLE[$gender->getValue()];
        $points = 0;
        $ageFound = false;
        foreach($table as $a => $row) {
            $a = intval($a);
            if($age < $a) {
                continue;
            }
            $ageFound = true;
            foreach($row as $p => $r) {
                $r = floatval($r);
                if($result < $r) {
                    continue;
                }
                $points = floatval($p);
                break;
            }
            break;
        }
        if(!$ageFound) {
            $points = null;
        }
        return $points;
    }
}