# 🌐 HTTP APIs

---

## 🤝 APIs as Client-Server Glue

**APIs (Application Programming Interfaces)**, particularly those based on HTTP, serve as the vital connection layer between disparate software systems. They act as the "glue" between Clients (such as web browsers, mobile applications, or other servers) and Servers that host specific logic or data. APIs define the rules, specifications, and data formats that govern how these systems can communicate in a structured manner, typically following a request-response model. Among various architectural styles for web APIs, **REST (Representational State Transfer)** is a widely adopted approach. RESTful APIs leverage standard HTTP methods, use **URIs** to identify resources, and commonly employ **JSON** for exchanging data representations.

---

## 📦 JSON (JavaScript Object Notation)

**JSON** is a popular, lightweight data interchange format. It is designed to be easily readable by humans while remaining simple for machines to parse and generate. JSON is based on a subset of JavaScript's object literal syntax, making it naturally compatible with JavaScript environments. The standard media type for JSON data transmitted over HTTP is `application/json`.

### JSON Structure

JSON data is constructed using a combination of primitive and composite data types:

*   **Primitive Types:** These include Strings (enclosed in double quotes, e.g., `"hello"`), Numbers (integers or floating-point, e.g., `123`, `3.14`), Booleans (`true` or `false`), and Null (`null`).
*   **Composite Types:**
    *   **Arrays:** Ordered collections of values, enclosed in square brackets `[]` (e.g., `[1, "apple", {}]`). Values within an array can be of any valid JSON type.
    *   **Objects:** Unordered collections of **string key**-value pairs, enclosed in curly braces `{}` (e.g., `{ "name": "Alice", "age": 30, "isStudent": false }`). Importantly, keys within a JSON object **must** be strings, and they must be enclosed in double quotes. Values can be any valid JSON type (primitive or composite).

### Using JSON in JavaScript

JavaScript provides native, built-in support for working with JSON through the global `JSON` object:

*   `JSON.stringify(jsValue)`: This method takes a JavaScript value (such as an object, array, or primitive) and converts it into a JSON formatted string.
*   `JSON.parse(jsonString)`: This method takes a JSON formatted string and parses it back into a corresponding JavaScript object or value.

```javascript
const jsObj = { name: "API", version: 1, enabled: true };
// Convert JS object to JSON string
const jsonStr = JSON.stringify(jsObj); // Result: '{"name":"API","version":1,"enabled":true}'

// Parse JSON string back to JS object
const parsedObj = JSON.parse(jsonStr); // Result: { name: 'API', version: 1, enabled: true }

console.log(typeof jsonStr); // Output: string
console.log(typeof parsedObj); // Output: object
```

---

## 🔗 URI Design

Well-designed **URIs** (Uniform Resource Identifiers, often URLs) are crucial for discoverability and clarity in HTTP APIs, especially following the **REST** architectural style. A core RESTful principle is that URIs should represent **nouns** (the resources), while the **HTTP method** indicates the action to be performed on that resource.

### Types of Resource URIs

URIs typically identify two main types of resources:

*   **Collection URI:** Represents a group or collection of resources (e.g., `/users`, `/products`).
*   **Element URI:** Represents a single, specific resource within a collection, usually identified by its unique ID (e.g., `/users/123`, `/products/abc`).

### Best Practices for URI Naming

Adhering to consistent naming conventions improves API usability:

*   Use **Nouns** to name resources (e.g., `/photos`, not `/getPhotos`).
*   Use **Plural Nouns** for collection resources (e.g., `/users` for the list of users, not `/user`).
*   Choose specific, descriptive names for resources (e.g., `/courses`, which is better than a generic `/items`).
*   Maintain consistent casing, commonly using lowercase for all URI components.
*   Use **hyphens** (`-`) as separators in multi-word resource names for readability (e.g., `/user-accounts`, not `/user_accounts` or `/useraccounts`).

---

## 🛠 CRUD Operations via HTTP Methods

The fundamental data operations known as **CRUD** (Create, Read, Update, Delete) map cleanly and logically to standard HTTP methods in **RESTful** API design.

### Standard CRUD-HTTP Mapping

| Resource           | CREATE (Add New)       | RETRIEVE (Read/Fetch)  | UPDATE (Modify Existing)                              | DELETE (Remove)       |
| :----------------- | :--------------------- | :--------------------- | :---------------------------------------------------- | :-------------------- |
| **Collection**     | `POST /collection`     | `GET /collection`      | `PUT /collection` *(Typically for bulk updates)*      | `DELETE /collection` *(Use with caution, potentially deletes all or mass delete)* |
| **Single Element** | *(Handled by POST to collection)* | `GET /collection/{id}` | `PUT /collection/{id}` (Full Replacement)<br>`PATCH /collection/{id}` (Partial Update) | `DELETE /collection/{id}` |

### Examples using a `/dogs` Resource

Let's illustrate the mapping with a hypothetical `/dogs` resource:

*   **CREATE:** To add a new dog, send a `POST` request to the collection URI: `POST /dogs`. The new dog's data is included in the request body. The server typically responds with `201 Created` and potentially the new resource's URI in the `Location` header, or the full created object.
*   **READ:** To retrieve a list of all dogs, send `GET /dogs`. The response body would be an array of dog representations. To get a specific dog by its ID (e.g., ID 123), send `GET /dogs/123`. The response body would be the representation of that single dog.
*   **UPDATE:** To replace the entire representation of dog 123 with new data, send `PUT /dogs/123` with the complete new data in the body. To apply only partial modifications (e.g., change only the dog's name), send `PATCH /dogs/123` with just the partial data in the body.
*   **DELETE:** To remove dog 123, send `DELETE /dogs/123`. The server should delete the resource and typically respond with `204 No Content` on success, indicating no body is returned but the action was successful.

---

## 🔄 Representing Relationships

**RESTful** APIs often use **nested URIs** to represent relationships between resources, particularly when one resource or collection is naturally contained within or strongly related to another.

Examples of nested resource URIs:

*   To retrieve all courses associated with a specific student: `GET /students/s123/courses`.
*   To get the author of a specific book: `GET /books/b543/author`.
*   To list all comments for a specific post: `GET /posts/p789/comments`.

---

## 🔍 Advanced Queries

For performing more complex queries on collection resources—such as filtering the results, sorting them, or paginating through large sets—**query parameters** are used. These are appended to the base collection URI after a question mark (`?`), with key-value pairs separated by ampersands (`&`).

```bash
# Example: Retrieve users who are 'active', sorted by 'date' in descending order,
# on the second page with 20 items per page.
GET /users?status=active&sort_by=date&order=desc&page=2&limit=20
```
In an Express.js application, these query parameters are automatically parsed by Express and are easily accessible via the `req.query` object within route handlers and middleware.

---

## ⚠️ Error Handling in APIs

Effective error handling is **critical** for building usable APIs. An API should use appropriate **HTTP status codes** in the response to clearly signal the outcome of a request, especially when it results in an error. This allows clients to programmatically understand what happened. Categories like `4xx` indicate a problem with the client's request, while `5xx` signify a server-side issue.

Key error status codes to use:

*   `404 Not Found`: The requested resource could not be found at the specified URI.
*   `400 Bad Request`: The server cannot process the request due to invalid syntax, malformed request message framing, or deceptive request routing.
*   `422 Unprocessable Entity`: The server understands the request entity's content type, and the syntax is correct, but it was unable to process the contained instructions (commonly used for validation errors in the request body).
*   `401 Unauthorized`: Authentication is required and has failed or has not yet been provided.
*   `403 Forbidden`: The client does not have permission to access the requested resource, even if authenticated.
*   `500 Internal Server Error`: A generic error message indicating that the server encountered an unexpected condition that prevented it from fulfilling the request.

Beyond just the status code, it's best practice to provide more detailed error information in the response **body**. This is typically done using a structured JSON object that includes fields like `title` (a short, human-readable summary), `detail` (more specific information), `status` (repeating the HTTP status code), and potentially an array of specific field `errors` for validation failures.

---

## 📚 API Design Guidelines

Building a good API involves more than just mapping CRUD to HTTP methods. Key guidelines include:

*   Follow **RESTful Principles**: Leverage HTTP methods, represent resources with nouns, maintain statelessness (server doesn't store client state between requests), use standard status codes, and provide representations of resources (e.g., JSON).
*   Maintain **Consistency**: Be consistent in your URI naming conventions, data formats (always use JSON for requests/responses unless specified otherwise), parameter names, error response structures, and authentication methods across all endpoints.
*   Provide clear, comprehensive **Documentation**: Good APIs are well-documented. Provide details on available endpoints, supported HTTP methods for each, required/optional parameters (query, path, body), example request/response payloads, possible status codes, error response formats, and authentication requirements. Popular resources for design philosophy and style guides include the Google API Design Guide and API Stylebook.

---

## 🚀 Implementing HTTP APIs in Express.js

**Express.js** is well-suited for building HTTP APIs due to its flexible routing and middleware system.

### Key Implementation Steps

Implementing a RESTful-like API in Express involves combining core features:

1.  **Define Routes with Parametric Paths:** Use `app.METHOD('/path/:param', handler)` syntax to match the desired endpoints and extract dynamic resource IDs or other values from the URL path. These captured values are available in the `req.params` object. Inside the handler, you implement the logic to fetch, create, update, or delete data, typically interacting with a database or other services based on the path parameters. You then send the appropriate response using methods like `res.json()` (sends JSON data, default status 200 OK), `res.status(statusCode).send()`, or `res.status(statusCode).json()`. Remember to handle cases where a requested element is not found by sending a `404 Not Found` status and an appropriate error body.
    ```javascript
    // Example: GET /dogs/:id endpoint
    app.get('/dogs/:id', async (req, res) => { // Using async is common when handlers perform async operations
      const dogId = req.params.id; // Get ID from path params
      // In a real app, you would fetch the dog from a database or service
      // const dog = await dogsService.getDogById(dogId);

      // Simulate fetching for example:
      const dog = { id: dogId, name: `Doggy ${dogId}`, breed: 'Mixed' }; // Example data

      if (!dog) { // Check if the resource was found
        return res.status(404).json({ error: `Dog with ID ${dogId} not found` }); // Send 404 if not found
      }
      res.json(dog); // Send the dog object as JSON response (defaults to 200 OK)
    });
    ```
2.  **Use Body Parsing Middleware:** For endpoints that receive data in the request body (POST, PUT, PATCH), you **must** configure Express to parse these bodies. Add `app.use(express.json());` to parse JSON bodies and/or `app.use(express.urlencoded({ extended: true }));` to parse URL-encoded bodies. These middleware functions should be placed *before* any routes that need access to `req.body`.
    ```javascript
    // Example: POST /dogs endpoint (expecting JSON body)
    // Assume app.use(express.json()); is configured globally before this route
    app.post('/dogs', async (req, res) => { // Using async for potential DB write
      const newDogData = req.body; // Access the parsed JSON body data

      // In a real app: validate newDogData, save to DB
      // const createdDog = await dogsService.createDog(newDogData);

      // Simulate creation:
      const createdDog = { id: 99, ...newDogData }; // Assign a simulated ID

      res.status(201).json({ message: 'Dog created successfully', dog: createdDog }); // Send 201 Created status with the new resource details
    });
    ```
3.  **Implement Validation:** Integrate a validation library like **`express-validator`** by placing validation middleware functions *before* your route handler but *after* body parsing middleware. Define validation rules for incoming data using `check()`. Inside the handler, check the result with `validationResult(req)`. If validation fails, use `return res.status(400/422).json({ validationErrors: errors.array() });` to immediately respond with error details. If validation passes, proceed with processing the data from `req.body`, `req.query`, etc., knowing it meets your criteria.
    *(Refer to the Validation section in the previous notes for a detailed `express-validator` example combining rules and result checking.)*

Combine these implementation steps with `async/await` within your route handlers to cleanly manage asynchronous operations like database calls. Robust error handling involves using `try...catch` blocks within your handlers to catch potential errors from asynchronous operations or business logic, responding with appropriate `5xx` status codes and informative error bodies (e.g., `res.status(500).json({ error: 'An internal server error occurred.' })`).

---

## 🧪 Testing APIs

Testing API endpoints is essential to ensure they behave as expected, handle inputs correctly, produce the right outputs, and manage errors gracefully. **REST Client tools** are invaluable for this. Popular examples include Postman, Insomnia, or integrated tools within IDEs like the VS Code REST Client extension. These tools allow you to construct and send arbitrary HTTP requests (specifying method, URL, headers, and body) and inspect the full response received from the server.

Using a text-based REST client file (often with `.http` or `.rest` extension) provides a structured way to write and manage API test cases: You define each request with its method, path, headers, and body. Separate individual requests using `###`. The REST client tool can then execute these requests directly from the file and display the complete HTTP response (status code, headers, body) for verification.

```http
### GET all dogs (assuming /dogs endpoint exists)
GET http://localhost:3000/dogs
Accept: application/json

### POST a new dog
POST http://localhost:3000/dogs
Content-Type: application/json

{
  "name": "Buddy",
  "breed": "Golden Retriever",
  "age": 3
}

### GET a specific dog by ID (assuming ID 1 exists)
GET http://localhost:3000/dogs/1
Accept: application/json

### PUT update a specific dog by ID (assuming ID 1 exists)
PUT http://localhost:3000/dogs/1
Content-Type: application/json

{
  "name": "Buddy Updated",
  "breed": "Golden Retriever",
  "age": 4
}

### DELETE a specific dog by ID (assuming ID 1 exists)
DELETE http://localhost:3000/dogs/1

### GET a non-existent dog (expect 404 Not Found)
GET http://localhost:3000/dogs/9999
Accept: application/json
```
This approach facilitates efficient verification of how each endpoint responds to various inputs and methods, confirming correct functionality and error handling according to the API design.