<?php

namespace Hungarofit\Evaluator;


interface ChallengeInterface
{
    /**
     * @param ExerciseInterface $exercise
     * @return bool
     */
    function isValidExercise(ExerciseInterface $exercise);

    /**
     * @return ExerciseInterface[]
     */
    function getValidExercises();

    /**
     * @return ExerciseInterface[]
     */
    function getRequiredExercises();

    /**
     * @param Gender $gender
     * @return $this
     */
    function setGender(Gender $gender);

    /**
     * @param int $age
     * @return $this
     */
    function setAge($age);

    /**
     * @param ExerciseInterface  $exercise
     * @param float $result
     * @return $this
     */
    function setResult(ExerciseInterface  $exercise, $result);

    /**
     * @return bool
     */
    function hasAerob();

    /**
     * @return bool
     * @throws Validate\Exception
     */
    function validate();

    /**
     * @return float[]
     * @throws Validate\Exception
     */
    function evaluate();

    /**
     * @return Rating
     * @throws Validate\Exception
     */
    function rate();

    /**
     * @param bool $all
     * @return $this
     */
    function clear($all=false);
}