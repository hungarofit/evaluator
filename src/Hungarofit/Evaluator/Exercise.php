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

    /** @var static[] */
    static protected $_instances = [];

    /**
     * @return static
     * @throws InvalidArgumentException
     */
    static public function get()
    {
        $class = get_called_class();
        $constants = (new ReflectionClass($class))->getConstants();
        foreach(['TABLE','UNIT_EXERCISE','UNIT_RESULT'] as $c) {
            if(!array_key_exists($c, $constants)) {
                throw new InvalidArgumentException('Exercise class ('.$class.') must declare constant: '.$c);
            }
        }
        if(!array_key_exists($class, self::$_instances)) {
            self::$_instances[$class] = new $class();
        }
        return self::$_instances[$class];
    }

    /**
     * @param string $name
     * @return static
     */
    public static function fromName($name)
    {
        /** @var Exercise $class */
        $class = Exercise::class . '\\' . Text::camelcase($name);
        if(!class_exists($class)) {
            throw new InvalidArgumentException('No such exercise: '.$name);
        }
        return $class::get();
    }

    /** @var string */
    protected $_name;
    /** @var string */
    protected $_key;
    /** @var Unit */
    protected $_exerciseUnit;
    /** @var Unit */
    protected $_resultUnit;


    protected function __construct()
    {
        preg_match("/\\\?([^\\\]+)$/", get_called_class(), $match);
        $this->_key = Text::kebabcase($match[1]);
        unset($match);
        preg_match("/^([^\\-]+)\\-(.+)/", $this->_key, $match);
        switch($match[1]) {
            case 'motor3':
            case 'motor4':
            case 'motor6':
                $this->_name = $match[2];
                break;
            default:
                $this->_name = $this->_key;
        }
        $this->_key = $this->_name;
        $this->_exerciseUnit = Unit::fromValue(static::UNIT_EXERCISE);
        $this->_resultUnit = Unit::fromValue(static::UNIT_RESULT);
    }

    function isAerob()
    {
        return substr($this->getKey(), 0, 6) === 'aerob-';
    }


    public function getName()
    {
        return $this->_name;
    }

    public function getKey()
    {
        return $this->_key;
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
        $table = static::TABLE[$gender->getValue()];
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
        $table = static::TABLE[$gender->getValue()];
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
        $table = static::TABLE[$gender->getValue()];
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
        if($result < 0) {
            throw new InvalidArgumentException('Invalid result: '.$result);
        }
        $ascending = $this->getResultUnit()->isAscending();
        $table = static::TABLE[$gender->getValue()];
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