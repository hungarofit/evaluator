# Hungarofit Evaluator

WASM library for browsers

## Usage

### Basic

```html
<!DOCTYPE html>
<html>
  <body>
    <code><pre id="result-output"></pre></code>
    <script type="module">
      //
      import {
        default as initHfit,
        Evaluator,
        EvaluatorMini,
        Gender,
        Exercise,
        exerciseFromString,
      } from "https://cdn.jsdelivr.net/npm/@hungarofit/evaluator-browser@0.0.23/evaluator.js";
      async function initializeEvaluator() {
        try {
          // await the initialization function to fetch and load WASM binary
          // should be done once globally
          await initHfit();

          // instance evaluate and get score of results
          const aerobExercise = exerciseFromString("aerob-run-12min");
          // const aerobExercise = Exercise.AerobRun12min;
          const evaluator = new EvaluatorMini(aerobExercise);
          const score = evaluator.evaluate(
            Gender.Female,
            20,
            1900,
            2.4,
            42,
            42,
            42
          );

          // score is a pointer object to WASM, use toJSON to grab js object
          console.log(score.toJSON());

          // show in html
          const output = document.getElementById("result-output");
          output.textContent += `${JSON.stringify(score.toJSON(), null ,"  ")}\n`;
          
          // evaluator2 shares age + gender for subsequent evaluations
          const evaluator2 = new Evaluator(Exercise.AerobRun12Min)
            .with_age(20)
            .with_gender(Gender.Male);
          const score3 = evaluator2.evaluate(2600, 2.4, 40, 70, 90, 1.4, 1.2);
          console.log(score3.toJSON());
          const score4 = evaluator2.evaluate(2700, 2.6, 50, 80, 100, 1.6, 1.4);
          console.log(score4.toJSON());
        } catch (e) {
          console.error(e);
          document.getElementById("result-output").textContent = "Error loading evaluator";
        }
      }
      initializeEvaluator();
    </script>
  </body>
</html>
```
