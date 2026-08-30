# JavaScript: Objects and Functions

## JavaScript Objects: Key Characteristics

JavaScript objects are fundamentally **classless**, created directly rather than from blueprints. This makes them highly **dynamic**, allowing properties and methods to be added, removed, or changed even after an object is created. While standard objects lack built-in access control keywords like `private` or `public`, patterns utilizing closures can emulate privacy. Notably, functions in JavaScript are **first-class objects**, meaning they can be treated like any other value, including being assigned as properties to an object (where they are then called methods).

---

## Understanding the Object Structure

At its core, an object is essentially an **unordered collection of properties**. Each individual property consists of a unique **name** (or key, which is typically a string or Symbol) and an associated **value**. This value can be any JavaScript data type. For complex types (like other objects or arrays), the value stored is actually a **reference** to the underlying data. Properties are accessed and updated using their name. The most common and straightforward method for creating objects is using the **object literal syntax**, which looks like `{ name: value, ... }`.

```javascript
let point = { x: 2, y: 5 };

let book = {
  author: "Enrico",
  title: "Learning JS",
  pages: 520
};
```

---

## Object Properties in Detail

Let's delve deeper into properties. Property **names** serve as the unique identifiers within a given object. These names can be defined when using object literals, added dynamically after creation, or removed using the `delete` operator. Property **values**, on the other hand, can hold any JavaScript data type. Primitive values (like numbers, strings, booleans) are stored directly by value. However, complex types (like arrays or other objects) are stored by reference, meaning modifications made through one reference will be visible via all other references to that same complex value. As mentioned, when a property's value is a function, it is referred to as a **method**.

---

## Accessing Object Properties

There are two primary ways to access or update the values of an object's properties:

1.  **Dot Notation:** `object.propertyName`. This is generally the preferred method when the property name is a valid JavaScript identifier (e.g., doesn't contain spaces or special characters, doesn't start with a number).
2.  **Square Bracket Notation:** `object["propertyName"]`. This notation is necessary when the property name is not a valid identifier (e.g., `"chapter pages"`) or when you need to access a property whose name is stored in a variable, allowing for dynamic access.

```javascript
let book = { author: "Enrico", "chapter pages": 90 };

let authorName = book.author; // Accessed using Dot notation
let chapterPages = book["chapter pages"]; // Accessed using Square brackets (required for invalid identifier)

book.author = "Luigi"; // Modified using Dot notation
book["chapter pages"] = 100; // Modified using Square brackets
```

---

## Objects Behaving Like Associative Arrays

The square bracket notation (`object[variable]`) allows JavaScript objects to effectively behave like associative arrays or hash maps, using string keys to access values. Importantly, if you assign a value to a property name that does not currently exist on the object using either dot or square bracket notation, the property is automatically **created**. Similarly, the `delete` operator can be used to remove a property from an object.

```javascript
let person = {}; // Start with an empty object
let propName = "age";

person.name = "Alice"; // Create property 'name' using dot notation
person[propName] = 30; // Create property 'age' using square bracket notation with a variable

console.log(person); // Output: { name: 'Alice', age: 30 }
delete person.name; // Delete the property 'name'
console.log(person); // Output: { age: 30 }
```

---

## Computed Property Names

*(Available in ES6 and later)* The square bracket syntax `[variableOrExpression]` can also be used *within* an object literal when initially defining the object. This allows you to set property names dynamically based on the value of a variable or the result of an expression evaluated at the time the object is created.

```javascript
let propKey = "model";
let car = {
  make: "Toyota",
  [propKey]: "Camry" // Here, the property name is computed from the value of propKey ('model')
};
console.log(car.model); // Access using dot notation: Camry
// Contrast with car[propKey] which accesses the property using the variable
```
This differs from accessing `object[variable]` after the object is created, although both use the variable's value as the key.

---

## Handling Property Access Errors

A common source of errors occurs when attempting to access a property that does not exist on an object; accessing `object.nonExistentProp` simply returns `undefined`. However, attempting to chain access (`object.prop1.prop2`) when an intermediate property (`prop1` in this case) is `undefined` or `null` will result in a `TypeError`. To prevent this, you should check for the existence of intermediate properties. This can be done using verbose nested `if` statements, using logical `&&` short-circuiting (e.g., `book && book.author && book.author.surname`), or, most safely and concisely, using the **Optional Chaining (`?.`)** operator.

*(Available in ES2020 and later)* Optional Chaining (`object?.prop1?.prop2`) allows you to safely access deeply nested properties. If any part of the chain before the `?.` is `null` or `undefined`, the entire expression evaluates to `undefined` instead of throwing a `TypeError`.

---

## Iterating Over Object Properties

Several methods exist for iterating over the properties of an object:

*   **`for...in` loop:** This loop iterates over the names (keys) of all **enumerable** properties belonging to the object itself, as well as any enumerable properties inherited from its prototype chain.

    ```javascript
    let protoObj = { inherited: 'from prototype' };
    let ownObj = Object.create(protoObj);
    ownObj.own = 'my own';

    for (let key in ownObj) {
      console.log(key); // Output: own, then inherited (order not guaranteed)
    }
    ```
*   For more control, particularly to avoid iterating over inherited properties, use the static methods available on the `Object` constructor:
    *   **`Object.keys(obj)`:** Returns an array containing the names of the object's *own* enumerable string properties.
    *   **`Object.values(obj)`:** Returns an array containing the values of the object's *own* enumerable string properties.
    *   **`Object.entries(obj)`:** Returns an array where each element is a `[key, value]` pair for the object's *own* enumerable string properties.

These `Object` methods return arrays, which can then be iterated using standard array methods like `forEach`, `for...of`, etc.

```javascript
let obj = { name: "Test", value: 10 };
console.log(Object.keys(obj));   // [ 'name', 'value' ]
console.log(Object.values(obj)); // [ 'Test', 10 ]
console.log(Object.entries(obj)); // [ [ 'name', 'Test' ], [ 'value', 10 ] ]

for (const [key, value] of Object.entries(obj)) {
  console.log(`${key}: ${value}`); // name: Test, value: 10
}
```

---

## Copying Objects

When you use simple assignment (`=`) with objects (`let obj2 = obj1;`), you are creating a **reference copy**. Both `obj1` and `obj2` variables will point to the exact same object in memory. Consequently, modifying the object through one variable will affect the object when accessed via the other.

To create a *new*, distinct object based on an existing one, you need different techniques:

*   **Shallow Copy:** This method creates a new object and copies the top-level properties from the source object. However, if any of the source object's properties hold references to nested objects or arrays, only the *references* are copied, not the nested structures themselves. This means the original and the shallow copy will *share* any nested objects. You can achieve a shallow copy using `Object.assign({}, source)` or, more commonly in modern JavaScript, the **Spread operator (`{ ...source }`)** (available in ES2018+).

    ```javascript
    let obj1 = { a: 1, nested: { b: 2 } };
    let obj2 = { ...obj1 }; // Creates a new object, copies 'a' and the reference to 'nested'
    obj2.a = 10; // Changes obj2's 'a', obj1.a remains 1
    obj2.nested.b = 20; // Changes the shared nested object, so obj1.nested.b is also 20
    ```
    The `Object.assign(target, source1, ...)` method copies enumerable own properties from one or more source objects to a target object. If the target is `{}`, it effectively creates a new object. The Spread operator `{ ...source1, ...source2 }` provides a concise way to perform a shallow copy and merge properties from multiple sources into a new object (properties from later sources overwrite those from earlier ones).
*   **Deep Copy:** This method creates a completely independent new object, recursively copying all properties and their values, including creating new copies of any nested objects or arrays. JavaScript does not have a simple built-in way to perform a robust deep copy. This typically requires custom logic or using specialized libraries.

---

## Checking if a Property Exists

To determine if a property exists on an object, use the following:

*   **`'propertyName' in object`:** This operator checks if a property with the specified name exists anywhere in the object's prototype chain (i.e., as an **own property** or an **inherited property**).
*   **`object.hasOwnProperty('propertyName')`:** This method checks only if the property is an **own property** of the object itself (defined directly on the object), ignoring the prototype chain. For most checks where you only care about properties directly on the object instance, `hasOwnProperty` is generally preferred.

---

## Object Creation: Summary of Methods

While object literals are common, several methods exist for creating objects:

*   `{}`: The **Object literal** syntax is the simplest and most common way to create a single object directly.
*   `new Object()`: Using the `Object` constructor with `new` is another way to create an empty object, but it is less common than `{}`.
*   `Object.create(prototype)`: This method creates a new object and explicitly sets its prototype to a specified object.
*   **Constructor Function Pattern:** A traditional pattern involves defining a regular function (often capitalized by convention, e.g., `function Name(...)`) and calling it using the `new` keyword (`new Name(...)`). This pattern is used to create multiple objects that share a similar structure and behavior. Inside a constructor function called with `new`, the `this` keyword refers to the new object instance being built. The `new` operator automates several steps: it creates a new empty object, links it to the constructor's prototype, binds `this` to the new object inside the function, and finally returns the new object. ES6 `class` syntax is primarily syntactic sugar over this underlying constructor function and prototype mechanism.

```javascript
// Example using the Constructor Function Pattern
function Car(make, model) {
  this.make = make; // Define properties on the new object (this)
  this.model = model;
  this.display = function() { console.log(this.make, this.model); }; // Define a method
}
let myCar = new Car("Honda", "Civic"); // Use 'new' to create an instance
myCar.display(); // Call the method on the instance: Honda Civic
```

---

## Functions

Functions in JavaScript are fundamental building blocks. They are reusable blocks of code designed to perform a specific task or calculate a value. Functions have their own isolated **scope**, can accept data through **parameters** when called, and can return a **value** back to the caller (if no `return` statement or a `return;` statement is used, they implicitly return `undefined`). As mentioned, functions are **first-class objects** in JavaScript; this means they can be assigned to variables, passed as arguments to other functions, returned as values from other functions, and stored in data structures like arrays or objects.

---

## Declaring Functions: 3 Main Ways

There are three primary syntaxes for declaring functions in JavaScript:

1.  **Function Declaration:** Defined using the `function` keyword followed by a name: `function name() {}`. Functions declared this way are **hoisted**, meaning they can typically be called in your code *before* their actual definition appears in the source file (within the same scope).
2.  **Function Expression:** A function defined as part of an expression, often assigned to a variable: `const name = function() {}`. Function expressions are **not hoisted** and must be defined *before* they are called. They can be anonymous (no name after `function`) or named (primarily for easier debugging).
3.  **Arrow Function:** (`=>`) A more concise syntax introduced in **ES6**. `const name = () => {}`. Arrow functions are also **not hoisted**. They have a different, lexically bound `this` value compared to traditional functions.

---

## Classic Function Declarations

The traditional way to define a named function is using a Function Declaration. This involves the `function` keyword, followed by the function's name, a list of parameters enclosed in parentheses `()`, and the function body enclosed in curly braces `{}`. The `return` statement is used to send a specific value back to the code that called the function.

```javascript
function add(a, b) { // Function declaration with name and parameters
  return a + b; // Returns the sum
}
let sum = add(2, 3); // Call the function with arguments 2 and 3; sum is 5
```

---

## Function Expressions: Assignment to Variables

A Function Expression defines a function as part of a larger expression. The most common use is assigning the function value to a variable (e.g., `let cube = function(...) { ... };`). The variable name is then used to call the function. This highlights the concept of functions being values that can be assigned and passed around.

```javascript
let multiply = function(x, y) { // Function expression assigned to multiply variable
  return x * y;
};
let result = multiply(5, 6); // Call using the variable name; result is 30
```

---

## Function Parameters

Parameters are placeholders defined in the function's signature (`(param1, param2)`). When the function is called, the values provided (arguments) are assigned to these parameters. **Default parameter values** (`(param = defaultValue)`) can be specified starting from ES6, allowing parameters to have a fallback value if no corresponding argument is provided or the argument is `undefined`.

When arguments are passed:
*   Primitive types (numbers, strings, booleans) are passed by value, meaning a copy of the value is assigned to the parameter. Changes to the parameter inside the function do not affect the original value outside.
*   Complex types (objects, arrays) are passed by reference value. A copy of the *reference* to the object/array is assigned to the parameter. This means both the parameter and the original variable point to the same object/array in memory, so modifications to the object/array's *contents* via the parameter *will* affect the original object/array outside the function. Reassigning the parameter variable itself to a *new* object/array does not affect the original.
*   If a function is called with fewer arguments than parameters, the missing parameters will have the value `undefined`. You can handle optional parameters by checking if their value is `undefined` (or using logical operators like `&&` or the nullish coalescing operator `??`).

---

## Handling a Variable Number of Parameters

*(Available in ES6 and later)* The **rest parameter syntax** (`...parameterName`) provides a concise way to handle functions that can accept a variable number of arguments. The rest parameter must be the last parameter in the function's definition. It collects all remaining arguments passed to the function into a **real array**.

```javascript
// The 'base' argument is the first one, remaining arguments are collected into the 'numbers' array
function sum(base, ...numbers) { // numbers is now a standard array [1, 2, 3]
  let total = base;
  for (let num of numbers) { // Iterate over the array of rest parameters
    total += num;
  }
  return total;
}
console.log(sum(10, 1, 2, 3)); // Output: 16 (10 + 1 + 2 + 3)
```

---

## Arrow Functions: Concise Syntax

*(Available in ES6 and later)* Arrow functions offer a shorter syntax for writing function expressions. They are defined using the `=>` syntax.

*   If the function body consists of a single expression, you can omit the curly braces `{}` and the `return` keyword. The result of the expression will be implicitly returned. (`param => param * 2`).
*   If the body requires multiple statements, you must use curly braces `{}` and an explicit `return` statement if a value needs to be returned.
*   To implicitly return an object literal from a single-expression arrow function, you must wrap the object literal in parentheses: `() => ({ key: value })`.

```javascript
const double = x => x * 2; // Single parameter, single expression body -> implicit return
const process = (a, b) => { // Multiple parameters, multi-line body -> explicit return needed
  let sum = a + b;
  return sum * 2;
};
const create = () => ({ id: 1 }); // Zero parameters, implicitly returning an object literal
```
Arrow functions also do not have their own `this` binding; they inherit `this` from the surrounding lexical context.

---

## Arrow Function Parameter Syntax Variations

The syntax for parameters in an arrow function depends on the number of parameters:

*   **Zero parameters:** Parentheses are required: `() => {}`.
*   **One parameter:** Parentheses are optional: `x => {}` or `(x) => {}`.
*   **Multiple parameters:** Parentheses are required: `(a, b) => {}`.
*   **Rest parameters:** Parentheses are required if used with other parameters: `(...args) => {}` or `(p1, ...args) => {}`.
*   **Default values:** Parentheses are required when using default values: `(param = default) => {}`.

---

## Function Return Values

A function implicitly returns the value `undefined` if it finishes executing without encountering a `return` statement, or if it executes a `return;` statement without a value. The `return value;` statement sends a specific value back to the caller. A function can only return one value directly; if you need to return multiple values, package them into a single array or object, which the caller can then potentially destructure. As mentioned, arrow functions with a single expression body have an implicit return.

```javascript
function getData() {
  return [1, "item"]; // Package multiple values into an array
}
const [id, type] = getData(); // Use array destructuring to unpack the returned values
```

---

## Nested Functions

JavaScript allows you to define functions inside other functions. These are called **nested** or inner functions. Inner functions have access to the variables and parameters of their containing (outer) function's scope. Nested functions are typically private to the outer function where they are defined.

```javascript
function outer(x) {
  function inner(y) { // This is a nested function
    return x + y; // The inner function accesses 'x' from the outer function's scope
  }
  return inner(10); // The outer function calls the inner function
}
console.log(outer(5)); // Output: 15
```

---

## Closures

**Closures** are a fundamental and powerful concept in JavaScript. A closure is created when an inner function is defined within another function. The inner function "remembers" and retains access to the variables and parameters from its outer function's scope *even after* the outer function has finished executing and returned. The inner function essentially "closes over" or captures its surrounding environment. Closures are widely used to maintain private state, create function factories, and build stateful functions.

```javascript
function createCounter() {
  let count = 0; // This variable is local to createCounter (private state)
  return function() { // This is the inner function, which forms the closure
    count++; // It accesses and modifies 'count' from createCounter's scope
    return count;
  };
}
const counter1 = createCounter(); // counter1 is the inner function returned by the first call
const counter2 = createCounter(); // counter2 is a *new* inner function with its *own* 'count' variable

console.log(counter1()); // Output: 1 (counter1's 'count' is now 1)
console.log(counter1()); // Output: 2 (counter1's 'count' persists and is incremented)
console.log(counter2()); // Output: 1 (counter2 has its own independent 'count' starting at 0)
```

---

## Using Closures to Emulate Objects (Stateful Functions)

Closures are a key mechanism for allowing functions to maintain private state across multiple calls, effectively emulating some aspects of object-oriented private members. The outer function acts as a factory (like `createCounter` above), returning one or more inner functions. These inner functions form closures that share access to the variables declared in the outer function's scope. These closed-over variables act as the private state, accessible only by the inner functions returned by the factory.

```javascript
function personFactory(name) {
  let _age = 0; // This is a private variable, closed over by the returned object's methods
  return { // This object contains methods that are inner functions (closures)
    getName: () => name, // This closure accesses the outer function's 'name' parameter
    getAge: () => _age, // This closure accesses the private '_age' variable
    celebrateBirthday: () => { _age++; } // This closure modifies the private '_age' variable
  };
}
const alice = personFactory("Alice");
console.log(alice.getName()); // Output: Alice
alice.celebrateBirthday();
console.log(alice.getAge()); // Output: 1
// console.log(alice._age); // Output: undefined. The _age variable is private and not directly accessible from outside.
```

---

## Immediately Invoked Function Expressions (IIFE)

An **Immediately Invoked Function Expression (IIFE)** is a function that is both defined and executed immediately after its definition. The syntax is `(function() { ... })();`. The primary purpose of an IIFE is to create a private, isolated scope for variables and functions declared inside it, preventing them from polluting the global scope or other surrounding scopes. IIFEs can receive arguments by passing values into the final parentheses `(function(arg){...})(value)`, and they can also return a value.

```javascript
(function() { // Define the function
  let temp = 10; // Variable 'temp' is local to this IIFE's scope
  console.log(temp); // Output: 10
})(); // Immediately invoke the function

// console.log(temp); // This would result in a ReferenceError, as 'temp' is not accessible outside the IIFE's scope.
```
By combining IIFEs and Closures, you can create structures that act like single instances (singletons) with private state.

```javascript
const singletonCounter = (function() { // The IIFE creates a private scope
  let privateCount = 0; // This variable is only accessible within the IIFE's scope
  return { // The IIFE returns an object containing functions (closures)
    increment: () => privateCount++, // This closure accesses/modifies privateCount
    getValue: () => privateCount // This closure accesses privateCount
  };
})(); // The IIFE is executed immediately, and its return value (the object) is assigned to singletonCounter

console.log(singletonCounter.getValue()); // Output: 0
console.log(singletonCounter.increment()); // Output: 0 (returns value *before* increment)
console.log(singletonCounter.getValue()); // Output: 1 (the privateCount state was maintained)
```

---

## Constructor Functions Revisited

Returning to object creation, the **Constructor Function Pattern** utilizes a regular JavaScript function definition (conventionally capitalized, e.g., `function Book(title, author)`) and calls it using the `new` keyword (e.g., `let myBook = new Book(...)`). This pattern is specifically intended for creating multiple objects that are instances of a conceptual "type" and share a common structure and initial state. When a function is invoked with `new`, the `this` keyword inside that function refers to the newly created object instance being constructed. The `new` operator automates the process: it creates a brand new, empty object, sets up the new object's internal link to the constructor function's `prototype` property, binds the `this` keyword within the constructor function to this new object, executes the constructor function's code (which typically initializes properties on `this`), and finally, unless the constructor function explicitly returns a non-primitive value, it returns the newly created `this` object.

```javascript
function Book(title, author) { // Constructor Function (capitalized by convention)
  this.title = title; // Initialize properties on the new object ('this')
  this.author = author;
}
let myBook = new Book("JavaScript", "Anon"); // Use 'new' to create an instance
console.log(myBook.title); // Access property: JavaScript
```

---

## Handling Dates

JavaScript includes a built-in `Date` object for working with dates and times. A `Date` object fundamentally represents a single point in time, measured as the number of milliseconds that have elapsed since the **Unix epoch** (January 1, 1970, 00:00:00 Coordinated Universal Time - UTC). Many methods of the native `Date` object operate based on the user's **local time zone**, which can lead to inconsistencies when dealing with different time zones or standardizing operations. While native `Date` can handle basic tasks, its limitations, particularly around reliable string parsing, complex date calculations, inconsistent formatting across environments, and timezone handling, are significant. Date comparisons can be done by converting Date objects to their numerical timestamp representation.

```javascript
let now = new Date(); // Creates a Date object for the current moment
let specificDate = new Date(2023, 9, 27, 12, 0); // Creates a date: Oct 27, 2023, 12:00:00 (Month is 0-indexed: 0=Jan, 9=Oct)
console.log(now.getFullYear()); // Get the local year (e.g., 2023)
```

---

## Robust Date/Time Handling: Libraries

Due to the native `Date` object's limitations, particularly regarding time zones, reliable parsing of various formats, performing complex calculations (like adding months or finding the difference between dates in different units), and consistent formatting, using dedicated third-party date and time libraries is highly recommended or even necessary for most serious applications. Popular and well-maintained options in the JavaScript ecosystem include **Luxon**, **Day.js**, **date-fns**, and **js-joda**. **Moment.js**, while historically very popular, is now in maintenance mode and not recommended for new projects.

---

## Introduction to Day.js Library

**Day.js** is a prominent choice among modern date/time libraries. It is known for being lightweight (~2kB compressed), **immutable** (date manipulation methods return new objects, not modifying the original), and having an API largely compatible with the widely used (but now legacy) Moment.js, making it easy for developers familiar with Moment.js. It works seamlessly in both Node.js environments and browsers. Day.js keeps its core small by providing extended features through a plugin system.

Installation is typically done via npm: `npm install dayjs`. You import it using `require('dayjs')` in CommonJS or `import dayjs from 'dayjs'` in ES Module environments.

Basic Usage Examples:
*   `dayjs()`: Creates a Day.js object for the current date and time.
*   `dayjs(string)`: Attempts to parse a date/time string. Using ISO 8601 format (`YYYY-MM-DDTHH:mm:ssZ`) is strongly recommended for reliability.
*   `dayjs(Date)`: Creates a Day.js object from a native JavaScript `Date` object.
*   `dayjs.unix(timestamp)`: Creates a Day.js object from a Unix timestamp (number of *seconds* since the epoch).
*   `.format(tokens)`: Returns a formatted date/time string based on a token pattern (e.g., `'YYYY/MM/DD HH:mm:ss'`).
*   `.toDate()`: Converts a Day.js object back into a native `Date` object.

By default, Day.js operations occur in the user's **local time zone**.

---

## Basic Operations with Day.js

Day.js objects provide methods for various operations:

*   **Creating:** As seen before: `dayjs()`, `dayjs(string)`, `dayjs(Date)`, `dayjs.unix()`. You can also create from parts: `dayjs({ year: 2023, month: 9, day: 27 })`.
*   **Displaying:** Use the `.format(tokens)` method to get a formatted string.
*   **Getting:** Use `.get(unit)` to extract specific units like `'year'`, `'month'`, `'day'`, `'hour'`, `'minute'`, `'second'`.
*   **Setting:** Use `.set(unit, value)` to set a specific unit. Crucially, this method returns a **new** Day.js object with the updated value; the original object is **immutable**.

```javascript
const dayjs = require('dayjs'); // or import dayjs from 'dayjs';
let now = dayjs(); // Current date/time
console.log(now.format('YYYY-MM-DD')); // Display formatted date
console.log(now.get('year')); // Get the year

let nextYear = now.set('year', now.get('year') + 1); // Create a *new* object for next year
console.log(now.format('YYYY'));      // Original 'now' is unchanged
console.log(nextYear.format('YYYY')); // 'nextYear' is the new date
```

---

## Date Manipulation and Comparison with Day.js

Day.js excels at making date arithmetic and comparisons straightforward.

Manipulation methods are consistent and return **new**, immutable Day.js objects:
*   `.add(value, unit)`: Adds a specified value in a given unit (e.g., `day`, `month`, `year`, `hour`) to the date.
*   `.subtract(value, unit)`: Subtracts a specified value in a given unit from the date.
*   `.startOf(unit)`: Sets the date to the beginning of a specified unit (e.g., `day`, `month`, `year`).
*   `.endOf(unit)`: Sets the date to the end of a specified unit.
*   `.diff(d2, unit, precise?)`: Calculates the difference between the current Day.js object and another date `d2` in a specified unit. The optional `precise` boolean argument determines if a floating-point number should be returned for partial units.

Comparison methods return a boolean value (`true` or `false`):
*   `.isBefore(d2)`: Checks if the date is before `d2`.
*   `.isSame(d2)`: Checks if the date is the same as `d2`.
*   `.isAfter(d2)`: Checks if the date is after `d2`.
*   Plugins provide additional comparison methods like `.isSameOrBefore()`, `.isSameOrAfter()`, and `.isBetween()`.

```javascript
let date1 = dayjs('2023-10-27');
let date2 = date1.add(1, 'day'); // date2 is '2023-10-28', date1 is unchanged

console.log(date1.isBefore(date2)); // Output: true
console.log(date2.diff(date1, 'day')); // Output: 1 (difference in days)
console.log(date1.diff(date2, 'hour')); // Output: -24 (difference in hours)
```

---

## Day.js Plugins

To keep the core library small, Day.js provides non-core features via a plugin system. Using a plugin involves two steps: first, `require` or `import` the specific plugin file (usually found under `dayjs/plugin/`), and second, extend Day.js by calling `dayjs.extend(plugin)`.

```javascript
const dayjs = require('dayjs');
const isLeapYear = require('dayjs/plugin/isLeapYear'); // Require the plugin file
dayjs.extend(isLeapYear); // Extend Day.js with the plugin's functionality

console.log(dayjs('2024-01-01').isLeapYear()); // Output: true (2024 is a leap year)
```

---

## Advanced Day.js Topics (Overview)

Day.js offers a range of plugins for advanced functionalities, including: **Localization** (using specific locale files from `dayjs/locale/` and `dayjs.locale()`), handling **Durations** (periods of time), working with **Time Zones** (using plugins like `utc` and `timezone`), displaying dates relative to "now" (e.g., "2 hours ago" using the `relativeTime` plugin), and more robust parsing of non-standard date/time strings (with the `customParseFormat` plugin).