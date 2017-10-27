<?php

namespace Hungarofit\Evaluator;


use ReflectionClass;
use RuntimeException;
use InvalidArgumentException;

class Exercise implements ExerciseInterface
{
    const UNIT_EXERCISE = '';
    const UNIT_RESULT = '';
    const TABLE = [];

    /** @var static */
    static protected $_instance;

    /**
     * @return static
     * @throws InvalidArgumentException
     */
    static public function get()
    {
        $constants = (new ReflectionClass(static::class))->getConstants();
        foreach(['TABLE','UNIT_EXERCISE','UNIT_RESULT'] as $c) {
            if(!array_key_exists($c, $constants)) {
                throw new InvalidArgumentException('Exercise class ('.static::class.') must declare constant: '.$c);
            }
        }
        if(!static::$_instance) {
            self::$_instance = new self;
        }
        return self::$_instance;
    }

    /** @var string */
    private $_name;
    /** @var Unit */
    private $_exerciseUnit;
    /** @var Unit */
    private $_resultUnit;

    /** @var string */
    protected $_lookupClass;

    protected function __construct()
    {
        $nameSplit = explode('\\', static::class);
        $nameClass = array_pop($nameSplit);
        $this->_name = Text::kebabcase($nameClass);
        $this->_exerciseUnit = Unit::fromValue(static::UNIT_EXERCISE);
        $this->_resultUnit = Unit::fromValue(static::UNIT_RESULT);
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
        $table = self::TABLE[$gender->getValue()];
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
        $table = self::TABLE[$gender->getValue()];
        foreach($table as $a => $v) {
            $a = intval($a);
            if($age < $a) {
                continue;
            }
            return floatval(array_pop($v));
        }
        throw new RuntimeException('Failed to find rows for age: '.$age);
    }

    /**
     * @inheritdoc
     * @throws RuntimeException
     * @throws InvalidArgumentException
     */
    function getMinPoints(Gender $gender, $age)
    {
        if($age < 1) {
            throw new InvalidArgumentException('Invalid age: '.$age);
        }
        $table = self::TABLE[$gender->getValue()];
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
        $ascending = $this->getResultUnit()->isAscending();
        $table = self::TABLE[$gender->getValue()];
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
                if($ascending) {
                    // array is reversed!
                    if($result < $r) {
                        continue;
                    }
                }
                else {
                    // array is reversed!
                    if($result > $r) {
                        continue;
                    }
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