import {Evaluator, Challenge, Gender, Exercise} from "../pkg/bundler";

function case1() {
  const evaluator = new Evaluator<Challenge.Hungarofit>(Exercise.AerobRun12Min);
  return evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case2() {
  const evaluator = new Evaluator<Challenge.Hungarofit>(Exercise.AerobRun12Min)
    .withAge(20);
  return evaluator.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case3() {
  const evaluator = new Evaluator<Challenge.Hungarofit>(Exercise.AerobRun12Min)
    .withGender(Gender.Male);
  return evaluator.evaluate(20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function case4() {
  const evaluator = new Evaluator<Challenge.Hungarofit>(Exercise.AerobRun12Min)
    .withAge(20)
    .withGender(Gender.Male);
  return evaluator.evaluate(2600, 2.4, 40, 70, 90, 1.4, 1.2);
}

function caseMini1() {
  const evaluator = new Evaluator<Challenge.HungarofitMini>(Exercise.AerobRun12Min);
  return evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90);
}

function caseMini2() {
  const evaluator = new Evaluator<Challenge.HungarofitMini>(Exercise.AerobRun12Min)
    .withAge(20);
  return evaluator.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90);
}

function caseMini3() {
  const evaluator = new Evaluator<Challenge.HungarofitMini>(Exercise.AerobRun12Min)
    .withGender(Gender.Male);
  return evaluator.evaluate(20, 2600, 2.4, 40, 70, 90);
}

function caseMini4() {
  const evaluator = new Evaluator<Challenge.HungarofitMini>(Exercise.AerobRun12Min)
    .withAge(20)
    .withGender(Gender.Male);
  return evaluator.evaluate(2600, 2.4, 40, 70, 90);
}

/**
 * evaluate() : Evaluation should have scheme:
 * {
 *   evaluation: 6,
 *   total_score: 110,
 *   exercise_scores: [56, 14, 5, 4, 9],
 * }
 */
