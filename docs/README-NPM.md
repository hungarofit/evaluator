# Hungarofit Evaluator

WASM library for NodeJS backends

## Install

```bash
npm install --save @hungarofit/evaluator
```

## Usage

### Basic

```js
import {Gender, Exercise, evaluate, evaluateMini, exerciseFromString} from "@hungarofit/evaluator";

// 1 aerob + 6 motor exercises
const score = evaluate(
  Gender.Male,      // gender
  20,               // age
  Exercise.AerobRun12Min,  // aerobic exercise type
  2600,             // aerobic result (meters)
  2.4,              // long jump (meters)
  40,               // push-ups (count)
  70,               // sit-ups (count)
  90,               // trunk lift (count)
  1.4,              // medicine ball throw double hand (meters)
  1.2               // medicine ball throw single hand (meters)
);
console.log(score.toJSON());

// 1 aerob + 4 motor exercises
const aerobExercise = exerciseFromString("aerob-run-12min");
const scoreMini = evaluateMini(
  Gender.Male,      // gender
  20,               // age
  aerobExercise,    // aerobic exercise type
  2600,             // aerobic result (meters)
  2.4,              // long jump (meters)
  40,               // push-ups (count)
  70,               // sit-ups (count)
  90                // trunk lift (count)
);
console.log(scoreMini.toJSON());
```

### Using exerciseFromString

```js
import {Gender, evaluate, evaluateMini, exerciseFromString} from "@hungarofit/evaluator";

const aerobExercise = exerciseFromString("aerob-run-12min");

const score1 = evaluate(
  Gender.Male,      // gender
  20,               // age
  aerobExercise,    // aerobic exercise type
  2600,             // aerobic result (meters)
  2.4,              // long jump (meters)
  40,               // push-ups (count)
  70,               // sit-ups (count)
  90,               // trunk lift (count)
  1.4,              // medicine ball throw double hand (meters)
  1.2               // medicine ball throw single hand (meters)
);
console.log(score1.toJSON());

const score2 = evaluateMini(
  Gender.Female,    // gender
  25,               // age
  aerobExercise,    // aerobic exercise type
  1900,             // aerobic result (meters)
  2.4,              // long jump (meters)
  42,               // push-ups (count)
  42,               // sit-ups (count)
  42                // trunk lift (count)
);
console.log(score2.toJSON());
```
