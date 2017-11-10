<?php

namespace Hungarofit\Evaluator;


use InvalidArgumentException;

abstract class Challenge implements ChallengeInterface
{
    const RATING = [20.5, 40.5, 60.5, 80.5, 100.5, 120.5, 140];

    /** @var Gender */
    protected $_gender;
    /** @var int */
    protected $_age;
    /** @var float[] */
    protected $_results;

    public function __construct(Gender $gender=null, $age=null)
    {
        if($gender) {
            $this->setGender($gender);
        }
        if($age > 0) {
            $this->setAge($age);
        }
    }

    abstract function getRequiredExercises();

    function __call($name, $arguments)
    {
        if(strlen($name) > 3 && substr($name, 0, 3) === 'set' && strtolower($name[3]) !== $name[3]) {
            $key = Text::kebabcase(substr($name, 3));
            foreach($this->getValidExercises() as $exercise) {
                $eKey = $exercise->getName();
                if($eKey === $key) {
                    $this->setResult($exercise, $arguments[0]);
                    return $this;
                }
            }
        }
        throw new \RuntimeException('No such method: '.$name);
    }

    function getValidExercises()
    {
        $valid = [];
        foreach($this->getRequiredExercises() as $x1) {
            if(is_array($x1)) {
                foreach($x1 as $x2) {
                    $valid[] = $x2;
                }
            }
            else {
                $valid[] = $x1;
            }
        }
        return $valid;
    }

    function isValidExercise(ExerciseInterface $exercise)
    {
        return in_array($exercise, $this->getValidExercises(), true);
    }

    function setGender(Gender $gender)
    {
        $this->_gender = $gender;
        return $this;
    }

    function setAge($age)
    {
        if($age < 1) {
            throw new InvalidArgumentException('Invalid age: '.$age);
        }
        $this->_age = $age;
        return $this;
    }

    function setResult(ExerciseInterface $exercise, $result)
    {
        if(!$this->isValidExercise($exercise)) {
            throw new InvalidArgumentException('Invalid exercise ' . get_class($exercise) . ' for challenge ' . get_class($this));
        }
        $this->_results[get_class($exercise)] = $result;
        return $this;
    }

    function validate()
    {
        $errors = [];
        foreach($this->getRequiredExercises() as $requireAnd) {
            if(is_array($requireAnd)) {
                $or = false;
                foreach($requireAnd as $requireOr) {
                    if(array_key_exists(get_class($requireOr), $this->_results)) {
                        $or = true;
                        break;
                    }
                }
                if(!$or) {
                    foreach($requireAnd as $requireOr) {
                        $errors[$requireOr->getKey()] = 'is optionally required';
                    }
                }
            }
            else {
                if(!array_key_exists(get_class($requireAnd), $this->_results)) {
                    $errors[$requireAnd->getKey()] = 'is required';
                }
            }
        }
        $minAge = 1;
        foreach($this->_results as $class => $result) {
            /** @var ExerciseInterface $exercise */
            $exercise = $class::get();
            $minAge = max($minAge, $exercise->getMinAge());
            if($result < 0) {
                $errors[$exercise->getKey()] = 'invalid result: '.$result;
            }
        }
        if($this->_age < $minAge) {
            $errors['age'] = ' ('.$this->_age.') is less than the minimum '.$minAge;
        }
        if(count($errors) > 0) {
            throw new Validate\Exception($errors);
        }
        return true;
    }

    function evaluate()
    {
        $this->validate();
        $points = [];
        foreach($this->_results as $exerciseClass => $result) {
            /** @var ExerciseInterface $exercise */
            $exercise = $exerciseClass::get();
            /*
            if(array_key_exists($exercise->getKey(), $points)) {
                throw new RuntimeException($exercise->getKey() . ' ('.$exercise->getName().') exercise name is already in use');
            }
            */
            $points[$exercise->getKey()] = $exercise->evaluate($this->_gender, $this->_age, $result);
        }
        return $points;
    }

    function rate()
    {
        $points = $this->evaluate();
        $sum = array_sum($points);
        $rating = 7;
        foreach(self::RATING as $k => $v) {
            if($sum > $v) {
                continue;
            }
            $rating = $k+1;
            break;
        }
        return Rating::fromValue($rating);
    }

    function clear($all = false)
    {
        $this->_results = [];
        if($all) {
            unset($this->_gender);
            unset($this->_age);
        }
        return $this;
    }

}