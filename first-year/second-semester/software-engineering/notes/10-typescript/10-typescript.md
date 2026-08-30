# TypeScript

*   **Context:** This material covers the foundational technologies needed for implementing the project, specifically focusing on the second task.
*   **Topic Sequence:** This lesson focuses on **TypeScript**. Future lessons will cover ORM and provide an overview of the project structure.

## What is TypeScript?

*   **Core Definition:** TypeScript is a programming language developed and maintained by Microsoft as an open source project (introduced in 2012).
*   **Relationship to JavaScript:** It is defined as a **superset of JavaScript**. This means:
    *   TypeScript includes all the features of standard JavaScript (EcmaScript).
    *   It adds new features on top of JavaScript.
*   **Key Added Feature:** The most significant addition is **static type features**, which are largely absent in original JavaScript.
*   **Transpiled Language:** TypeScript code is not executed directly by browsers or Node.js. Instead, it's a **transpiled** language.
    *   **Transpilation Process:** TypeScript code goes through a process by the TypeScript compiler (or other build tools) where it is **translated ("transpiled") into standard JavaScript**.
    *   **Execution:** The resulting standard JavaScript code is what is then executed by the JavaScript engine in the browser or Node.js.

### Why Choose TypeScript for Development?

Using TypeScript offers several significant advantages:

*   **Prevents Programming Mistakes:** By adding static type checking, TypeScript allows many common coding errors (like trying to call a method on a value that might be `null` or passing a number to a function that expects a string) to be caught by the compiler *before* the program even runs. This significantly reduces bugs found at runtime.
*   **Enforces Data Constraints:** It allows developers to define and enforce specific data types and structures for variables, function parameters, and object properties, ensuring data consistency.
*   **Improves Correctness and Maintainability:** The static type information makes code easier to understand. Developers and tools (like IDEs) can quickly see what type of data a variable is expected to hold or what type a function expects and returns, improving code correctness and long-term maintainability.
*   **Access to Modern JavaScript Features:** TypeScript supports and integrates with the latest features from ECMAScript (ES6, ES7, and beyond) often before they are fully supported in all target execution environments, allowing you to write modern JavaScript syntax while relying on TypeScript's compiler to ensure compatibility.
*   **Cross-Platform and Cross-Browser Compatibility:** Since the final output is standard JavaScript, TypeScript code runs wherever JavaScript runs, including across different operating systems and web browsers.
*   **Excellent Tooling Support:** TypeScript provides rich support for developer tools (IDEs, code editors). This includes features like:
    *   IntelliSense (autocompletion based on types).
    *   Code navigation and refactoring tools.
    *   Early error highlighting based on type mismatches.
*   **Backward Compatibility:** A major advantage is that **all existing valid JavaScript code is also valid TypeScript code**. You can gradually introduce TypeScript into an existing JavaScript project.

### TypeScript as a Superset (Reinforced)

*   TypeScript builds upon standard JavaScript. It takes all of JavaScript's syntax and features and adds powerful new capabilities primarily focused on **typing**.
*   **Added Features:** This includes:
    *   **Strong Typing:** Explicitly defining and checking data types.
    *   **Interfaces:** Defining the "shape" or structure of objects.
    *   **Generics:** Creating reusable code components that can work with different types.
    *   Features from modern ECMAScript standards (like arrow functions, classes, modules).
*   **Compilation Target:** All these TypeScript-specific features and modern JavaScript syntax are ultimately processed by the TypeScript compiler and translated ("transpiled") back into standard JavaScript code that can be understood by the target execution environment.

---

## Key Features: Typing System

TypeScript's static type system is its defining characteristic.

*   **Structural Typing (Duck Typing):** TypeScript's type system is based on the **shape** or structure of the data, rather than relying solely on explicit declarations or inheritance hierarchies (which is how type systems work in nominal languages like Java or C#).
    *   **Concept:** If two different objects or types have the same properties with compatible types, TypeScript considers them compatible, even if they were declared using different names or in different parts of the code.
    *   **Analogy:** This is sometimes referred to as **Duck Typing** – "If it walks like a duck and quacks like a duck, it's probably a duck." TypeScript checks if the "shape" of the data matches what is expected.
    *   **Example:**
        ```typescript
        type Point2D = { x: number, y: number };
        interface Vector2D { x: number, y: number }; // Different declaration syntax

        let p: Point2D = { x: 10, y: 20 };
        let v: Vector2D = { x: 5, y: 15 };

        // TypeScript allows this assignment because 'v' has the same 'shape' as Point2D
        p = v; // OK
        ```
        *Even though `Point2D` and `Vector2D` are defined differently, they are structurally compatible because they both have a `number` property named `x` and a `number` property named `y`.*

### Primitive Types

TypeScript supports the same fundamental primitive data types as JavaScript. These are not objects and are immutable.

*   `string`: Represents text. Can be enclosed in single (`'...'`) or double (`"..."`) quotes, or backticks (``` `...` ```).
*   `number`: Represents both integers and floating-point numbers.
*   `boolean`: Represents logical values (`true` or `false`).
*   `null`: Represents an intentional absence of any object value.
*   `undefined`: Represents a variable that has been declared but has not yet been assigned a value. Note that `null` and `undefined` are distinct types and values in JavaScript/TypeScript.
*   `symbol`: Represents a unique, immutable identifier (used as object property keys).
*   `bigint`: Represents integers of arbitrary precision, used for numbers larger than the standard `number` type can safely represent.
*   **Type Annotation:** When explicitly annotating a primitive type, use the lowercase type name (e.g., `let age: number = 30;`).

### Object Wrappers (Brief Note)

JavaScript also has built-in "object wrapper" classes corresponding to primitives (e.g., `String`, `Number`, `Boolean`). These are actual classes that provide methods and support object-oriented paradigms (e.g., `"hello".toUpperCase()`). While related to primitives, they are distinct and less commonly used for basic type annotation than the lowercase primitive type names.

### Variable Declaration and Type Annotation

*   **Declaration Keywords:** Use `let` for variables whose value might change, and `const` for variables that are assigned once and should not be reassigned.
*   **Explicit Type Annotation:** You can explicitly tell TypeScript what type a variable should hold using a colon (`:`) followed by the type name after the variable name during declaration.
    ```typescript
    let quantity: number = 100; // variable 'quantity' is explicitly typed as 'number'
    const appName: string = "MyWebApp"; // constant 'appName' is explicitly typed as 'string'
    ```

### Type Inference

*   **Automatic Deduction:** TypeScript is smart enough to often automatically deduce (infer) the type of a variable even if you don't provide an explicit type annotation.
*   **When it Happens:** Inference commonly occurs when:
    *   A variable is initialized with a value (`let x = 10;` -> `x` is inferred as `number`).
    *   An object member is initialized (`class Car { model = "Sedan"; }` -> `model` is inferred as `string`).
    *   A function parameter has a default value (`function greet(name = "Guest") { ... }` -> `name` is inferred as `string`).
    *   Based on the return value of a function (often inferred, though explicit is good practice).
*   **Fixed Type:** Once TypeScript infers a variable's type, that type is fixed for the variable's scope. You cannot later assign a value of a different type without causing a compilation error.
    ```typescript
    let message = "Hello"; // TypeScript infers 'message' as string
    // message = 123; // Error: Type 'number' is not assignable to type 'string'.
    ```

### Function Types and Return Values

*   **Syntax:** You define function parameters and return types using the colon (`:`) syntax.
    ```typescript
    // Example: Function with typed parameters and return type
    function add(a: number, b: number): number {
      // 'a' is typed as number, 'b' is typed as number
      // The function is specified to return a number
      return a + b;
    }

    // Example: Function with optional type inference for return (explicit is better)
    function subtract(x: number, y: number) {
        return x - y; // TypeScript infers return type is number
    }
    ```
*   **Parameter Typing:** Specify the type after the parameter name (e.g., `a: number`).
*   **Return Type Annotation:** Specify the return type after the parameter list's closing parenthesis, just before the function body (e.g., `): number { ... }`). Explicitly annotating the return type is recommended for clarity, especially for complex functions or when implementing interfaces.
*   `void` Type: Use the `void` type as the return type annotation to indicate that a function does **not** return any value (or returns `undefined` implicitly).
    ```typescript
    function logMessage(msg: string): void {
      console.log(msg);
      // No return statement, or returns undefined.
    }
    ```

### Literal Types

*   **Definition:** A literal type defines a set that contains **only one specific, exact primitive value** (e.g., the number `5`, the string `"hello"`, the boolean `true`).
*   **Union Types with Literals:** Literal types are often combined using **Union Types (`|`)** to define a variable that can only hold one of a specific *set* of allowed literal values.
    *   **Example:**
        ```typescript
        type Status = "pending" | "processing" | "completed" | "failed"; // Define a custom type 'Status'

        let orderStatus: Status = "processing"; // Valid assignment
        // orderStatus = "cancelled"; // Error: Type '"cancelled"' is not assignable to type 'Status'.
        ```

### Enums (Enumerations)

*   **Definition:** Enums provide a way to define a set of named constants. They are conceptually similar to using a union of literal types but have different underlying behavior.
*   **Compilation:** Unlike literal types (which are purely TypeScript constructs erased during compilation), **Enums are actually compiled into JavaScript code**. By default, this compiled code is an object mapping the enum member names to numeric values (or string values if initialized with strings).
*   **Runtime Use:** Because Enums exist in the compiled JavaScript output, they can be used at runtime (e.g., in `if` statements, loops, property lookups).

```typescript
// Example: Numeric Enum
enum Direction {
  Up,    // Automatically assigned value 0
  Down,  // Automatically assigned value 1
  Left,  // Automatically assigned value 2
  Right  // Automatically assigned value 3
}

let playerDirection: Direction = Direction.Up; // playerDirection holds the value 0

// Example: String Enum (more common for readability)
enum UserRole {
    Admin = "ADMIN",
    Editor = "EDITOR",
    Viewer = "VIEWER"
}

let currentUserRole: UserRole = UserRole.Editor; // currentUserRole holds the string "EDITOR"
```

### Generics

*   **Purpose:** Generics allow you to write code (like functions or classes) that can work with different data types without losing type safety. They enable the creation of reusable components that are type-agnostic.
*   **Parameterized Types:** You define generic types using angle brackets (`<>`) with type variables (commonly `T`, `U`, `K`, `V`). These variables act as placeholders for actual types that will be specified when the generic code is used.
*   **Example:** A function that returns the first element of an array of *any* type.
    ```typescript
    function getFirstElement<T>(arr: T[]): T | undefined {
      // <T> declares T as a type variable
      // arr: T[] means arr is an array of type T
      // : T | undefined means the function returns either type T or undefined
      return arr.length > 0 ? arr[0] : undefined;
    }

    let firstNum = getFirstElement([1, 2, 3]); // T is inferred as number, firstNum is number | undefined
    let firstStr = getFirstElement(["a", "b"]); // T is inferred as string, firstStr is string | undefined
    // let firstError = getFirstElement<number>(["a", "b"]); // Error: Type 'string' is not assignable to type 'number'.
    ```
*   **Project Use:** While a fundamental TS feature, extensive use of complex generics might not be required for the core parts of this specific project, but they are important to understand.

### Arrays and Tuples

*   **Arrays:** Represent ordered lists of elements. TypeScript allows you to specify the type of elements the array should contain.
    *   **Syntax:** `ElementType[]` (e.g., `number[]`, `string[]`) or `Array<ElementType>` (e.g., `Array<number>`, `Array<string>` - less common for simple arrays).
    *   **Mixed Types:** An array can contain elements of multiple types using a union type: `(string | number)[]` (an array that can contain strings or numbers).
*   **Tuples:** Represent fixed-size arrays where each element at a specific index has a specific, predefined type. The number of elements and their types at each position are strictly defined.
    *   **Syntax:** Defined using square brackets containing the sequence of types: `[type1, type2, type3, ...]`.
    *   **Example:**
        ```typescript
        // Define a tuple type for employee data: ID (number) and Name (string)
        let employee: [number, string];

        employee = [101, "Alice"]; // Valid assignment (number at index 0, string at index 1)
        // employee = ["Bob", 102]; // Error: Type '[string, number]' is not assignable to type '[number, string]'.
        // employee = [201, "Charlie", true]; // Error: Type '[number, string, boolean]' is not assignable to type '[number, string]' (wrong size).
        ```

### Union and Intersection Types

*   **Union Types (`|`):** Allow a variable to hold a value that belongs to *any one* of the specified types. We saw this with literal types, but it works with any types.
    *   Example: `let id: number | string;` (`id` can be a number or a string).
    *   Example: `let result: Success | Error;` (`result` can be an object of type `Success` or an object of type `Error`).
*   **Intersection Types (`&`):** Allow combining multiple types into one. A variable of an intersection type must have *all* the properties and characteristics of *all* the types being intersected.
    *   Example: `type Combined = TypeA & TypeB;` (An object of type `Combined` must have all properties defined in `TypeA` AND all properties defined in `TypeB`). Useful for merging object shapes or adding capabilities from mixins.

### `any` Type

*   **Purpose:** The `any` type is a powerful escape hatch in TypeScript. Variables typed as `any` effectively **disable type checking and compiler checks** for that variable and any properties accessed on it.
*   **Use Cases:**
    *   When migrating a JavaScript project to TypeScript and you haven't yet defined types for certain parts.
    *   When dealing with data from external sources (like APIs or user input) where the exact type or structure might be unknown or highly variable.
    *   In arrays where you intentionally want to mix different types without a strict tuple definition (though union types are often safer: `(string | number | boolean)[]`).
*   **Caution:** Using `any` bypasses TypeScript's safety guarantees. Use it sparingly and try to narrow down the type later if possible.

### Decorators

*   **Definition:** Decorators are a special type of declaration in TypeScript that can be attached to classes, methods, properties, or parameters. They provide a way to add metadata or modify the behavior of the decorated element.
*   **Execution:** Decorators are functions that are executed at runtime with information about the element they are decorating.
*   **Framework Use:** Decorators are commonly used in frameworks and libraries (like Angular, TypeORM, NestJS) to reduce boilerplate and add configuration metadata.
*   **ORM Use:** In TypeORM, decorators like `@Entity()`, `@Column()`, `@PrimaryColumn()`, `@OneToMany()` are used extensively to define the mapping between your TypeScript classes and the database schema.
*   **Further Details:** While you'll use them, the detailed mechanics of *writing* decorators are beyond the scope here, but understanding *how to use* framework-provided decorators is key.

---

## Object Oriented Programming (OOP) in TypeScript

*   **Full Support:** TypeScript fully supports class-based object-oriented programming concepts similar to languages like Java or C#.
*   **Classes and Interfaces:** It allows you to define classes (blueprints for objects) and interfaces (contracts for object shapes or function signatures).
*   **Enhancements:** TypeScript adds static typing, access modifiers, and stricter rules to JavaScript's prototype-based OOP, making it safer and more aligned with class-based OOP paradigms.

### Classes

*   **Definition:** Classes define the structure of objects, including their data (properties/variables) and behavior (methods/functions).
*   **Modifiers:** TypeScript adds explicit access modifiers to control the visibility and accessibility of class members (properties, methods, constructors):
    *   `public`: (Default) Accessible from anywhere.
    *   `private`: Accessible only within the class itself.
    *   `protected`: Accessible within the class and by instances of derived (child) classes.
*   **Constructors:** A special method within a class that is called when a new instance of the class is created using the `new` keyword.
    *   **Default Constructor:** If you don't define a constructor, TypeScript provides a default empty one.
    *   **Custom Constructor:** Defining your own constructor replaces the default. You can specify parameters to initialize the instance's properties.
    *   **Constructor Shorthand:** TypeScript offers a concise syntax for initializing class properties directly from constructor parameters: Include an access modifier (`public`, `private`, or `protected`) directly on a constructor parameter. This automatically creates a class property with the same name and modifier and assigns the parameter's value to it.
        ```typescript
        class Person {
          // Longhand:
          // public name: string; // Define the property
          // private age: number;
          // constructor(name: string, age: number) {
          //   this.name = name; // Initialize the property
          //   this.age = age;
          // }

          // Shorthand:
          constructor(public name: string, private age: number) {
            // Properties 'name' (public) and 'age' (private) are automatically created and initialized.
            // No need to declare 'name' and 'age' explicitly outside the constructor or assign them inside.
          }

          greet(): void {
            console.log(`Hello, my name is ${this.name}.`);
            // console.log(`I am ${this.age} years old.`); // Can access private 'age' inside the class
          }
        }
        const person1 = new Person("Alice", 30);
        person1.greet();
        // console.log(person1.name); // Access public property OK
        // console.log(person1.age); // Error: Property 'age' is private.
        ```
*   **Static Members:** Members marked with the `static` keyword belong to the class itself, **not** to any specific instance of the class.
    *   **Use Cases:** Useful for utility functions related to the class concept but not requiring instance data (e.g., a `Math.random()` is a static method), or for properties that hold configuration or shared data for all instances.
    *   **Access:** Accessed using the class name followed by a dot (`ClassName.staticMember`).
        ```typescript
        class AppConfig {
          static appVersion = "1.0.0"; // Static property

          static displayVersion(): void { // Static method
            console.log(`App Version: ${AppConfig.appVersion}`); // Access static property using class name
          }
        }
        // Access static members using the class name
        console.log(AppConfig.appVersion); // Output: 1.0.0
        AppConfig.displayVersion(); // Output: App Version: 1.0.0

        // let config = new AppConfig();
        // console.log(config.appVersion); // Error: Property 'appVersion' is static.
        ```

### Interfaces

*   **Definition:** Interfaces in TypeScript are a powerful way to define the **structure** or **shape** that objects or functions should adhere to. They describe what properties and methods an object *must* have, along with their types. However, they **do not define the implementation or values**.
*   **Purpose:** Primarily used for type checking (compile-time). They act as contracts.
*   **Syntax:** Defined using the `interface` keyword.
    ```typescript
    // Example: Interface defining the structure of a user object
    interface User {
      id: number;         // Required property: must be a number
      username: string;   // Required property: must be a string
      email?: string;     // Optional property: may or may not be a string (marked with '?')
      isActive: boolean;
      vatCode?: string; // Optional string property
      readonly ssn: string; // Readonly string property (cannot be changed after initialization)
    }

    const user1: User = { id: 1, username: "alice", email: "alice@example.com", isActive: true, ssn: "ABC123" }; // Valid object matching interface
    // user1.ssn = "XYZ"; // Error: Cannot assign to 'ssn' because it is a read-only property.

    // const user2: User = { id: 2, username: "bob", isActive: false }; // Valid - email and vatCode are optional
    // const user3: User = { username: "charlie" }; // Error: Missing required properties 'id', 'isActive', 'ssn'.
    ```
*   **Usage as Types:** Interfaces can be used directly as types for variables, function parameters, or return values.
*   **Optional Properties (`?`):** Adding a question mark (`?`) after a property name in an interface definition indicates that this property is optional. Objects implementing this interface are allowed to omit this property.
*   **Readonly Properties (`readonly`):** Marking a property with `readonly` means that once the object is initialized, the value of this property cannot be reassigned.
*   **Function Types in Interfaces:** Interfaces can also define the signature (parameter types and return type) for function types.
    ```typescript
    // Example: Interface defining the shape of a function
    interface StringProcessor {
      // Defines a function that takes a string, a number, and returns a string
      (input: string, config: number): string;
    }

    // A function variable typed using the interface
    let process: StringProcessor;

    process = (text: string, options: number): string => {
      // Implementation must match the signature defined in the interface
      return text.substring(0, options);
    };

    console.log(process("Hello World", 5)); // Output: Hello
    ```
*   Interfaces can also define types for arrays (e.g., `interface StringArray { [index: number]: string; }`) and dictionary-like objects with dynamic string keys (e.g., `interface StringDictionary { [key: string]: string; }`).

### Optional Parameters (in Functions)

*   **Syntax:** Similar to optional properties in interfaces, adding a question mark (`?`) after a function parameter name makes that parameter optional.
    ```typescript
    function greet(name: string, greeting?: string): void {
      // The 'greeting' parameter is optional
      // Inside the function, 'greeting' will be 'undefined' if not provided
      const finalGreeting = greeting === undefined ? "Hello" : greeting; // Explicit check
      // Or using nullish coalescing operator (ES2020):
      // const finalGreeting = greeting ?? "Hello";

      console.log(`${finalGreeting}, ${name}!`);
    }

    greet("Alice");        // Output: Hello, Alice! (greeting is undefined)
    greet("Bob", "Hi");    // Output: Hi, Bob! (greeting is "Hi")
    ```
*   **Handling `undefined`:** When accessing an optional parameter inside a function, remember that it might be `undefined`. Use explicit checks (`=== undefined`), the logical OR operator (`||` - but be careful with falsy values like `0` or `''`), or the nullish coalescing operator (`??`) to provide a fallback value if the parameter was not provided.

### Function Overloading (Brief Mention)

*   **Concept:** TypeScript allows defining multiple function signatures (different parameter lists and return types) for the same function name. This is called function overloading.
*   **Implementation:** This is primarily a **compile-time** feature for providing better type checking and IntelliSense. At runtime, there is still only **one actual JavaScript function implementation**.
*   **Requirement:** The single JavaScript implementation must be written to handle all the defined overload cases, typically using union types for parameters and checking the types or number of arguments received inside the function body.

### Polymorphism

*   **Concept:** Polymorphism ("many forms") is an OOP principle where objects of different types can be treated as if they are of a common type, and an operation on that common type behaves differently depending on the underlying specific type of the object.
*   **TypeScript Support:** TypeScript supports polymorphism through both **inheritance** (treating instances of derived classes as instances of a base class) and **interfaces** (treating any object that implements an interface as an instance of that interface type).
*   **Structural Polymorphism:** Due to TypeScript's structural typing, it also supports structural polymorphism. If two objects have compatible structures (e.g., both have a method named `draw()` with the same signature), you can treat them interchangeably based on that shared structure, even if they don't share a base class or explicitly implement a common interface.

### Compatibility with Existing JavaScript Libraries

*   **Seamless Integration:** TypeScript is designed to be highly compatible with existing JavaScript code and libraries. You can use any JavaScript library in a TypeScript project.
*   **Declaration Files (`.d.ts`):** To get the benefits of static type checking and IntelliSense for JavaScript libraries that were not written in TypeScript, you use **declaration files**. These files have the `.d.ts` extension and contain only type definitions (interfaces, function signatures, variable types) describing the library's API, without any implementation code.
*   **Getting Declaration Files:** Many popular JavaScript libraries now include `.d.ts` files in their npm packages. For many older libraries, community-maintained declaration files are available from the `@types` organization on npm (e.g., `npm install @types/react`).

---

## Promises

*   **Purpose:** Promises are a fundamental pattern in modern JavaScript (and TypeScript) for handling the **eventual result of an asynchronous operation**. Asynchronous operations (like making a network request, reading a file, or accessing a database) don't finish immediately; they start now and complete later.
*   **Concept:** A Promise is an object that represents the current state of an asynchronous operation that is in progress or has already completed.
*   **Promise States:** A Promise can be in one of three mutually exclusive states:
    *   **Pending:** The initial state; the asynchronous operation is still running.
    *   **Fulfilled:** The operation completed successfully. The Promise holds a resulting **value**.
    *   **Rejected:** The operation failed. The Promise holds a **reason** for the failure (typically an `Error` object).
*   **Handling Results:** Traditionally, you use the `.then()` method to specify a callback function to execute when the Promise is Fulfilled (receiving the value), and the `.catch()` method to specify a callback function to execute when the Promise is Rejected (receiving the reason).
*   **"Callback Hell":** Chaining multiple asynchronous operations that depend on each other using nested `.then()` callbacks can lead to deeply indented and difficult-to-read code, known as "callback hell".

### Async/Await

*   **Purpose:** `async` and `await` syntax (introduced in ES2017) provide a more concise and readable way to write asynchronous code based on Promises. They make asynchronous code look and behave more like synchronous code.
*   `async` Keyword: You add the `async` keyword before a function declaration (`async function myFunction() { ... }`) or expression (`const myFunction = async () => { ... };`).
    *   An `async` function always implicitly returns a **Promise**.
    *   If the function returns a value directly, the Promise returned by the `async` function will be fulfilled with that value.
    *   If the function throws an error directly, the Promise returned by the `async` function will be rejected with that error.
    *   You can use the `await` keyword *only* inside an `async` function (or in the top-level of an ES Module).
*   `await` Keyword: You use `await` before a call to a function that returns a Promise (`let result = await somePromiseFunction();`).
    *   `await` **pauses** the execution of the `async` function until the Promise on its right-hand side settles (either fulfills or rejects).
    *   If the Promise **fulfills**, `await` "unwraps" the Promise and returns its fulfillment value. The `async` function then resumes execution from that point.
    *   If the Promise **rejects**, `await` throws the rejection reason as an error. This error can be caught using a standard `try...catch` block.
*   **TypeScript and Async Functions:** TypeScript correctly infers that an `async` function that returns a value of type `T` actually returns a `Promise<T>`. Explicitly annotating the return type as `Promise<T>` is good practice for clarity and enforcing the function's contract.
*   **Parallel vs. Sequential:**
    *   Using `await` inside a loop processes items **sequentially** (waits for one to finish before starting the next).
    *   To process multiple independent Promises **concurrently**, create all the Promises first and then use `Promise.all(arrayOfPromises)` to wait for all of them. Combine with `Array.prototype.map()` to easily create an array of Promises from an array of data.

### Async/Await Examples

```typescript
// Example: Async function returning a Promise
async function fetchData(id: number): Promise<string> {
  // Simulate an async operation (e.g., API call, DB query)
  return new Promise(resolve => {
    setTimeout(() => {
      resolve(`Data for ID ${id}`);
    }, 1000); // Resolve after 1 second
  });
}

// Example: Using await inside another async function
async function processData(userId: number): Promise<void> { // Function returns Promise<void>
  console.log("Starting data process...");
  try {
    // Pause execution here until fetchData(userId) Promise resolves
    const data = await fetchData(userId); // 'data' will be the resolved string
    console.log("Data received:", data); // This line runs AFTER fetchData resolves

    // You can await multiple times for sequential operations
    const moreData = await fetchData(userId + 1);
    console.log("More data received:", moreData); // This line runs AFTER the second fetchData resolves

  } catch (error) {
    // If any awaited Promise in the 'try' block rejects, execution jumps here
    console.error("An error occurred:", error);
    // Handle the error
  } finally {
    console.log("Data process finished."); // This always runs
  }
}

processData(1); // Call the async function
console.log("ProcessData function called (This prints immediately).");
```

### Error Handling with Async/Await

*   **Simpler Syntax:** Error handling with `async`/`await` is significantly cleaner than using `.catch()` on every link in a Promise chain.
*   `try...catch` Blocks: You use standard synchronous `try...catch` blocks.
    *   Place the `await` calls that might fail inside the `try` block.
    *   If an awaited Promise rejects, the rejection reason is thrown as an error, and execution jumps directly to the `catch` block.
    *   Synchronous errors thrown within the `try` block are also caught by the `catch` block.
*   **Catching Specific Errors:** If your Promises reject with custom error objects/classes, you can check the type of the caught error in the `catch` block using `instanceof` or TypeScript type guards to handle different error types specifically.
*   **Throwing Errors:**
    *   It is highly suggested to always `throw` **objects** (especially `Error` objects or instances of classes that extend `Error`), rather than throwing simple strings or numbers.
    *   Define **custom error classes** (e.g., `NotFoundError`, `ValidationError`, `UnauthorizedError`) by extending the built-in JavaScript `Error` class. This allows you to create specific types of errors that can be easily identified and caught later in your code.
    *   `throw new CustomError("Details about the error");`

```typescript
// Example: Define custom error classes
class NotFoundError extends Error {
  constructor(message = "Resource not found") {
    super(message);
    this.name = "NotFoundError"; // Set the name property for identification
  }
}

class UnauthorizedError extends Error {
    constructor(message = "Unauthorized access") {
        super(message);
        this.name = "UnauthorizedError";
    }
}


// Example: Using try...catch with custom errors
async function getUserData(userId: number): Promise<any> {
  // Simulate an async operation that might fail or return special results
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (userId === 1) {
        resolve({ id: 1, name: "Alice" }); // Success
      } else if (userId === 404) {
        reject(new NotFoundError(`User with ID ${userId} not found`)); // Reject with custom error
      } else if (userId === 401) {
          reject(new UnauthorizedError(`Access denied for user ID ${userId}`)); // Reject with another custom error
      } else {
        reject(new Error(`Failed to fetch data for ID ${userId}`)); // Reject with generic error
      }
    }, 500);
  });
}

async function displayUserProfile(id: number): Promise<void> {
  try {
    console.log(`Fetching user ${id}...`);
    // Await the promise - it might reject with different error types
    const user = await getUserData(id);
    console.log("Successfully loaded user:", user.name);

  } catch (error: unknown) { // Use 'unknown' or 'any' for the catch variable type initially
    console.error(`Failed to load user ${id}.`);

    // Check the type of the error using instanceof or type guards
    if (error instanceof NotFoundError) {
      console.error("Reason: User was not found.");
      // Display a 404 message to the user
    } else if (error instanceof UnauthorizedError) {
        console.error("Reason: Access denied.");
        // Redirect to login or show permission error
    }
    else if (error instanceof Error) { // Catch any other standard Error object
      console.error("Reason:", error.message);
      // Display a generic error message
    } else {
        console.error("An unknown error occurred.");
    }

  } finally {
    console.log(`Finished processing for user ${id}.\n`);
  }
}

displayUserProfile(1);   // Success
displayUserProfile(404); // Not Found Error
displayUserProfile(401); // Unauthorized Error
displayUserProfile(99);  // Generic Error
```