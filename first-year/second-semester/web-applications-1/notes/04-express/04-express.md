# Express.js

## 🌐 HTTP Protocol Basics

### What is HTTP?

**HTTP** (HyperText Transfer Protocol) serves as the fundamental protocol for exchanging information across the World Wide Web. It operates based on a **client-server request-response model**: a Client initiates communication by sending a **request** to a Server, which then processes the request and sends back a **response**.

### Key Components of HTTP Messages

Both HTTP Request (Client -> Server) and HTTP Response (Server -> Client) messages adhere to a similar basic structure: an initial line, followed by headers, and optionally, a body.

1.  **HTTP Request:** This message asks the server to perform a specific action or provide a resource.
    *   **Initial Line (Request Line):** Specifies the `METHOD`, the `PATH` of the resource, and the `HTTP_VERSION` being used (e.g., `GET /users HTTP/1.1`).
    *   **Headers:** These are key-value pairs providing metadata about the request, the client, and the desired response characteristics (e.g., `Host: example.com`, `User-Agent`, `Accept`, `Authorization`, `Content-Type`).
    *   **Body** (Optional): Contains data being sent *to* the server, commonly used with methods like POST, PUT, or PATCH. Examples include form data or a JSON payload.

2.  **HTTP Response:** This is the server's answer to a request.
    *   **Status Line:** Indicates the outcome of the request, including the `HTTP_VERSION`, a numeric `STATUS_CODE`, and a brief `STATUS_MESSAGE` (e.g., `HTTP/1.1 200 OK`). The Status Code is crucial for understanding the result (e.g., 200 indicates Success, 404 means Not Found, 500 signifies a Server Error).
    *   **Headers:** Provide metadata about the response itself and the server (e.g., `Content-Type`, `Content-Length`, `Cache-Control`, `Set-Cookie`).
    *   **Body** (Optional): Contains the actual content being sent back, such as HTML, JSON data, or image files. This is typically included in successful GET responses.

### HTTP Methods

HTTP methods define the intended action to be performed on a resource identified by its URI.

| Method    | Description                                     | Properties                                                                             | Common Use Case                                                      |
| :-------- | :---------------------------------------------- | :------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| **GET**   | Requests a representation of the specified resource (retrieve data). | Safe (no state change), Idempotent (repeating has same effect), Cacheable.             | Fetching data (e.g., getting a web page, retrieving API data).       |
| **POST**  | Submits data to be processed to a specified resource. | Not Safe (usually changes state), Generally Non-idempotent (repeating might create duplicates). | Creating a new resource, submitting a form.                          |
| **PUT**   | Updates or replaces the target resource with the request payload. | Not Safe, Idempotent.                                                                  | Performing a complete update of an existing resource at a known URI. |
| **DELETE**| Deletes the specified resource.                 | Not Safe, Idempotent.                                                                  | Removing a resource.                                                 |
| **HEAD**  | Identical to GET, but the server MUST NOT return a response body; only headers. | Safe, Idempotent, Cacheable (metadata).                                                | Checking resource existence, retrieving metadata without downloading content. |
| **OPTIONS**| Describes the communication options (e.g., supported methods, headers) for the target resource or server. | Safe, Idempotent.                                                                      | Used by browsers for CORS "pre-flight" requests to check permissions. |
| **PATCH** | Applies partial modifications to a resource.    | Not Safe, Can be Idempotent depending on implementation, often Non-idempotent.         | Performing a partial update of a resource.                           |

Understanding key properties:
*   **Idempotent:** A request is idempotent if making the same request multiple times has the same overall effect on the server's state as making it just once (e.g., GET, PUT, DELETE).
*   **Safe:** A request is safe if it does not change the state of the server (e.g., GET, HEAD).

### HTTP Status Codes

These are 3-digit codes returned in the Status Line of an HTTP Response, indicating the outcome of the request. The first digit defines the category:
*   `1xx` Informational: The request was received and is continuing process.
*   `2xx` Success: The request was successfully received, understood, and accepted (e.g., `200 OK`, `201 Created`, `204 No Content`).
*   `3xx` Redirection: Further action needs to be taken by the client to complete the request (e.g., `301 Moved Permanently`, `302 Found`).
*   `4xx` Client Error: The request could not be fulfilled or was invalid due to an issue on the client's part (e.g., `400 Bad Request`, `401 Unauthorized`, `403 Forbidden`, `404 Not Found`, `405 Method Not Allowed`, `409 Conflict`, `422 Unprocessable Entity`).
*   `5xx` Server Error: The server failed to fulfill a valid request due to an error on the server side (e.g., `500 Internal Server Error`, `503 Service Unavailable`).

### Important Headers

Specific headers provide crucial context and metadata for both requests and responses:

*   `Host` (Mandatory in HTTP/1.1+ Request): Specifies the domain name of the server being requested.
*   `Content-Type` (Both): Indicates the media type of the message body data (e.g., `text/html`, `application/json`, `image/png`).
*   `Content-Length` (Both): Specifies the size of the message body in bytes.
*   `Content-Encoding` (Both): Indicates the type of encoding or compression applied to the body data (e.g., `gzip`).
*   `Authorization` (Request): Carries credentials used to authenticate the client with the server.
*   `Accept` (Request): Tells the server which content types the client is able to process in the response.
*   `Set-Cookie` (Response) / `Cookie` (Request): Used by the server to send a cookie to the client (`Set-Cookie`) and by the client to send stored cookies back to the server in subsequent requests (`Cookie`).

---

## 🛠 Express.js Fundamentals

### What is Express.js?

**Express.js** (often simply "Express") is a minimal and flexible web application framework built for Node.js. It significantly simplifies the process of building web applications and APIs by providing a robust set of features specifically for routing HTTP requests and managing middleware.

### Setup Steps

To start using Express.js:

1.  **Initialize Project:** Navigate to your project directory in the terminal and run `npm init -y` to create a `package.json` file.
2.  **Install Express:** Install the Express package as a project dependency: `npm install express`.
3.  **Install Nodemon (Recommended for Development):** For automatic server restarts during development, install Nodemon globally: `npm install -g nodemon`.

### Running Your Server

Once your application code is written:

*   **Manually:** Execute your main server file using Node.js: `node index.js` (assuming your entry file is `index.js`).
*   **Automatically (with Nodemon):** If you installed Nodemon, run `nodemon index.js`. This will start your server and automatically restart it whenever you save changes to your project files.

### Basic Server Example

Here's a minimal Express server that responds to requests on the root path:

```javascript
import express from 'express'; // Use import with type: module in package.json, or const express = require('express');
const app = express(); // Initialize an Express application instance
const PORT = 3000; // Define the port the server will listen on

// Define a route handler for GET requests to the root path '/'
app.get('/', (req, res) => { // 'req' is the request object, 'res' is the response object
  res.send('Hello World!'); // Send the string 'Hello World!' as the response body and end the request-response cycle
});

// Start the Node.js HTTP server and make it listen on the specified port
app.listen(PORT, () => {
  console.log(`Server ready on port ${PORT}`);
  console.log(`Access at: http://localhost:${PORT}/`);
});
```

In this example:
-   `app = express()`: Creates the main Express application instance.
-   `app.get(PATH, HANDLER)`: Registers a specific `HANDLER` function to respond only to GET requests matching the defined `PATH`.
-   `(req, res)`: These are the standard parameters for Express route and middleware handlers, representing the incoming HTTP request and the outgoing HTTP response respectively.
-   `res.send(...)`: A convenient method on the response object that sends data as the HTTP response body, automatically sets some headers (like `Content-Type` and `Content-Length`), and importantly, terminates the request-response cycle.
-   `app.listen(...)`: This method starts the underlying Node.js HTTP server, binding it to a specific network port to listen for incoming connections.

---

## 🚦 Routing

Routing is how Express determines which part of your application code should handle an incoming HTTP request based on its HTTP method (like GET, POST) and its URI path.

### Route Syntax

You define routes using methods on the `app` instance that correspond to HTTP methods:

`app.METHOD(PATH, HANDLER_FUNCTION1[, HANDLER_FUNCTION2, ...]);`

You can provide one or more handler functions; they execute in sequence.

```javascript
app.get('/', (req, res) => { res.send('Homepage'); }); // Handles GET requests to the root path '/'
app.post('/users', (req, res) => { res.status(201).send('User created'); }); // Handles POST requests to '/users', sends 201 Created status
app.get('/users', (req, res) => { res.json([{id:1, name: 'Alice'}]); }); // Handles GET requests to '/users', sends JSON response
```
`res.json(...)` automatically sets the `Content-Type` header to `application/json`.

### Parametric Paths

Express allows capturing dynamic values from parts of the URL path using **parametric paths**, denoted by a colon followed by a parameter name (e.g., `:paramName`).

```javascript
// This route will match paths like /users/123/books/456
app.get('/users/:userId/books/:bookId', (req, res) => {
  // Access the captured values via the req.params object
  const { userId, bookId } = req.params;
  res.send(`Requested data for User ID: ${userId} and Book ID: ${bookId}`);
});
```
The captured values are available as properties on the `req.params` object, named according to the parameter names defined in the path string.

You can also add basic validation to parameters using regular expressions `/:paramName(regex)`.

```javascript
// This route matches /user/123 but NOT /user/abc because of the regex
app.get('/user/:id([0-9]+)', (req, res) => {
  res.send(`The numeric user ID requested is: ${req.params.id}`);
});
```

---

## 📦 Middleware

### What is Middleware?

**Middleware functions** in Express are functions that have access to the request object (`req`), the response object (`res`), and the next middleware function in the application's request-response cycle (often named `next`). These functions are executed in a pipeline for every incoming request, potentially before the final route handler.

A middleware function can perform various tasks:
*   Execute any code.
*   Make changes to the request (`req`) and the response (`res`) objects.
*   End the request-response cycle by sending a response (e.g., using `res.send()`, `res.json()`, `res.end()`).
*   Call the next middleware function in the stack using `next()`.

It's critical that a middleware function either calls `next()` to pass control to the next function in the pipeline or sends a response to end the cycle. If a middleware does neither, the request will simply hang.

### Using Middleware

You apply middleware to your Express application using the `app.use()` method:

`app.use(middlewareFunction);` or `app.use(path, middlewareFunction);`

Middleware functions are executed in the order they are added with `app.use()`. If a path is specified with `app.use()`, the middleware will only execute for requests whose path begins with that specified path. Without a path, it applies to all requests.

```javascript
// Define a simple request logger middleware
function requestLogger(req, res, next) {
  console.log(`[${new Date().toISOString()}] ${req.method} ${req.url}`);
  next(); // Pass control to the next function (either another middleware or the route handler)
}

// Apply the logger middleware to ALL incoming requests
app.use(requestLogger);

// This route handler will only run *after* the requestLogger middleware has executed
app.get('/', (req, res) => {
  res.send('Home page, request was logged.');
});
```

### Built-in Express Middleware

Express provides several useful built-in middleware functions:

*   `express.static('directory')`: Used to serve static files (HTML, CSS, JavaScript, images, etc.) from a specified directory to clients.
    ```javascript
    app.use(express.static('public')); // Serve files like public/style.css at /style.css
    // Optionally, serve static files under a specific path prefix:
    // app.use('/assets', express.static('public')); // Serve files like public/style.css at /assets/style.css
    ```
*   `express.json()`: Parses incoming requests with JSON payloads (`Content-Type: application/json`). It makes the parsed JSON data available on `req.body`.
*   `express.urlencoded({ extended: true })`: Parses incoming requests with URL-encoded payloads (`Content-Type: application/x-www-form-urlencoded`), common for form submissions. It makes the parsed data available on `req.body`. The `{ extended: true }` option allows for rich objects and arrays to be encoded.

These body-parsing middlewares (`express.json()`, `express.urlencoded()`) should typically be applied using `app.use()` *before* any route handlers that need to access data from the request body via `req.body`.

### Third-Party Middleware

The Express ecosystem benefits from a large number of third-party middleware packages available on npm, providing solutions for logging, security, session management, etc. A popular example is **Morgan**, an HTTP request logger.

To use a third-party middleware:
1.  Install the package: `npm install morgan`.
2.  Import/require it in your application file.
3.  Use it with `app.use()`.

```javascript
import morgan from 'morgan'; // Or: const morgan = require('morgan');

// Use morgan middleware with 'dev' format for concise logging in development
app.use(morgan('dev'));

// This route will trigger the morgan logger when accessed
app.get('/', (req, res) => { res.send('This request was logged by Morgan!'); });
```

---

## 📄 Handling Requests

When a route handler or middleware function is invoked, the incoming request details are accessible via the `req` object.

### Accessing GET Parameters (Query Strings)

Data appended to the URL after a question mark (`?`), in the format `key=value&anotherkey=anothervalue`, is known as the query string. You can access this data via the `req.query` object. The keys and values on `req.query` are always strings.

```javascript
// Example URL: /search?q=node.js&page=2&sort=asc
app.get('/search', (req, res) => {
  // Access query parameters from req.query
  const searchTerm = req.query.q; // "node.js"
  const pageNumber = req.query.page; // "2" (string)
  const sortBy = req.query.sort; // "asc"

  console.log(`Search term: ${searchTerm}, Page: ${pageNumber}, Sort: ${sortBy}`);
  res.send(`Search results for "${searchTerm}"`);
});
```

### Accessing POST/PUT Data (Request Body)

Data sent in the body of an HTTP message (commonly used with POST, PUT, and PATCH methods) is accessed via `req.body`. However, this data is not automatically parsed by Express. As mentioned in the Middleware section, you must use appropriate body-parsing middleware (like `express.json()` for JSON bodies or `express.urlencoded()` for form data) *before* your route handler. Once the middleware has processed the body, the parsed data will be available on the `req.body` object.

```javascript
// Assuming app.use(express.json()) has been configured earlier
// Example: Handling a POST request to /api/users with a JSON body like { "name": "Alice", "age": 30 }
app.post('/api/users', (req, res) => {
  // Access the parsed body data from req.body (thanks to express.json() middleware)
  const { name, age } = req.body;

  // Process the received data (e.g., save to database)
  console.log(`Received user data: Name=${name}, Age=${age}`);

  // Send a response indicating success
  res.status(201).json({ message: 'User data received and processed' }); // Sending a JSON response
});
```

---

## 🛡 Validation

Validating and sanitizing incoming user input is a critical aspect of building secure and reliable applications. Data from requests should never be trusted implicitly. Libraries like **`express-validator`** provide a robust way to define and enforce validation and sanitization rules as middleware.

To use `express-validator`:
1.  Install it: `npm install express-validator`.
2.  Import necessary functions: `check`, `validationResult`.

### Example: Using `express-validator` for Input Validation and Sanitization

Validation rules are defined as middleware functions, typically using `check('fieldName').validator().sanitizer()...`. These validation middleware functions are placed in the route definition *before* the final handler function. Inside the final handler, you check the results of the validation using `validationResult(req)`. If there are errors, you typically send an error response (e.g., with a `422 Unprocessable Entity` status). If validation passes, you can safely use the data from `req.body` or `req.query` (which might have been modified by sanitization rules).

```javascript
import { check, validationResult } from 'express-validator'; // Or: const { check, validationResult } = require('express-validator');

// Define an array of validation and sanitization rules as middleware
const userRegistrationValidationRules = [
  // Check 'email' field: remove leading/trailing whitespace, must be an email, add custom message, normalize email format
  check('email').trim().isEmail().withMessage('A valid email address is required.').normalizeEmail(),
  // Check 'password' field: must be at least 6 characters, must contain a number, with custom messages
  check('password').isLength({ min: 6 }).withMessage('Password must be at least 6 characters long.')
    .matches(/\d/).withMessage('Password must contain at least one number.'),
  // Add more rules for other fields if needed...
];

// Define a POST route handler for registration, including the validation middleware
app.post('/register', userRegistrationValidationRules, (req, res) => {
  // Check if any validation errors occurred based on the rules defined above
  const errors = validationResult(req);

  if (!errors.isEmpty()) {
    // If there are validation errors, send a 422 status code and the array of error details
    return res.status(422).json({ validationErrors: errors.array() });
  }

  // If execution reaches this point, all validation rules passed.
  // The data in req.body (or req.query) is now validated and potentially sanitized.
  const { email, password } = req.body; // Access the data

  console.log("User registered (validation successful):", email);
  // Proceed with processing the valid data (e.g., creating a user in the database)
  res.status(201).json({ message: 'User registration successful.' });
});
```

Key components from `express-validator` used here:
-   `check('field')`: Initiates the validation chain for a specified field name in `req.body`, `req.query`, or `req.params`.
-   Chainable methods: Various built-in validators (`isEmail`, `isLength`, `matches`, etc.) and sanitizers (`trim`, `escape`, `normalizeEmail`). You chain them together to apply multiple rules to a single field.
-   `.withMessage()`: Allows providing a custom error message that will be included in the validation result if the preceding validator fails.
-   `validationResult(req)`: A function that collects the results of all validation rules that have run for the current request (`req`).
-   `errors.isEmpty()`: A method on the result object to quickly check if any validation errors occurred.
-   `errors.array()`: A method to get the validation errors as a simple array of objects.
-   `return res.status(422).json(...)`: Crucially, after checking for errors, you must explicitly stop the request processing and send a response if errors are found, preventing the rest of the handler code from executing with invalid data. The validated and potentially sanitized data is then available in `req.body`, `req.query`, etc., for use *after* the error check has passed.