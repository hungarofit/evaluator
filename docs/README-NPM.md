# Hungarofit Evaluator

WASM library for NodeJS backends

## Install

```bash
npm install --save @hungarofit/evaluator
```

## Usage

### Basic

```js
import {Gender, Exercise, Evaluator, EvaluatorMini, exerciseFromString} from "@hungarofit/evaluator";

// 1 aerob + 6 motor exercises
const evaluator = new Evaluator(Exercise.AerobRun12Min);
const score = evaluator.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
console.log(score.toJSON());

// 1 aerob + 4 motor exercises
const aerobExercise = exerciseFromString("aerob-run-12min");
const evaluatorMini = new EvaluatorMini(aerobExercise);
const scoreMini = evaluatorMini.evaluate(Gender.Male, 20, 2600, 2.4, 40, 70, 90);
console.log(scoreMini.toJSON());
```

### Builder

```js
import {Gender, Exercise, Evaluator, EvaluatorMini} from "@hungarofit/evaluator";

// evaluator1 shares age for subsequent evaluations
const evaluator1 = new Evaluator(Exercise.AerobRun12Min)
    .with_age(20);
const score1 = evaluator1.evaluate(Gender.Male, 2600, 2.4, 40, 70, 90, 1.4, 1.2);
console.log(score1.toJSON());
const score2 = evaluator1.evaluate(Gender.Male, 2700, 2.6, 50, 80, 100, 1.6, 1.4);
console.log(score2.toJSON());

// evaluator2 shares age + gender for subsequent evaluations
const evaluator2 = new Evaluator(Exercise.AerobRun12Min)
    .with_age(20)
    .with_gender(Gender.Male);
const score3 = evaluator2.evaluate(2600, 2.4, 40, 70, 90, 1.4, 1.2);
console.log(score3.toJSON());
const score4 = evaluator2.evaluate(2700, 2.6, 50, 80, 100, 1.6, 1.4);
console.log(score4.toJSON());
```
