# JavaScript Basics

## Course Goals

This material aims to help you:

1.  **Learn JavaScript:** Understand the fundamental concepts and syntax of JavaScript as a programming language.
2.  **Grasp Key Concepts:** Recognize its unique features, ways of working, and common programming patterns, assuming you have some background in other programming languages.
3.  **Focus on Modern JavaScript:** Learn features introduced up to **ES6 (ECMAScript 2015)** and potentially later versions.
4.  **Understand Execution Environments:** See how JavaScript runs in different places, including:
    -   **Server-side:** Using environments like **Node.js**.
    -   **Client-side:** Running directly within web browsers.
    -   Recognize how modern features can be used even in older environments through tools like **transpilers**.

## Content Outline

*   What is JavaScript?
*   History and Versions
*   Language Structure
*   Types and Variables
*   Expressions and Operators
*   Control Structures
*   Arrays
*   Strings

---

## What is JavaScript?

### Understanding JavaScript

*   **Core Definition:** **JavaScript** (often shortened to **JS**) is a high-level programming language primarily used to create interactive effects within web browsers and build web applications.
*   **Browser Native:** It holds the unique position of being the **only** programming language that web browsers can execute natively (without plugins).
*   **Beyond the Browser:** JavaScript isn't limited to web pages; it also runs on servers and computers using environments like **Node.js**.
*   **Name Origin:** Despite its name, JavaScript has **no direct technical relationship to the Java language**. The name was chosen largely for marketing reasons during the early days of the web when Java was popular.
*   **Rapid Development:** The initial version of JavaScript was famously developed in just 10 days. This rapid development, influenced by company politics at the time, led to some initial design quirks compared to languages developed over longer periods.

### Simple Example

Here's a basic JavaScript code snippet. When run in a web browser, it displays a simple pop-up alert box:

```javascript
// This code shows a pop-up message in the browser.
alert("Hello, JavaScript!");
```

---

## History and Versions

### JavaScript's Evolution

#### Official Standard: ECMAScript

*   The official standard specification for JavaScript is called **ECMAScript (ES)**. When people refer to versions like ES5 or ES6, they are talking about versions of this standard.
*   **Major Milestones:** Key releases include ES5 (2009) and especially **ES6 (ECMAScript 2015)**, which introduced many significant improvements and features that are central to modern JavaScript development. This course focuses heavily on ES6 and later features.
*   **Continuous Development:** JavaScript evolves with a **yearly release cycle**. The standard is managed by the **TC39 committee**, which includes members from major technology companies like Mozilla, Google, Facebook, Apple, and Microsoft, ensuring broad industry support and direction.

#### JavaScript Engines: The Power Behind Execution

Different environments use different "engines" to interpret and run JavaScript code. Key engines include:

<p align="center">

| Engine             | Developed By | Used In                                          |
| :----------------- | :----------- | :----------------------------------------------- |
| **V8**             | Google       | Chrome, Chromium, Node.js, Microsoft Edge      |
| **SpiderMonkey**   | Mozilla      | Firefox                                          |
| **JavaScriptCore** | Apple        | Safari                                           |
| *ChakraCore*       | *Microsoft*  | *Previously used in older versions of Edge*     |

</p>

#### Compatibility and Modern Features

Understanding compatibility is crucial:

1.  **Backward Compatibility:** JavaScript is generally **backward-compatible**. Code written using features from an older version *(e.g., ES5)* will almost always run correctly in newer JavaScript engines. Once a feature is officially added to the standard, it's expected to remain supported.
2.  **Forward Compatibility:** JavaScript is **not forward-compatible**. Code using new features (e.g., from ES2020) will cause errors if run in an older engine that doesn't understand those features.
3.  **Strict Mode (`"use strict";`):** Introduced in ES5, this is a special mode you can opt into. It makes JavaScript stricter by changing certain potentially problematic behaviors (like prohibiting the use of undeclared variables) and throwing more errors, leading to cleaner and more secure code.
4.  **Transpiling:** To use the latest JavaScript features while still supporting older browsers or environments, developers use tools called **transpilers** (like **Babel**). These tools convert modern JavaScript code into an equivalent older version *(e.g., ES5)* that has wider compatibility.
5.  **Polyfills:** These are pieces of code that provide implementations for newer features that might be missing in older environments. If a browser doesn't natively support a specific function, a polyfill can add a custom version of that function.

#### Where JavaScript Runs

Common environments for running JavaScript:

*   **Web Browsers:** Google Chrome, Mozilla Firefox, Microsoft Edge, Apple Safari, etc.
*   **Server-Side / Command Line:** **Node.js** allows running JavaScript outside the browser on various operating systems (Windows, Linux, macOS, including WSL2 on Windows).
*   **Developer Tools:** Online tools like JavaScriptTutor (for visualizing execution) and browser developer consoles (`jsconsole`) are invaluable for learning and debugging.

#### Example: Enabling Strict Mode

To enable strict mode, add the following string literal at the very beginning of a JavaScript file or `<script>` tag:

```javascript
"use strict"; // Enables strict mode for the following code

let x = 10; // Declaring variables with let/const/var is required in strict mode.

// Attempting to use an undeclared variable will cause an error in strict mode.
// y = 20; // This line would cause: ReferenceError: y is not defined
```

---

## Language Structure

### How JavaScript Code is Organized

#### Lexical Structure (The Basic Rules)

*   **Files and Programs:** Generally, each `.js` file is treated as a separate JavaScript program. Different files typically interact through shared global state (in older styles or simple scripts) or more commonly via **module systems** (import/export, discussed later).
*   **Built-in Features:** JavaScript relies on its execution environment (like a browser or Node.js) to provide a **standard library** of built-in functions and **Application Programming Interfaces (APIs)** (e.g., `console.log`, DOM manipulation APIs in browsers, file system APIs in Node.js).
*   **Character Set:** JavaScript code uses the **Unicode** standard, allowing characters from various languages, symbols, and even emojis in comments, strings, and (though often discouraged for clarity) variable names.
*   **Semicolons (;):** While JavaScript has rules for **Automatic Semicolon Insertion (ASI)** where the engine tries to guess where semicolons should be, relying on this can lead to subtle and unexpected errors.
*   **Case Sensitivity:** JavaScript is **case-sensitive**. `myVariable` and `myvariable` are treated as two different variables.
*   **Comments:** Uses C-style syntax for comments:
    *   `// Single-line comment`
    *   `/* Multi-line comment */`

### Semicolon Best Practice

*   Because **Automatic Semicolon Insertion (ASI)** can sometimes insert semicolons in surprising places or fail to insert them where needed, leading to bugs, the widely **recommended practice** is to **always explicitly write semicolons** at the end of your statements.

### Example: Case Sensitivity and Comments

```javascript
// This is a single-line comment.

/*
 This comment spans
 multiple lines.
*/

let studentName = "Alice"; // Variable named studentName

// let studentname = "Bob"; // This would declare a *different* variable because of case sensitivity.

console.log(studentName); // Output: Alice
// console.log(studentname); // If the line above were uncommented, this would output: Bob
```

---

## Types and Variables

### Handling Data in JavaScript

#### Primitive Data Types

JavaScript has several fundamental (primitive) data types:

*   **`string`:** Represents textual data. Defined using single quotes (`'abc'`), double quotes (`"abc"`), or backticks (``` `abc` ``` - template literals).
*   **`number`:** Represents both integers (`42`) and floating-point numbers (`3.14`). Includes special values like `Infinity`, `-Infinity`, and `NaN` (Not-a-Number).
*   **`boolean`:** Represents logical values: `true` or `false`.
*   **`null`:** Intentionally represents the absence of any object value. It's treated as a distinct value.
*   **`undefined`:** Represents a variable that has been declared but not yet assigned a value, or a function argument that wasn't provided.
*   **`bigint`:** Represents integers of arbitrary size, exceeding the limits of the standard `number` type. Created by appending `n` to an integer literal (e.g., `12345678901234567890n`).
*   **`symbol`:** *(Less common in introductory material)* Represents unique identifiers.

#### Declaring Variables

Modern JavaScript primarily uses `let` and `const` for variable declarations:

<p align="center">

| Keyword     | Scope          | Mutability (Reassignment)       | Hoisting Behavior                     | Modern Usage              |
| :---------- | :------------- | :------------------------------ | :------------------------------------ | :------------------------ |
| **`let`**   | **Block (`{}`)** | **Mutable** (Can reassign)      | **Not hoisted (TDZ\*)**                 | **Preferred** (Mutable)   |
| **`const`** | **Block (`{}`)** | **Immutable** (Cannot reassign\*\*) | **Not hoisted (TDZ\*)**                 | **Preferred** (Immutable) |
| **`var`**   | **Function**   | Mutable                         | **Hoisted (Initialized `undefined`)** | **Avoid (Legacy)**        |

</p>

*\* TDZ (Temporal Dead Zone): Variables declared with `let` and `const` exist from the start of their block but cannot be accessed until the line where they are declared is executed.*
*\*\* `const` prevents reassignment of the variable itself. If the variable holds an object or array, the *contents* of that object/array can still be modified.*

#### Scope and Hoisting Explained

*   **Scope:** Determines where a variable is accessible.
    *   **Block Scope (`let`, `const`):** Variables are only accessible within the **Block (`{}`)** where they are defined.
    *   **Function Scope (`var`):** Variables are accessible anywhere within the **Function** where they are defined, regardless of blocks.
*   **Hoisting (`var`):** Before code execution, JavaScript conceptually moves `var` *declarations* (but not their *assignments*) to the top of their containing function. This means you can reference a `var` variable earlier in the function than its declaration, and it will have the value `undefined` until its assignment line is reached.
*   **`let` and `const` Hoisting:** While `let` and `const` declarations are also notionally "hoisted" to the top of their block, they are not initialized. Accessing them before their declaration line results in a `ReferenceError` due to the **Temporal Dead Zone (TDZ)**.

#### Example: Primitive Types and Variable Declarations

```javascript
// String examples
let str1 = 'Single quotes';
let str2 = "Double quotes";
let str3 = `Template literal with backticks`;

// Number examples
let integerNum = 42;
let floatNum = 3.14;

// Boolean example
let isActive = true;

// Null and Undefined
let noValue = null;       // Intentionally no value
let notAssigned;          // Automatically undefined

// BigInt example
let veryLargeNumber = 90071992547409919999n; // Ends with 'n'

// --- Variable Declarations ---

// 'let' allows reassignment
let counter = 10;
counter = 20; // OK

// 'const' requires initialization and prevents reassignment
const apiKey = "xyz123abc";
// apiKey = "newKey"; // TypeError: Assignment to constant variable.

// 'var' (legacy - avoid)
var oldStyleVar = 50;
oldStyleVar = 60; // OK
```

#### Example: Demonstrating Scope (`let`, `const` vs `var`)

```javascript
function testScope() {
  let functionLet = "Available throughout function";

  if (true) {
    let blockScopedLet = "Inside block only (let)";
    const blockScopedConst = "Inside block only (const)";
    var functionScopedVar = "Inside function (var)";

    console.log(blockScopedLet);   // Output: Inside block only (let)
    console.log(blockScopedConst); // Output: Inside block only (const)
    console.log(functionScopedVar);// Output: Inside function (var)
  }

  // Trying to access block-scoped variables outside their block:
  // console.log(blockScopedLet);   // ReferenceError: blockScopedLet is not defined
  // console.log(blockScopedConst); // ReferenceError: blockScopedConst is not defined

  // Function-scoped 'var' is still accessible here:
  console.log(functionScopedVar);  // Output: Inside function (var)
  console.log(functionLet);        // Output: Available throughout function
}

testScope();

// Trying to access function-scoped variables outside the function:
// console.log(functionScopedVar); // ReferenceError: functionScopedVar is not defined
// console.log(functionLet);       // ReferenceError: functionLet is not defined
```

#### Example: Demonstrating Hoisting (`var`)

```javascript
// Example 1: Accessing 'var' before assignment
console.log(myVar); // Output: undefined (Declaration was hoisted, but assignment wasn't)
var myVar = "I am hoisted!";
console.log(myVar); // Output: I am hoisted!

// How JavaScript interprets the above due to hoisting:
/*
 var myVar; // Declaration moved to the top conceptually
 console.log(myVar); // Reads 'undefined'
 myVar = "I am hoisted!"; // Assignment happens here
 console.log(myVar); // Reads the assigned value
*/

// Example 2: Accessing 'let' before declaration (causes error)
// console.log(myLet); // ReferenceError: Cannot access 'myLet' before initialization (TDZ)
let myLet = "I am not hoisted like var";
console.log(myLet);
```

---

## Expressions and Operators

### Performing Operations in JavaScript

#### Common Operators

JavaScript provides various operators to manipulate values:

*   **Arithmetic:** `+` (addition), `-` (subtraction), `*` (multiplication), `/` (division), `%` (modulo/remainder), `**` (exponentiation - ES2016).
*   **Comparison:**
    *   **`==`** (Loose Equality: checks value, performs type conversion)
    *   **`!=`** (Loose Inequality: checks value, performs type conversion)
    *   **`===`** (Strict Equality: checks value AND type, no conversion) - **Generally Preferred**
    *   **`!==`** (Strict Inequality: checks value AND type, no conversion) - **Generally Preferred**
    *   `>` (greater than), `<` (less than), `>=` (greater than or equal), `<=` (less than or equal)
*   **Logical:** `&&` (logical AND), `||` (logical OR), `!` (logical NOT).
*   **Assignment:** `=` (assign), `+=` (add and assign), `-=` (subtract and assign), `*=` (multiply and assign), `/=` (divide and assign), etc.
*   **Bitwise:** `&` (AND), `|` (OR), `^` (XOR), `~` (NOT), `<<` (left shift), `>>` (right shift). *(Less common in typical web development)*.
*   **Ternary (Conditional):** `condition ? expressionIfTrue : expressionIfFalse`. A concise way to write simple `if-else` expressions.

#### Automatic Type Conversion (Coercion)

*   JavaScript often tries to automatically convert values between types when operators expect a certain type (e.g., in `5 == '5'`). This is called **implicit type coercion**.
*   **Loose Equality (`==`):** Performs type coercion before comparing values. This can lead to unexpected results (`0 == false` is true, `null == undefined` is true).
*   **Strict Equality (`===`):** Does **not** perform type coercion. It checks if both the value and the type are identical. This is generally safer and easier to reason about, making it the **preferred comparison operator**.
*   **Explicit Conversion:** You can manually convert types using built-in functions:
    *   `Number(value)`: Converts `value` to a number (or `NaN`).
    *   `String(value)`: Converts `value` to a string.
    *   `Boolean(value)`: Converts `value` to a boolean (most values convert to `true`, except `0`, `""`, `null`, `undefined`, `NaN`, and `false`).

#### Example: Arithmetic, Comparison, and Logical Operators

```javascript
let a = 10;
let b = 5;
let c = '10';

// Arithmetic
console.log("a + b:", a + b); // Output: 15
console.log("a * b:", a * b); // Output: 50

// Comparison (Loose vs. Strict)
console.log("a == c (Loose):", a == c);   // Output: true (string '10' coerced to number 10)
console.log("a === c (Strict):", a === c);  // Output: false (number 10 is not same type as string '10')
console.log("a != b (Loose):", a != b);     // Output: true
console.log("a !== c (Strict):", a !== c); // Output: true (different types)

// Logical
let isAvailable = true;
let hasPermission = false;
console.log("Available AND Permission:", isAvailable && hasPermission); // Output: false
console.log("Available OR Permission:", isAvailable || hasPermission);  // Output: true
console.log("NOT Available:", !isAvailable);                      // Output: false
```

#### Example: Assignment and Ternary Operators

```javascript
let score = 100;
score += 50; // score becomes 150
console.log("New score:", score); // Output: 150

let temperature = 15;
let weather = temperature > 20 ? "Warm" : "Cool";
console.log("Weather:", weather); // Output: Cool
```

#### Example: Automatic and Explicit Type Conversion

```javascript
// Automatic Coercion Examples
console.log("5 == '5':", 5 == '5');         // Output: true
console.log("1 == true:", 1 == true);       // Output: true
console.log("0 == false:", 0 == false);     // Output: true
console.log("null == undefined:", null == undefined); // Output: true ('==')
console.log("null === undefined:", null === undefined); // Output: false ('===')

// Explicit Conversion Examples
let yearString = "2024";
let yearNumber = Number(yearString);
console.log(`Number("2024"): ${yearNumber} (type: ${typeof yearNumber})`); // Output: 2024 (type: number)

let pointsNumber = 99;
let pointsString = String(pointsNumber);
console.log(`String(99): ${pointsString} (type: ${typeof pointsString})`); // Output: 99 (type: string)

let isEmpty = "";
let isEmptyBoolean = Boolean(isEmpty);
console.log(`Boolean(""): ${isEmptyBoolean} (type: ${typeof isEmptyBoolean})`); // Output: false (type: boolean)
```

---

## Control Structures

### Directing the Flow of Execution

#### Conditional Statements

These structures execute different code blocks based on whether a condition is true or false.

*   **`if` / `else if` / `else`:** Standard conditional logic. Checks conditions sequentially.
*   **`switch`:** Efficiently compares a single expression against multiple possible constant values (`case` labels). Often used as an alternative to long `if-else if` chains. Remember to use `break` to exit a case, otherwise execution "falls through" to the next case.

**Example: `if`, `else if`, `else`**

```javascript
let userRole = "editor";

if (userRole === "admin") {
  console.log("Full access granted.");
} else if (userRole === "editor") {
  console.log("Content editing access granted.");
} else {
  console.log("View-only access.");
}
// Output: Content editing access granted.
```

**Example: `switch` Statement**

```javascript
let statusCode = 200;
let message;

switch (statusCode) {
  case 200:
    message = "OK";
    break; // Exit the switch
  case 404:
    message = "Not Found";
    break;
  case 500:
    message = "Internal Server Error";
    break;
  default: // Optional: handles values not matching any case
    message = "Unknown Status Code";
}
console.log(`Status ${statusCode}: ${message}`); // Output: Status 200: OK
```

#### Looping Constructs

Loops repeat a block of code multiple times.

*   **`for` loop:** The classic loop. Initializes a counter, checks a condition before each iteration, and updates the counter after each iteration. `for (let i = 0; i < limit; i++) { ... }`
*   **`while` loop:** Executes a block of code as long as a condition remains true. The condition is checked *before* each iteration. `while (condition) { ... }`
*   **`do...while` loop:** Similar to `while`, but the condition is checked *after* the block executes. This guarantees the block runs at least once. `do { ... } while (condition);`
*   **`for...in` loop:** Iterates over the **property names** (keys) of an object. **Note:** Not generally recommended for iterating over arrays, as it can include non-index properties and order is not guaranteed.
*   **`for...of` loop:** *(ES6+)* Iterates over the **values** of an **iterable object** (like Arrays, Strings, Maps, Sets). This is the **preferred way to loop over array elements**.

**Example: `for` Loop**

```javascript
console.log("Counting up with 'for':");
for (let i = 1; i <= 3; i++) {
  console.log(i);
}
// Output:
// Counting up with 'for':
// 1
// 2
// 3
```

**Example: `while` Loop**

```javascript
let ticketsLeft = 3;
console.log("Selling tickets with 'while':");
while (ticketsLeft > 0) {
  console.log(`Ticket sold! ${ticketsLeft - 1} remaining.`);
  ticketsLeft--;
}
console.log("All tickets sold.");
// Output:
// Selling tickets with 'while':
// Ticket sold! 2 remaining.
// Ticket sold! 1 remaining.
// Ticket sold! 0 remaining.
// All tickets sold.
```

**Example: `do...while` Loop**

```javascript
let input;
// This loop ensures the body runs at least once before checking condition
// Uncomment the following lines to run in a browser/Node console
/*
do {
  input = prompt("Enter 'exit' to quit:"); // Example using prompt
  console.log(`You entered: ${input}`);
} while (input !== "exit");
console.log("Exited loop.");
*/
console.log("Do...while example structure complete.");
```

**Example: `for...in` Loop (Iterating Object Properties)**

```javascript
const car = {
  make: 'Toyota',
  model: 'Camry',
  year: 2022
};
console.log("Car properties (for...in):");
for (let propertyKey in car) {
  // key will be 'make', 'model', 'year'
  console.log(`${propertyKey}: ${car[propertyKey]}`);
}
// Output:
// Car properties (for...in):
// make: Toyota
// model: Camry
// year: 2022
```

**Example: `for...of` Loop (Iterating Array Values - Preferred)**

```javascript
const browsers = ['Chrome', 'Firefox', 'Edge'];
console.log("Browsers (for...of):");
for (let browser of browsers) {
  // browser will be 'Chrome', 'Firefox', 'Edge'
  console.log(browser);
}
// Output:
// Browsers (for...of):
// Chrome
// Firefox
// Edge
```

#### Exception Handling

Allows managing errors that occur during runtime.

*   **`try { ... }`:** Encloses code that might potentially throw an error.
*   **`catch (error) { ... }`:** Catches any error thrown in the `try` block. The *`error`* variable holds information about the error.
*   **`throw ...;`:** Manually creates and raises (throws) an error. Often used with `new Error("message")`.
*   **`finally { ... }`:** Contains code that will execute *regardless* of whether an error was thrown or caught in the `try`/`catch` blocks. Useful for cleanup operations (e.g., closing files, releasing resources).

**Example: `try...catch...finally`**

```javascript
function divide(a, b) {
  if (b === 0) {
    throw new Error("Division by zero is not allowed.");
  }
  return a / b;
}

try {
  console.log("Attempting division...");
  let result = divide(10, 0); // This will throw an error
  console.log("Result:", result); // This line won't execute
} catch (error) {
  console.error("An error occurred:", error.message); // Catch the thrown error
} finally {
  console.log("Division attempt finished."); // This always executes
}
// Output:
// Attempting division...
// An error occurred: Division by zero is not allowed.
// Division attempt finished.
```

---

## Arrays

### Working with Ordered Collections

*   **Definition:** **Arrays** in JavaScript are ordered collections of values. They can hold elements of **mixed data types** (numbers, strings, booleans, objects, other arrays, etc.).
*   **Syntax:** Arrays are created using square brackets `[...]`, with elements separated by commas: `let myArr = [1, 'hello', true];`.

### Common Array Methods

Arrays come with many built-in methods. It's helpful to categorize them:

**1. Methods that Modify the Original Array:**

<p align="center">

| Method                          | Description                                                | Example                            | Resulting Array          |
| :------------------------------ | :--------------------------------------------------------- | :--------------------------------- | :----------------------- |
| `.push(el)`                     | Adds element *`el`* to the **end**                         | `[1, 2].push(3)`                   | `[1, 2, 3]`              |
| `.pop()`                        | Removes and returns the element from the **end**           | `[1, 2, 3].pop()`                  | `[1, 2]` (returns `3`)   |
| `.shift()`                      | Removes and returns the element from the **beginning**     | `[1, 2, 3].shift()`                | `[2, 3]` (returns `1`)   |
| `.unshift(el)`                  | Adds element *`el`* to the **beginning**                   | `[2, 3].unshift(1)`                | `[1, 2, 3]`              |
| `.splice(idx, count, ...items)` | Removes *`count`* elements starting at *`idx`*, optionally inserting *`items`* | `[1, 2, 5].splice(1, 1, 3, 4)` | `[1, 3, 4, 5]` (returns `[2]`) |
| `.sort()`                       | Sorts the array in place (default: lexicographically)      | `[3, 1, 2].sort()`                 | `[1, 2, 3]`              |
| `.reverse()`                    | Reverses the array in place                                | `[1, 2, 3].reverse()`              | `[3, 2, 1]`              |

</p>

**2. Methods that Return a New Array (Do Not Modify Original):**

<p align="center">

| Method                 | Description                                                         | Example                         | Returned Array |
| :--------------------- | :------------------------------------------------------------------ | :------------------------------ | :------------- |
| `.map(fn)`             | Creates a new array by applying function *`fn`* to each element       | `[1, 2].map(x => x*2)`        | `[2, 4]`       |
| `.filter(fn)`          | Creates a new array with elements passing the test in *`fn`*           | `[1, 2, 3].filter(x => x>1)`    | `[2, 3]`       |
| `.concat(arr2)`        | Returns a new array merging the original with *`arr2`*                | `[1].concat([2, 3])`          | `[1, 2, 3]`    |
| `.slice(start, end?)` | Returns a shallow copy of a portion of the array (*`end?`* index is exclusive) | `[1, 2, 3, 4].slice(1, 3)`    | `[2, 3]`       |

</p>

**3. Methods for Iteration and Reduction:**

<p align="center">

| Method                   | Description                                                               | Example                                    | Return Value Example     |
| :----------------------- | :------------------------------------------------------------------------ | :----------------------------------------- | :----------------------- |
| `.forEach(fn)`           | Executes function *`fn`* for each element                               | `[1, 2].forEach(x => console.log(x))`    | `undefined` (logs 1, 2) |
| `.reduce(fn, initial?)`  | Applies function *`fn`* against an accumulator and each element to reduce it to a single value | `[1, 2, 3].reduce((sum, x) => sum + x, 0)` | `6`                      |
| `.find(fn)`              | Returns the **value** of the first element passing the test in *`fn`*     | `[1, 2, 3].find(x => x > 1)`             | `2`                      |
| `.findIndex(fn)`         | Returns the **index** of the first element passing the test in *`fn`*     | `[1, 2, 3].findIndex(x => x > 1)`        | `1`                      |
| `.includes(val)`         | Returns `true` if the array contains *`val`*, else `false`                  | `[1, 2, 3].includes(2)`                | `true`                   |

</p>

### Array Destructuring

A convenient syntax to unpack values from arrays into distinct variables.

```javascript
let coordinates = [100, 200, 50];
let [xCoord, yCoord] = coordinates; // Assigns first two elements

console.log("x:", xCoord); // Output: x: 100
console.log("y:", yCoord); // Output: y: 200
// The third element (50) is ignored in this destructuring pattern.
```

### Spread Operator (`...`)

This operator expands an iterable (like an array) into its individual elements. Useful for:

*   Combining arrays.
*   Copying arrays.
*   Passing array elements as individual arguments to functions.

```javascript
// Combining arrays
let firstHalf = [1, 2];
let secondHalf = [3, 4];
let combined = [...firstHalf, 0, ...secondHalf]; // Expands arrays into elements
console.log("Combined with Spread:", combined); // Output: [ 1, 2, 0, 3, 4 ]

// Copying an array (shallow copy)
let original = ['a', 'b'];
let copy = [...original];
console.log("Copied array:", copy); // Output: [ 'a', 'b' ]
copy.push('c');
console.log("Original after copy modified:", original); // Output: [ 'a', 'b' ] (original is unchanged)

// Passing elements as function arguments
function add(a, b, c) {
  return a + b + c;
}
let numbersToAdd = [5, 10, 15];
let sum = add(...numbersToAdd); // Passes 5, 10, 15 as individual arguments
console.log("Sum using spread args:", sum); // Output: 30
```

### Example: Demonstrating Various Array Methods

```javascript
let sampleNumbers = [10, 20, 30, 40, 50];

// Modify original: push, splice
sampleNumbers.push(60);
console.log("After push(60):", sampleNumbers); // Output: [ 10, 20, 30, 40, 50, 60 ]
sampleNumbers.splice(1, 2); // Remove 2 elements starting at index 1 (removes 20, 30)
console.log("After splice(1, 2):", sampleNumbers); // Output: [ 10, 40, 50, 60 ]

// Create new array: map, filter
let squares = sampleNumbers.map(num => num * num);
console.log("Mapped to squares:", squares); // Output: [ 100, 1600, 2500, 3600 ]

let highNumbers = sampleNumbers.filter(num => num > 40);
console.log("Filtered (> 40):", highNumbers); // Output: [ 50, 60 ]

// Reduce to single value
let totalSum = sampleNumbers.reduce((acc, current) => acc + current, 0);
console.log("Reduced sum:", totalSum); // Output: 160 (10 + 40 + 50 + 60)
```

---

## Strings

### Working with Textual Data

*   **Definition:** **Strings** in JavaScript represent sequences of Unicode characters.
*   **Immutability:** Strings are **immutable**. This means that once a string is created, its contents cannot be changed directly. Methods that appear to modify a string actually return a **new** string with the changes.

### Basic String Operations

*   **Concatenation:** Combining strings using the `+` operator.
*   **Indexing:** Accessing individual characters using square bracket notation `string[index]`. Note that this accesses the character at that position, but you cannot *change* it via index (due to immutability).
*   **Length:** Getting the number of characters using the `.length` property.

**Example: Basic Operations**

```javascript
let firstName = "Java";
let lastName = "Script";

// Concatenation
let fullName = firstName + lastName;
console.log("Concatenated:", fullName); // Output: JavaScript

// Indexing (Read-only)
let firstInitial = firstName[0];
console.log("First initial:", firstInitial); // Output: J
// firstName[0] = 'L'; // This would cause an error in strict mode, or fail silently. Strings are immutable.

// Length
console.log("Length of fullName:", fullName.length); // Output: 10
```

### Common String Methods

JavaScript provides many methods for string manipulation (remember, they return new strings):

*   **Case Conversion:** `.toUpperCase()`, `.toLowerCase()`.
*   **Whitespace Removal:** `.trim()` (removes from both ends), `.trimStart()`, `.trimEnd()`.
*   **Extracting Substrings:** `.slice(startIndex, endIndex?)`, `.substring(startIndex, endIndex?)`.
*   **Splitting:** `.split(separator)` (splits string into an array of substrings based on *`separator`*).
*   **Replacing:** `.replace(searchValue, newValue)` (replaces the *first* occurrence of *`searchValue`*), `.replaceAll(searchValue, newValue)` (replaces *all* occurrences).
*   **Checking Content:** `.includes(substring)`, `.startsWith(substring)`, `.endsWith(substring)`.

**Example: Various String Methods**

```javascript
let messyText = "   Hello World!   ";

console.log("Original:", `'${messyText}'`);
console.log("Trimmed:", `'${messyText.trim()}'`); // Output: 'Hello World!'
console.log("Uppercase:", messyText.toUpperCase()); // Output:    HELLO WORLD!
console.log("Lowercase:", messyText.toLowerCase()); // Output:    hello world!

let sentence = "The quick brown fox";
console.log("Slice(4, 9):", sentence.slice(4, 9)); // Output: quick (index 9 is exclusive)
console.log("Split by space:", sentence.split(" ")); // Output: [ 'The', 'quick', 'brown', 'fox' ]
console.log("Replace 'brown':", sentence.replace("brown", "red")); // Output: The quick red fox

console.log("Includes 'quick':", sentence.includes("quick")); // Output: true
console.log("Starts with 'The':", sentence.startsWith("The")); // Output: true
console.log("Ends with 'dog':", sentence.endsWith("dog"));   // Output: false
```

### Template Literals (Template Strings)

*(Introduced in ES6)* A more powerful way to create strings, especially useful for embedding variables or expressions directly within the string.

*   **Syntax:** Uses backticks (`` ` ``) instead of single or double quotes.
*   **Embedded Expressions:** Allows embedding expressions (like variables or calculations) inside `${...}` placeholders.

**Example: Template Literals**

```javascript
let item = "Laptop";
let price = 1200;
let taxRate = 0.07;

// Using template literal for easy embedding
let message = `The ${item} costs $${price}. Tax is $${(price * taxRate).toFixed(2)}.`;
console.log(message);
// Output: The Laptop costs $1200. Tax is $84.00.

// Multi-line strings are also easier with backticks
let multiLine = `This is line one.
This is line two.`;
console.log(multiLine);
// Output:
// This is line one.
// This is line two.
```