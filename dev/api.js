import {Gender, Exercise, Evaluator, EvaluatorMini, exerciseFromString} from "../pkg/bundler/evaluator.js";

const aerobExercise = exerciseFromString("aerob-run-12min");
const aerobExercise2 = Exercise.AerobRun12Min

function case1() {
  let evaluator = new Evaluator(aerobExercise);
  return evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case2() {
  const evaluator = new Evaluator(aerobExercise)
    .with_age(20);
  return evaluator.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case3() {
  const evaluator = new Evaluator(aerobExercise)
    .with_gender(Gender.Male);
  return evaluator.evaluate(20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case4() {
  const evaluator = new Evaluator(aerobExercise)
    .with_age(20)
    .with_gender(Gender.Male);
  return evaluator.evaluate(2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function caseMini1() {
  const evaluator = new EvaluatorMini(aerobExercise2);
  return evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90);
}

function caseMini2() {
  const evaluator = new EvaluatorMini(aerobExercise2)
    .with_age(20);
  return evaluator.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90);
}

function caseMini3() {
  const evaluator = new EvaluatorMini(aerobExercise2)
    .with_gender(Gender.Male);
  return evaluator.evaluate(20, 2600, 2.4, 40, 70, 90);
}

function caseMini4() {
  const evaluator = new EvaluatorMini(aerobExercise2)
    .with_age(20)
    .with_gender(Gender.Male);
  return evaluator.evaluate(2600, 2.4, 40, 70, 90);
}

const rm4 = caseMini4();
console.log(rm4);
console.log(rm4.toJSON());
