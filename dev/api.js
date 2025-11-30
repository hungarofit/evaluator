import {Gender, Exercise, Evaluator, EvaluatorMini} from "../pkg/bundler/evaluator.js";

function case1() {
  let evaluator = new Evaluator(Exercise.AerobRun12Min);
  return evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case2() {
  const evaluator = new Evaluator(Exercise.AerobRun12Min)
    .with_age(20);
  return evaluator.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case3() {
  const evaluator = new Evaluator(Exercise.AerobRun12Min)
    .with_gender(Gender.Male);
  return evaluator.evaluate(20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case4() {
  const evaluator = new Evaluator(Exercise.AerobRun12Min)
    .with_age(20)
    .with_gender(Gender.Male);
  return evaluator.evaluate(2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function caseMini1() {
  const evaluator = new EvaluatorMini(Exercise.AerobRun12Min);
  return evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90);
}

function caseMini2() {
  const evaluator = new EvaluatorMini(Exercise.AerobRun12Min)
    .with_age(20);
  return evaluator.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90);
}

function caseMini3() {
  const evaluator = new EvaluatorMini(Exercise.AerobRun12Min)
    .with_gender(Gender.Male);
  return evaluator.evaluate(20, 2600, 2.4, 40, 70, 90);
}

function caseMini4() {
  const evaluator = new EvaluatorMini(Exercise.AerobRun12Min)
    .with_age(20)
    .with_gender(Gender.Male);
  return evaluator.evaluate(2600, 2.4, 40, 70, 90);
}

const rm4 = caseMini4();
console.log(rm4);
console.log(rm4.toJSON());
