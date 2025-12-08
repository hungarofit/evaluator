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
        evaluate,
        evaluateMini,
        Gender,
        Exercise,
        exerciseFromString,
      } from "https://cdn.jsdelivr.net/npm/@hungarofit/evaluator-browser@0.0.23/evaluator.js";
      async function initializeEvaluator() {
        try {
          // await the initialization function to fetch and load WASM binary
          // should be done once globally
          await initHfit();

          // evaluate and get score of results (1 aerob + 4 motor exercises)
          const aerobExercise = exerciseFromString("aerob-run-12min");
          // or use: const aerobExercise = Exercise.AerobRun12Min;
          const score = evaluateMini(
            Gender.Female,  // gender
            20,             // age
            aerobExercise,  // aerobic exercise type
            1900,           // aerobic result (meters)
            2.4,            // long jump (meters)
            42,             // push-ups (count)
            42,             // sit-ups (count)
            42              // trunk lift (count)
          );

          // score is a pointer object to WASM, use toJSON to grab js object
          console.log(score.toJSON());

          // show in html
          const output = document.getElementById("result-output");
          output.textContent += `${JSON.stringify(score.toJSON(), null ,"  ")}\n`;
          
          // 1 aerob + 6 motor exercises
          const score2 = evaluate(
            Gender.Male,              // gender
            20,                       // age
            Exercise.AerobRun12Min,   // aerobic exercise type
            2600,                     // aerobic result (meters)
            2.4,                      // long jump (meters)
            40,                       // push-ups (count)
            70,                       // sit-ups (count)
            90,                       // trunk lift (count)
            1.4,                      // medicine ball throw double hand (meters)
            1.2                       // medicine ball throw single hand (meters)
          );
          console.log(score2.toJSON());
          output.textContent += `${JSON.stringify(score2.toJSON(), null ,"  ")}\n`;
          
          const score3 = evaluate(
            Gender.Male,              // gender
            25,                       // age
            Exercise.AerobRun12Min,   // aerobic exercise type
            2700,                     // aerobic result (meters)
            2.6,                      // long jump (meters)
            50,                       // push-ups (count)
            80,                       // sit-ups (count)
            100,                      // trunk lift (count)
            1.6,                      // medicine ball throw double hand (meters)
            1.4                       // medicine ball throw single hand (meters)
          );
          console.log(score3.toJSON());
          output.textContent += `${JSON.stringify(score3.toJSON(), null ,"  ")}\n`;
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
