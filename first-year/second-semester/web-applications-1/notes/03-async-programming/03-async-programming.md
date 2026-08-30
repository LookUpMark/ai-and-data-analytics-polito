# Asynchronous Programming in JavaScript

## Outline

This section covers fundamental concepts in asynchronous programming within JavaScript, specifically:

*   **Callbacks** (both synchronous and asynchronous applications)
*   **Functional Programming** concepts (with focus on array methods)
*   The necessity and principles of **Asynchronous Programming**
*   Practical examples using Database Access (specifically with **SQLite** and **Node.js**)
*   Introduction to **Promises** as a solution to manage asynchronous flow and the problem of **Callback Hell**
*   Simplifying asynchronous code further with **Async/Await** syntax

---

## CALLBACKS

### Understanding Callbacks

A fundamental concept in JavaScript is the **callback function**. This is a function that is passed as an argument to another function, with the intention that the outer function will invoke the callback at a later point. Callbacks are used to specify "what happens next" after an operation completes. They can be categorized into two main types:

*   **Synchronous Callbacks:** The outer function calls the callback function immediately, before the outer function itself finishes execution.
*   **Asynchronous Callbacks:** The outer function schedules the callback for execution at a later time, specifically after a potentially time-consuming operation (such as I/O, a timer, or an event) has completed and the JavaScript engine's call stack is clear.

**Example: Simple Synchronous Callback**

```javascript
// This is the callback function
function logQuote(quoteString) {
  console.log(quoteString);
}

// This function accepts a callback as an argument
function createQuote(originalQuote, callbackFunction) {
  const formattedQuote = `Like I always say, '${originalQuote}'`;
  callbackFunction(formattedQuote); // The callback is called immediately (synchronously)
}

// Call createQuote, passing logQuote as the callback
createQuote("WebApp I rocks!", logQuote); // Output: Like I always say, 'WebApp I rocks!'
```

---

## Synchronous Callbacks in Use

Synchronous callbacks are frequently used to allow calling code to customize the behavior of a standard method. This pattern is particularly common in **Functional Programming** paradigms and is seen in many built-in JavaScript array methods.

### Example: Sorting with a Callback (`Array.prototype.sort()`)

The `Array.prototype.sort()` method is a classic example using a synchronous callback. It takes an optional compare function as its argument. This compare function is called synchronously by `sort()` for pairs of elements (`a`, `b`) to determine their relative order. The return value dictates the sort order: a negative value means `a` comes before `b`, a positive value means `a` comes after `b`, and zero means their order is unchanged relative to each other.

```javascript
let numbers = [4, 2, 5, 1, 3];
// The callback (a, b) => a - b is called synchronously by sort()
numbers.sort((a, b) => a - b); // Sorts numbers in ascending order
console.log(numbers); // Output: [ 1, 2, 3, 4, 5 ]
```

---

## Synchronous Callbacks with `filter()`

Similarly, the `Array.prototype.filter()` method uses a synchronous callback. The callback function is executed once for each element in the array. If the callback returns a truthy value, the element is included in a **new array**. The `filter()` method returns this new array. Importantly, the original array remains unmodified.

```javascript
const market = [ { name: 'GOOG', var: -3.2 }, { name: 'AMZN', var:  2.2 }, { name: 'MSFT', var: -1.1 } ];
// The callback stock => stock.var < 0 is called synchronously for each stock object
const bad = market.filter(stock => stock.var < 0); // Creates a new array with only 'bad' stocks
console.log("Bad stocks:", bad); // Output: [ { name: 'GOOG', var: -3.2 }, { name: 'MSFT', var: -1.1 } ]
```

---

## FUNCTIONAL PROGRAMMING

### Applying Functional Concepts in JavaScript

**Functional Programming (FP)** is a programming paradigm that treats computation as the evaluation of mathematical functions and avoids changing state or mutable data. It promotes a more declarative style ("what" needs to be done) over an imperative one ("how" to do it). Adopting FP principles can lead to code that is often more readable, predictable (due to minimized side effects), and easier to test.

JavaScript supports FP concepts well due to:

*   **First-class functions:** Functions can be treated as values (assigned to variables, passed as arguments, returned from other functions).
*   **Higher-order functions:** Functions can accept other functions as arguments or return functions as results.
*   **Function composition:** Building complex operations by chaining simpler functions together.
*   **Call chaining:** Many methods (especially array methods) return objects (often new arrays) that allow chaining subsequent method calls (`array.filter().map().reduce()`).

---

## Iterating Over Arrays (Functional Methods)

JavaScript provides several built-in array methods that leverage synchronous callbacks to iterate over elements and perform operations. These methods often return new arrays or a single accumulated result, fitting well with FP's emphasis on avoiding mutation of original data.

Common **Functional Array Methods** utilizing callbacks include: `forEach()`, `every()`, `some()`, `map()`, `filter()`, and `reduce()`.

The callback function used with `reduce` has a specific signature: `(accumulator, currentValue[, index[, array]])`.

---

## `.forEach()` Method

The `forEach()` method executes a provided callback function once for each element in the array, in ascending index order. Its primary purpose is to perform **side effects** (e.g., printing to console, modifying external state) rather than transforming the array or computing a value. It always returns `undefined`. Since it doesn't return the array itself, it is not chainable with methods that typically operate on arrays (like `map` or `filter`).

```javascript
const letters = [..."Hi"]; // letters is ['H', 'i']
let uppercase = "";
letters.forEach(letter => { // Callback runs for 'H', then 'i'
  uppercase += letter.toUpperCase(); // Side effect: appends to uppercase string
});
console.log("Resulting uppercase string:", uppercase); // Output: HI
```

---

## `.every()` Method

The `every()` method tests whether **all** elements in the array pass the condition implemented by the provided callback function. The callback is executed once for each element. `every()` returns `true` if the callback returns a truthy value for *every* element. If the callback returns a falsy value for *any* element, `every()` immediately stops iteration and returns `false`.

```javascript
let a = [1, 3, 5];
// Callback checks if a number is odd
console.log("Are all odd?", a.every(x => x % 2 !== 0)); // Output: Are all odd? true

let b = [1, 2, 3];
console.log("Are all odd?", b.every(x => x % 2 !== 0)); // Output: Are all odd? false (stops at 2)
```

---

## `.some()` Method

The `some()` method tests whether **at least one** element in the array passes the condition implemented by the provided callback function. The callback is executed once for each element. `some()` returns `true` if the callback returns a truthy value for *any* element, and iteration stops immediately. If the callback returns a falsy value for all elements, `some()` returns `false`.

```javascript
let a = [1, 3, 5];
// Callback checks if a number is even
console.log("Is at least one even?", a.some(x => x % 2 === 0)); // Output: Is at least one even? false

let b = [1, 2, 3];
console.log("Is at least one even?", b.some(x => x % 2 === 0)); // Output: Is at least one even? true (stops at 2)
```

---

## `.map()` Method

The `map()` method creates a **new array** populated with the results of calling a provided callback function on every element in the calling array. The callback is executed once for each element, and its return value is placed into the new array at the corresponding position. `map()` always returns this new array and does not modify the original array.

```javascript
const a = [1, 2, 3];
// Callback transforms each element by multiplying by 2
const b = a.map(x => x * 2); // Creates new array b by doubling each element of a
console.log("Original array (a):", a); // Output: Original array (a): [ 1, 2, 3 ]
console.log("Mapped array (b):", b); // Output: Mapped array (b): [ 2, 4, 6 ]
```

---

## `.filter()` Method

As previously shown, the `filter()` method creates a **new array** containing only those elements from the calling array for which the provided callback function returns a truthy value. The callback is executed once for each element. `filter()` returns this new array and does not modify the original array.

```javascript
const a = [1, 2, 3, 4];
// Callback checks if an element is greater than 2
console.log("Elements > 2:", a.filter(x => x > 2)); // Creates a new array filtering elements
// Output: Elements > 2: [ 3, 4 ]
```

---

## `.reduce()` Method

The `reduce()` method executes a user-provided reducer callback function on each element of the array, in order, passing in the return value from the calculation on the preceding element. This process results in a single output value (which can be a number, an object, an array, etc.). `reduce()` returns this final accumulated value.

The reducer callback signature is `(accumulator, currentValue[, index[, array]])`.

*   `accumulator`: The value resulting from the previous callback invocation, or the `initialValue` if one was provided on the first call.
*   `currentValue`: The current element being processed in the array.

```javascript
const a = [1, 2, 3];

// Example: Summing array elements
// Here, the accumulator starts at 0 (initialValue) and adds each current element.
let sum = a.reduce((acc, cur) => acc + cur, 0); // 0 is the initialValue for the accumulator
console.log("Sum:", sum); // Output: Sum: 6

// Example: Finding the maximum value (without initialValue)
// If no initialValue is provided, the accumulator starts as the first element (1),
// and the iteration begins from the second element.
let max = a.reduce((acc, cur) => (acc > cur) ? acc : cur);
console.log("Max:", max); // Output: Max: 3
```

---

## ASYNCHRONOUS PROGRAMMING

### Handling Operations That Take Time

In a synchronous application, operations that take a significant amount of time to complete (such as fetching data over a network, reading large files from disk, querying a database, or performing complex calculations) would cause the application to freeze or become unresponsive – it would "block" execution of subsequent code until the long operation finishes. **Asynchronous programming** is essential to handle such operations efficiently without blocking. It allows you to initiate a potentially long-running task and continue executing other code immediately. The results or errors from the asynchronous task are then handled later, using mechanisms like asynchronous callbacks or Promises, when the task completes. This non-blocking behavior is facilitated by the JavaScript **Event Loop**, which manages the execution stack and handles tasks that are offloaded to the system (like I/O timers), executing their associated callbacks only when the main call stack is empty.

**Example: `setTimeout` (Basic Async Function)**

```javascript
const greet = (name) => { console.log(`Hello, ${name}!`); }
console.log("Scheduling...");
// setTimeout is an asynchronous function: it schedules greet to run AFTER 1000ms delay
setTimeout(greet, 1000, "Alice"); // Arguments after delay are passed to callback
console.log("Scheduled. Continuing..."); // This line executes immediately, not waiting for the timer
// Expected output in console:
// Scheduling...
// Scheduled. Continuing...
// (1 second delay)
// Hello, Alice!
```

---

## Characteristics of Asynchronous Operations

Asynchronous operations are characterized by:

*   **Non-Blocking User Interface (UI):** Crucial in environments with a UI (like web browsers or desktop apps) to keep the interface responsive to user input while background tasks run.
*   **Efficient Input/Output (I/O) Handling:** Necessary for interacting with external resources such as network requests, file system access, or database queries, preventing the main program thread from waiting idly.
*   **Event-Driven Nature:** Often tied to reacting to events, where code is executed only when a specific event (like a button click, a timer expiring, or data arriving) occurs.

---

## Asynchronous Callbacks in Use

Asynchronous callbacks have historically been the standard way in JavaScript to handle operations that do not complete immediately. The typical pattern involves initiating an asynchronous operation and providing a callback function that the system will invoke upon the operation's completion or if an error occurs.

Common use cases for asynchronous callbacks include: handling user interactions (using event listeners like `button.addEventListener('click', callback)`), performing network requests (older APIs), setting timers (`setTimeout`, `setInterval`), interacting with the file system or accessing databases in server-side environments like **Node.js**.

**Example: Asynchronous Callback with `readline` (Node.js)**

In this **Node.js** example, `rl.question` is asynchronous. It prompts the user and continues execution. The provided callback is only executed *after* the user has typed their answer and pressed Enter.

```javascript
import readline from 'readline'; // Node.js module for reading input
const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

// rl.question is asynchronous. The callback (answer) => {...} runs later.
rl.question('What is your name? ', (answer) => { // This callback is invoked when input is received
  console.log(`Hello, ${answer}!`);
  rl.close(); // Close the readline interface
});

console.log("Question asked. Waiting..."); // This prints immediately, before the user answers
// Expected output in console:
// Question asked. Waiting...
// What is your name? [User types "Alice" and presses Enter]
// Hello, Alice!
```

---

## Timers

JavaScript provides built-in asynchronous functions for scheduling code execution after a delay or at regular intervals:

*   `setTimeout(callback, delay, ...args)`: Executes the `callback` function only **once**, after a specified `delay` in milliseconds has passed. Any arguments provided after the `delay` are passed to the `callback` function. `setTimeout` returns a timer ID which can be used to cancel the scheduled execution.
*   `setInterval(callback, delay, ...args)`: Executes the `callback` function **repeatedly**, at approximately every `delay` milliseconds. Arguments after the `delay` are passed to the callback. `setInterval` returns a timer ID which can be used to stop the repetition.

Execution of these scheduled callbacks happens asynchronously, whenever the call stack is empty after the delay/interval has passed. You can cancel a timer created by `setTimeout` using `clearTimeout(id)` and stop an interval created by `setInterval` using `clearInterval(id)`.

```javascript
// Schedule a callback to run every 1000ms (1 second)
const intervalId = setInterval(() => { console.log('Tick'); }, 1000);
console.log("Timer started.");

// Schedule another callback to stop the interval after 3500ms (3.5 seconds)
setTimeout(() => {
  console.log("Stopping timer.");
  clearInterval(intervalId); // Use the ID returned by setInterval to stop it
}, 3500);

// Expected output:
// Timer started.
// (1s) Tick
// (2s) Tick
// (3s) Tick
// (3.5s) Stopping timer.
// (No more 'Tick' outputs)
```

---

## Error Handling in Asynchronous Callbacks

A common pattern for handling errors in asynchronous functions that use callbacks, particularly prevalent in **Node.js** APIs, is the **"error-first callback"**. In this pattern, the callback function is designed to accept an error object as its *first* argument, followed by the result(s) as subsequent arguments: `(err, result, ...)`.

When the asynchronous operation completes:

*   If an error occurred, the `err` argument will be a non-null Error object. You should check `if (err)` and handle the error (e.g., log it, inform the user).
*   If the operation succeeded, the `err` argument will be `null` or `undefined`, and the `result` argument(s) will contain the successful outcome. You can then proceed to process the result.

This pattern promotes checking for errors first within every asynchronous callback.

**Example: File Read with Error-First Callback (Node.js `fs` module)**

```javascript
import fs from 'fs'; // Node.js File System module

// fs.readFile is asynchronous. The callback (err, data) => {...} runs later.
fs.readFile('file.txt', 'utf8', (err, data) => { // The callback uses the error-first pattern
  if (err) { // Check for an error first
    console.error("Error reading file:", err.message);
    return; // Stop processing on error
  }
  // If no error, process the data
  console.log("File content:", data); // 'data' is available here on success
});

console.log("Read scheduled."); // This prints immediately, before the file is read and the callback runs
// Expected output:
// Read scheduled.
// (If file.txt exists) File content: [contents of file.txt]
// (If file.txt does not exist) Error reading file: [error message]
```

---

## DATABASE ACCESS WITH SQLITE

### Server-Side Persistence (Node.js)

Databases are fundamental for storing application data persistently, meaning data survives after the application stops running. **SQLite** is a popular choice for certain scenarios because it's a lightweight, embedded database. This means the database is stored in a single file on disk, and it doesn't require a separate database server process to run, making it convenient for smaller applications, development, testing, or use as an application file format.

### Using the `sqlite3` Node.js Module

To interact with **SQLite** databases in **Node.js**, the `sqlite3` module is commonly used. You install it via npm: `npm install sqlite3`. Its documentation is available on GitHub: <https://github.com/mapbox/node-sqlite3/wiki>.

Connecting to a database is done asynchronously: `new sqlite3.Database('filename', callback)`. The provided `callback` function is executed asynchronously after the connection attempt completes. It follows the error-first pattern, receiving a single `err` argument.

### Executing Database Queries

All methods for executing database queries using the `sqlite3` module are **asynchronous** and rely on the error-first callback pattern for handling results or errors.

Specific methods for different query types include:

*   `db.all(sql, [params], callback(err, rows))`: Used for `SELECT` queries that are expected to return zero or more rows. The `rows` argument in the callback is an array of row objects, where each object represents a row from the result set.
*   `db.get(sql, [params], callback(err, row))`: Used for `SELECT` queries that are expected to return zero or one row (e.g., querying by a unique ID). The `row` argument in the callback is a single row object if a row was found, or `undefined` if no row matched the query.
*   `db.each(sql, [params], callback(err, row), completeCallback)`: Also for `SELECT` queries returning potentially many rows. It executes the first callback `(err, row)` once for *each* row retrieved. An optional `completeCallback` is executed after all rows have been processed. This method is more memory-efficient than `db.all` for very large result sets as it processes rows individually rather than building a large array in memory.
*   `db.run(sql, [params], callback(err))`: Used for SQL queries that do not return data rows, such as `INSERT`, `UPDATE`, `DELETE`, or `CREATE TABLE`. The callback receives only an `err` argument. Within this callback, `this.changes` (number of rows affected) and `this.lastID` (the ID of the last inserted row for `INSERT`) are available, though accessing `this` correctly requires using a non-arrow function or explicitly binding `this`.

### Parameterized Queries (Preventing SQL Injection)

**CRITICAL SECURITY POINT:** When including external data (like user input) in SQL queries, you **must** use parameterized queries to prevent **SQL Injection** vulnerabilities. Parameterized queries separate the SQL logic from the data values.

In `sqlite3`, you use question marks (`?`) as placeholders within the SQL string for values you want to insert or compare. You then pass an array of the actual values as a separate argument to the query method (e.g., `db.get(sql, [value1, value2], callback)`). The `sqlite3` module safely substitutes these values for the placeholders, ensuring that malicious input is treated purely as data, not executable SQL code. **Never** build SQL strings by concatenating user input directly; this is insecure.

```javascript
// Assume 'userId' comes from user input, e.g., a web form
const userId = 5; // DO NOT build the SQL string like `WHERE id = ${userId}`

// Secure parameterized query example:
db.get('SELECT * FROM users WHERE id = ?', [userId], (err, row) => {
  if (err) { /* handle error */ }
  if (row) { console.log("Found user:", row); }
  else { console.log("User not found."); }
});
```

---

## Example: Database Query (transcript.mjs)

This example demonstrates connecting to an **SQLite** database (`transcript.sqlite`) and executing a `db.all` query using the `sqlite3` module. It highlights the asynchronous nature of the database operations and the use of the error-first callback pattern to handle the query results.

```javascript
import sqlite from 'sqlite3';

// Connect to the database (asynchronous operation)
const db = new sqlite.Database('transcript.sqlite', (err) => {
  if (err) {
    console.error("DB connection error:", err.message);
    throw err; // Rethrow or handle appropriately
  }
  console.log("Database connected."); // This line runs asynchronously after connection success
});

// Define the SQL query
let sql = "SELECT * FROM course LEFT JOIN score ON course.code = score.coursecode";

// Execute the query (asynchronous operation)
db.all(sql, (err, rows) => { // This callback runs only AFTER the query has finished and results are ready
  if (err) {
    console.error("Query error:", err.message);
    // Handle the error, e.g., show an error message to the user
    db.close(); // Close the database connection (asynchronously) even on error
    return;
  }
  console.log("Query results received:");
  // Process the results (the 'rows' array)
  for (let row of rows) {
    console.log(row);
  }

  // Close the database connection (asynchronous operation)
  db.close((closeErr) => { // This callback runs only AFTER the connection is closed
    if (closeErr) console.error(closeErr.message);
    else console.log("DB connection closed.");
  });
});

console.log("Query sent to database... (This prints immediately)"); // This line runs BEFORE the query callback

// Likely output order when run:
// Query sent to database... (This prints immediately)
// Database connected. (Runs after connection, before query results)
// Query results received: (Runs after query completes)
// [data row 1]
// [data row 2]
// ...
// DB connection closed. (Runs after db.close completes)
```

This example clearly shows that the code execution continues *immediately* after scheduling the asynchronous `db.all` operation, and the callback function processing the results runs later, after the database work is done.

---

## Asynchronous Issue Example (Order of Execution)

This example further illustrates the critical point that code immediately following an asynchronous operation call **does not wait** for the operation to complete or its callback to run. Assuming that results from an async operation will be available right after the scheduling call is a common pitfall.

```javascript
import sqlite from 'sqlite3';

const db = new sqlite.Database('data.sqlite', (err) => { if (err) console.error(err.message); });

let result = []; // This variable exists in the synchronous scope

let sql = "SELECT * FROM some_table"; // Assume this is valid SQL

// Schedule the asynchronous query. The callback runs LATER.
db.all(sql, (err, rows) => { // This is the asynchronous callback
  if (err) { console.error("Query error:", err.message); return; }
  // This code runs ONLY when the query results are ready
  for (let row of rows) {
    result.push(row); // The 'result' array is populated *INSIDE* this callback
  }
  console.log("Result array populated INSIDE callback.");
});

console.log('*************'); // This prints FIRST (synchronously)
console.log("Starting loop OUTSIDE callback."); // This prints SECOND (synchronously)

// This loop runs immediately after the db.all call returns, *before* the callback has had a chance to run.
for (let row of result) { // Therefore, the 'result' array is likely STILL EMPTY here
  console.log(row); // This loop probably won't print anything yet
}
console.log("Loop finished OUTSIDE callback."); // This prints THIRD (synchronously)

// Attempt to close the database (asynchronous operation)
db.close((err) => { if (err) console.error(err.message); }); // This callback runs LAST

// Likely output order:
// *************
// Starting loop OUTSIDE callback.
// Loop finished OUTSIDE callback.
// Query sent to database... (implicitly by db.all)
// (Wait for DB query to finish)
// Result array populated INSIDE callback.
// [data row 1]
// [data row 2]
// ... (Output from the loop *inside* the callback, if you added one)
// DB connection closed. (Runs after db.close completes)
```

This demonstrates that you cannot rely on data populated within an asynchronous callback being available in the synchronous code that immediately follows the async call. Any code that depends on the result of an asynchronous operation *must* be placed inside that operation's callback (or subsequent **Promise** handlers/**Async/Await** functions).

---

## Another Asynchronous Query Example (Mixing Sync/Async)

This example highlights the non-deterministic completion order that can occur when a synchronous loop schedules multiple independent asynchronous operations rapidly. While the loop finishes quickly, the order in which the callbacks for the asynchronous operations execute is not guaranteed.

```javascript
import sqlite from 'sqlite3';

const db = new sqlite.Database('data.sqlite', (err) => { if (err) console.error(err.message); });

// db.serialize ensures that the calls within its block run in sequence,
// even though the individual db.run calls are asynchronous.
db.serialize(() => {
    // Schedule CREATE TABLE (async)
    db.run("CREATE TABLE IF NOT EXISTS numbers (number INT)");
    // Schedule DELETE (async) - runs after CREATE because of serialize
    db.run("DELETE FROM numbers");
});

// This synchronous loop schedules 100 async INSERTs and 100 async SELECTs very quickly.
for (let i = 0; i < 100; i++) { // This loop finishes executing synchronously almost instantly
  // Schedule an async INSERT operation
  db.run('INSERT INTO numbers(number) VALUES(1)'); // This runs asynchronously

  // Schedule an async SELECT COUNT operation
  db.all('SELECT count(*) AS tot FROM numbers', (err, rows) => { // This callback runs asynchronously LATER
    if (rows && rows.length > 0) {
        // The order in which these console.log statements appear is NON-DETERMINISTIC.
        // They will print as the COUNT queries complete, which is not necessarily in the order they were scheduled.
        console.log(`Count result (${i}):`, rows[0].tot);
    }
  });
}

console.log("Loop finished (synchronously)."); // This prints BEFORE any of the COUNT callbacks run

// Attempt to close the database (asynchronous operation)
db.close((err) => { if (err) console.error(err.message); }); // This callback runs LAST

// Likely output order:
// Loop finished (synchronously).
// (Followed by 100 lines of "Count result (i): [count]", where 'i' is in random order and the count value varies depending on how many INSERTs have completed by the time the specific SELECT COUNT query runs)
// DB connection closed. (Runs after db.close completes)
```

This illustrates that while you can schedule many async tasks in a synchronous loop, coordinating them or relying on their completion order requires more sophisticated techniques than just raw callbacks, such as managing dependencies explicitly or using **Promises**/**Async/Await**.

---

## Callback Hell (The Problem with Deep Nesting)

While asynchronous callbacks allow non-blocking operations, sequencing *dependent* asynchronous operations using raw callbacks quickly leads to a problem known as **Callback Hell**, or the "**Pyramid of Doom**". This occurs when the code that needs to run after one asynchronous operation completes is placed inside that operation's callback, and the code needing to run after *that* operation completes is placed inside *its* callback, and so on. This creates deeply nested code that is extremely difficult to read, debug, refactor, and manage error handling consistently.

```javascript
// Example: Callback Hell with `readline` (Sequencing Sequential Questions)
import readline from 'readline'; // Node.js module
const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

rl.question('Q1: What is your favorite color? ', (ans1) => { // Q1 callback (first level)
  rl.question('Q2: What is your favorite food? ', (ans2) => { // Q2 callback (nested second level)
    rl.question('Q3: What is your favorite animal? ', (ans3) => { // Q3 callback (nested third level)
      // ... Code to process ans1, ans2, ans3 ...
      console.log(`Your answers: Color=${ans1}, Food=${ans2}, Animal=${ans3}`);
      rl.close(); // Close interface after all questions
    }); // End of Q3 callback
  }); // End of Q2 callback
}); // End of Q1 callback

console.log("Starting questions..."); // Prints immediately
// The nested indentation visually represents the "Pyramid of Doom" structure.
```

This nested structure is difficult to follow and error handling becomes complex (e.g., how to handle an error in Q2's callback and still ensure `rl.close()` is called?). **Promises** and **Async/Await** provide cleaner ways to handle such sequences.

---

## PROMISES

### A Better Way to Handle Asynchronous Results

**Promises**, introduced in ES6 (ECMAScript 2015), provide a more structured and manageable way to handle asynchronous operations and are designed to solve the issues associated with **Callback Hell**. A **Promise** object acts as a placeholder representing the **eventual outcome** (either a successful value or an error) of an asynchronous operation that is expected to complete at some point in the future. **Promises** form the underlying basis for the more modern **`async/await`** syntax.

### Promise States

A **Promise** can be in one of three exclusive states at any given time:

*   **Pending:** The initial state. The asynchronous operation associated with the **Promise** is still ongoing.
*   **Fulfilled:** The operation completed successfully. The **Promise** now has a **fulfillment value** (the result of the operation). This is a **Settled** state.
*   **Rejected:** The operation failed. The **Promise** now has a **rejection reason** (typically an Error object explaining why it failed). This is also a **Settled** state.

Once a **Promise** enters either the **Fulfilled** or **Rejected** state, it is considered **settled**. A **settled** Promise's state is final and cannot change thereafter.

### Creating a Promise

You create a new **Promise** instance using the `new Promise()` constructor: `new Promise(executorFunction)`. The `executorFunction` is a function that you provide to the constructor. It runs immediately when the **Promise** is created and receives two functions as arguments: `resolve` and `reject`.

*   Inside the `executorFunction`, you perform the asynchronous operation.
*   If the operation succeeds, you call the `resolve(value)` function, passing the successful result as the `value`. This transitions the **Promise** from Pending to Fulfilled.
*   If the operation fails, you call the `reject(reason)` function, passing the reason for failure (usually an `Error` object) as the `reason`. This transitions the **Promise** from Pending to Rejected.

```javascript
// Function that returns a Promise representing a delayed success
function delayedSuccess(value) {
  // The executor function runs immediately
  return new Promise((resolve, reject) => {
    // Perform an async operation (a timer here)
    setTimeout(() => {
      console.log("Async operation done!");
      resolve(value); // Call resolve() on success
    }, 500); // Complete after 500ms
  });
}

// Calling delayedSuccess() returns a Promise object, it doesn't block
const myPromise = delayedSuccess("Task Completed!");
console.log("A Promise object has been returned."); // Prints immediately
```

---

## Consuming a Promise (`.then()`, `.catch()`, `.finally()`)

Once you have a **Promise** object, you attach handler functions to it to specify what should happen when the **Promise** settles (either fulfills or rejects). These handlers are registered using methods on the **Promise** instance:

*   `.then(onFulfilled, onRejected)`: This method registers callback functions to handle the Promise's outcome. The optional `onFulfilled` callback is executed only if the **Promise** is **Fulfilled**, receiving the fulfillment value as its argument. The optional `onRejected` callback is executed only if the **Promise** is **Rejected**, receiving the rejection reason as its argument.
*   `.catch(onRejected)`: This method is a shortcut for calling `.then(undefined, onRejected)`. It's specifically used for registering a handler for **Promise** rejections and is commonly placed at the end of a **Promise** chain for centralized error handling.
*   `.finally(onSettled)`: *(Available in ES2017 and later)* This method registers a callback function that is executed regardless of whether the **Promise** fulfills or rejects. The `onSettled` callback receives no arguments. It's useful for performing cleanup tasks (like hiding a loading spinner) that should happen after an asynchronous operation finishes, irrespective of its success or failure.

All `.then()`, `.catch()`, and `.finally()` methods **always return a new Promise instance**. This is the key mechanism that enables **Promise** chaining.

```javascript
delayedSuccess("Done!") // Call function that returns a Promise
  .then(result => { // Attach success handler
    console.log("Fulfilled:", result); // Handles the value from resolve("Done!")
  })
  .catch(error => { // Attach failure handler (will not run in this specific case)
    console.error("Rejected:", error); // Handles the reason from reject(reason)
  })
  .finally(() => { // Attach settlement handler
    console.log("Settled (Success or Failure)."); // Always runs after the promise settles
  });

console.log("Promise scheduled."); // This prints immediately, before any handlers execute
// Expected output:
// A Promise object has been returned. (from previous example snippet, if combined)
// Promise scheduled.
// Async operation done! (after 500ms delay)
// Fulfilled: Done!
// Settled (Success or Failure).
```

---

## Chaining Promises

As mentioned, `.then()`, `.catch()`, and `.finally()` methods return **new Promise** instances. This allows chaining them together to sequence dependent asynchronous operations in a more readable, linear flow, avoiding the nested structure of **Callback Hell**.

The return value of the handler function passed to `.then()` determines the state and value of the *new* **Promise** returned by that `.then()` call:

*   If the handler returns a regular non-Promise value, the new **Promise** returned by `.then()` will be Fulfilled with that value.
*   If the handler returns *another Promise*, the new **Promise** returned by `.then()` will wait for and adopt the state (Fulfillment or Rejection) of the returned Promise. This is how you sequence async operations: the next `.then()` in the chain waits for the **Promise** returned by the previous handler.
*   If the handler throws an error, the new **Promise** returned by `.then()` will be Rejected with that error.

Errors and rejections propagate down the chain. A single `.catch()` handler at the end of the chain can catch any rejection that occurs at any step within the preceding `.then()` calls or the initial **Promise** itself.

**Example: Conceptual Promise Chain**

```javascript
// Assume step1(), step2(result1), and step3(result2) are functions that each return a Promise

step1() // Returns Promise A
  .then(result1 => { // This handler runs when Promise A fulfills
    console.log("Step 1 done with result:", result1);
    // Return a new Promise (from step2). The next .then will wait for this Promise.
    return step2(result1); // Returns Promise B
  })
  .then(result2 => { // This handler runs when Promise B fulfills
    console.log("Step 2 done with result:", result2);
    // Return another new Promise (from step3). The final .then will wait for this Promise.
    return step3(result2); // Returns Promise C
  })
  .then(result3 => { // This handler runs when Promise C fulfills
    console.log("Chain complete with final result:", result3); // Final success handler
  })
  .catch(error => { // This catch handler catches any rejection that occurs anywhere UPSTREAM in the chain (step1, step2, step3, or errors thrown in the .then handlers)
    console.error("A step in the chain failed:", error);
  })
  .finally(() => { // This runs after the chain fully settles (either successfully or due to a rejection)
    console.log("Chain processing settled.");
  });

console.log("Promise chain started."); // This line prints immediately when the chain setup begins
```

---

## Chaining with `fetch()` API

A very common real-world example of **Promise** chaining is using the `fetch()` API in web browsers to make network requests. The `fetch()` function itself returns a **Promise**, and methods on the returned `Response` object (like `.json()` or `.text()`) also return **Promises**, allowing for straightforward chaining.

```javascript
// fetch() returns a Promise that resolves with a Response object
fetch('/api/data') // 1. Initiate fetch request - returns Promise<Response>
  .then(response => { // 2. This handler runs when the fetch Promise fulfills (i.e., receives headers)
    // Check if the HTTP status is OK (200-299). fetch() Promise resolves even on 404/500 status, use response.ok.
    if (!response.ok) {
      // If status is not OK, throw an error. This rejects the Promise returned by *this* then handler.
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    // Return the result of response.json(). response.json() returns a Promise
    return response.json(); // 3. Returns Promise<ParsedData>. The next .then waits for this.
  })
  .then(data => { // 4. This handler runs when the response.json() Promise fulfills (i.e., JSON is parsed)
    console.log('Data received:', data); // Process the successfully parsed data
  })
  .catch(error => { // 5. This catch handler will catch:
    // - Network errors during fetch
    // - The error thrown by `throw new Error(...)` if response.ok is false
    // - Errors during JSON parsing (response.json() rejection)
    // - Any other errors thrown in the preceding .then handlers
    console.error('There was a problem with the fetch operation:', error);
  })
  .finally(() => {
    console.log("Fetch process finished."); // Runs after success or failure
  });

console.log("Fetch operation scheduled."); // This prints immediately
```

---

## Promises in Parallel

While chaining is for *sequential, dependent* async operations, static methods on the `Promise` constructor allow managing *collections* of **Promises** that can run *concurrently* or whose outcome depends on the state of multiple promises.

*   `Promise.all(iterable)`: Takes an iterable (e.g., an array) of **Promises**. It returns a **single new Promise**. This new **Promise**:
    *   Resolves when *all* of the input Promises in the iterable have successfully fulfilled. The fulfillment value is an array containing the fulfillment values of the input Promises, in the same order as the original iterable.
    *   Rejects immediately if *any one* of the input Promises rejects. The rejection reason is the reason of the first **Promise** that rejected.
*   `Promise.race(iterable)`: Takes an iterable of **Promises**. It returns a **single new Promise**. This new **Promise** settles (either fulfills or rejects) as soon as *any one* of the input Promises settles first. The state and value/reason of the returned **Promise** will match the state and value/reason of the first settling input **Promise**.
*   `Promise.allSettled(iterable)`: *(Available in ES2020)* Takes an iterable of **Promises**. It returns a **single new Promise**. This new **Promise** resolves when *all* of the input Promises have settled (either fulfilled or rejected). The fulfillment value is an array of objects, where each object describes the outcome of the corresponding input **Promise** (e.g., `{ status: 'fulfilled', value: ... }` or `{ status: 'rejected', reason: ... }`). This is useful when you want to know the outcome of all promises, even if some fail.
*   `Promise.any(iterable)`: *(Available in ES2021)* Takes an iterable of **Promises**. It returns a **single new Promise**. This new **Promise** fulfills with the value of the *first* input **Promise** that fulfills. If *all* of the input Promises reject, the returned **Promise** rejects with an `AggregateError` (an error object that groups multiple errors).

```javascript
const p1 = Promise.resolve(1); // A promise that resolves immediately with 1
const p2 = new Promise(resolve => setTimeout(() => resolve(2), 100)); // A promise that resolves with 2 after 100ms
const p3 = Promise.resolve(3); // A promise that resolves immediately with 3

// Promise.all waits for all three promises to resolve
Promise.all([p1, p2, p3])
  .then(results => { // This handler runs after p2 resolves (since it's the slowest)
    console.log("All results:", results); // Output: All results: [ 1, 2, 3 ] (order matches input array)
  })
  .catch(error => { // This handler would run if any of p1, p2, or p3 rejected
    console.error("One of the promises failed:", error);
  });

console.log("Promise.all operation started."); // Prints immediately

// Expected output:
// Promise.all operation started.
// (Wait 100ms for p2)
// All results: [ 1, 2, 3 ]
```

---

## ASYNC/AWAIT

### Modern Syntax for Asynchronous Code

**Async/Await**, introduced in ES2017 (ECMAScript 2017), provides a more modern and readable way to write asynchronous code. It is essentially **syntactic sugar** built **on top of Promises**. The goal is to allow asynchronous code to be written with a structure that closely resembles synchronous code, making it much easier to understand, particularly when dealing with sequences of operations. It also simplifies error handling by allowing the use of standard `try...catch` blocks.

#### How it Works:

1.  `async function`: A function declared with the `async` keyword (e.g., `async function myFunc() {}`, `const myFunc = async () => {}`). An `async` function always implicitly returns a **Promise**. If the function executes without throwing an error and returns a value, the returned **Promise** will fulfill with that value. If the function throws an error, the returned **Promise** will be rejected with that error. The `await` keyword can **only** be used inside an `async` function (or at the top-level of ES modules).
2.  `await Promise`: The `await` keyword is used before a **Promise** within an `async` function. When encountered, it **pauses** the execution of the `async` function until the awaited **Promise** settles.
    *   If the awaited **Promise** **fulfills**, `await` "unwraps" the fulfillment value, and the `async` function's execution resumes from that point with the unwrapped value.
    *   If the awaited **Promise** **rejects**, `await` effectively "throws" the rejection reason as a JavaScript error. This causes the execution flow to jump to the nearest `catch` block within a surrounding `try...catch` statement inside the `async` function.

```javascript
// Assume asyncFunc(shouldSucceed) is a function that returns a Promise
// This Promise resolves with "Success!" if shouldSucceed is true, or rejects with "Failure!" if false.
async function asyncFunc(shouldSucceed) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (shouldSucceed) resolve("Success!");
            else reject(new Error("Failure!"));
        }, 200); // Simulate async delay
    });
}

async function performTask() { // This is an async function
  console.log("Starting task...");
  try { // Use try...catch for error handling, similar to synchronous code
    // await pauses execution until asyncFunc(true) Promise resolves
    let result1 = await asyncFunc(true); // result1 will be "Success!"
    console.log("Task 1 result:", result1); // This line runs ONLY after result1 is ready

    // await pauses execution until asyncFunc(false) Promise settles
    let result2 = await asyncFunc(false); // This Promise will reject
    // If asyncFunc(false) rejects, the line below is SKIPPED, and execution jumps to the catch block
    console.log("Task 2 result:", result2);

  } catch (error) { // This block catches the error thrown by await if the awaited Promise rejects
    console.error("Caught an error during task:", error.message);
  } finally { // This block runs after try or catch finishes
    console.log("Task finished.");
  }
}

// Calling an async function does NOT block the caller; it returns a Promise
performTask();
console.log("Task execution scheduled (via calling performTask())."); // Prints immediately

// Likely output:
// Task execution scheduled (via calling performTask()).
// Starting task...
// (Wait 200ms for asyncFunc(true))
// Task 1 result: Success!
// (Wait 200ms for asyncFunc(false) - which rejects)
// Caught an error during task: Failure!
// Task finished.
```

---

## Async Functions (Syntax)

The `async` keyword can be placed before various function definition syntaxes:

*   Before a traditional `function` declaration: `async function declareAsync() {}`
*   Before a `function` expression: `const expressAsync = async function() {};`
*   Before an **Arrow Function**: `const arrowAsync = async () => {};`

---

## The `await` Operator

The `await` operator is specifically designed to be used with **Promises**. Its purpose is to pause the execution of the enclosing `async` function until the **Promise** it is applied to settles.

*   Syntax: `await promise`
*   Usage context: Can **only** be used inside an `async` function or at the top-level of an ES module (supported in newer environments).
*   Behavior on settlement:
    *   If the awaited **Promise** fulfills, `await` evaluates to the Promise's fulfillment value.
    *   If the awaited **Promise** rejects, `await` throws the Promise's rejection reason as an error. This allows asynchronous errors to be caught using standard synchronous-looking `try...catch` blocks within `async` functions.