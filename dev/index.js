import {Gender, Exercise, evaluate, evaluateMini, exerciseFromString, aerobExerciseList} from "../pkg/bundler/evaluator.js";

const aerobExercises = aerobExerciseList();
const aerobExercise0 = exerciseFromString(aerobExerciseList[0]);
const aerobExercise1 = exerciseFromString("aerob-run-12min");
const aerobExercise2 = Exercise.AerobRun12Min;

// Full evaluation example
function caseFull() {
  return evaluate(
    Gender.Male,
    20,
    aerobExercise1,
    2600,  // aerob result (meters)
    2.4,   // long jump (meters)
    40,    // push-ups (count)
    70,    // sit-ups (count)
    90,    // trunk lift (count)
    1.4,   // medicine ball throw double hand (meters)
    1.2    // medicine ball throw single hand (meters)
  );
}

// Mini evaluation example
function caseMini() {
  return evaluateMini(
    Gender.Male,
    20,
    aerobExercise2,
    2600,  // aerob result (meters)
    2.4,   // long jump (meters)
    40,    // push-ups (count)
    70,    // sit-ups (count)
    90     // trunk lift (count)
  );
}

// Female full evaluation example
function caseFullFemale() {
  return evaluate(
    Gender.Female,
    20,
    Exercise.AerobRun2Km,
    1800,  // aerob result (meters)
    1.9,   // long jump (meters)
    25,    // push-ups (count)
    50,    // sit-ups (count)
    70,    // trunk lift (count)
    1.0,   // medicine ball throw double hand (meters)
    0.8    // medicine ball throw single hand (meters)
  );
}

// Female mini evaluation example
function caseMinieFemale() {
  return evaluateMini(
    Gender.Female,
    20,
    Exercise.AerobRun6Min,
    1200,  // aerob result (meters)
    1.9,   // long jump (meters)
    25,    // push-ups (count)
    50,    // sit-ups (count)
    70     // trunk lift (count)
  );
}

console.log("Full Evaluation (Male):");
const fullResult = caseFull();
console.log(fullResult);
console.log(fullResult.toJSON());
fullResult.free();

console.log("\nMini Evaluation (Male):");
const miniResult = caseMini();
console.log(miniResult);
console.log(miniResult.toJSON());
miniResult.free();

console.log("\nFull Evaluation (Female):");
const fullResultFemale = caseFullFemale();
console.log(fullResultFemale);
console.log(fullResultFemale.toJSON());
fullResultFemale.free();

console.log("\nMini Evaluation (Female):");
const miniResultFemale = caseMinieFemale();
console.log(miniResultFemale);
console.log(miniResultFemale.toJSON());
miniResultFemale.free();
