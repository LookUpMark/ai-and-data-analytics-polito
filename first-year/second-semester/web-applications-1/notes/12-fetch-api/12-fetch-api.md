# Fetch API

---

## Goal

Client-side JavaScript applications, such as those built with **React**, frequently need to interact with a backend server by making HTTP requests. The goals addressed in this section relate to:

*   Asynchronously loading data from a server without blocking the user interface.
*   Sending various types of asynchronous HTTP requests using the modern **Fetch API**.
*   Managing scenarios involving multiple concurrent or sequential requests.
*   Briefly introducing alternative libraries like **Axios** for more advanced use cases.

---

## Asynchronous JS Requests: Fetch API

The **Fetch API** is the modern, built-in browser interface provided by web browsers for making asynchronous network requests. As touched upon in JavaScript asynchronous concepts, `fetch` is **Promise-based** and fundamental to performing non-blocking data transfers over HTTP from the client side.

Reference: For comprehensive documentation, consult the MDN Fetch API guide ([https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)).

---

## Asynchronous API Data Transfers: Conceptual Overview

In a typical architecture, a **React** application running client-side in the user's browser interacts with a separate API Server (backend). This interaction involves the Client-Side React app initiating **HTTP Requests** using an API like `fetch`, sending them over the network to the Backend API Server. The API Server, which hosts the application logic and often interacts with a database, processes these requests and sends back **HTTP Responses** containing the requested data or status information.

Conceptual Flow: Client UI interacts with Client Logic (which includes data fetching functions) -> Client Logic uses `fetch` to send Request -> Network -> API Server (processes Request, potentially DB interaction) -> API Server sends Response -> Network -> Client Logic receives Response -> Client Logic processes data -> Updates Client UI.

Within **React**, performing such data fetching (especially on component mount or update) is considered a **side effect**, and is typically managed using the `useEffect` Hook. The `fetch()` method is specifically what executes the underlying HTTP call.

---

## How to Exchange Data Asynchronously with `fetch()`

The core method of the Fetch API is `fetch()`. Its basic syntax is `fetch(URL [, initObject])`.

*   The **first argument** is the `URL` of the resource you want to access. This is required.
*   The **second argument**, `init`, is an optional configuration object (see Fetch Options below) allowing you to customize the request (e.g., specify the HTTP method, add headers, include a request body). If omitted, the default method is `GET`.
*   The `fetch()` method is available in browser JavaScript contexts.
*   Crucially, `fetch()` returns a **Promise**.

**Understanding the Fetch Promise Behavior:**

The **Promise** returned by `fetch()` behaves in a specific way:

*   It **fulfills** when the server responds, even if the response indicates an HTTP error (like 404 Not Found or 500 Internal Server Error). On fulfillment, the Promise's value is a `Response` object containing information about the response, including the status code.
*   It **rejects** **only for network errors** that prevent the request from completing successfully at all (e.g., DNS lookup failure, server being unreachable, browser is offline, or a CORS policy block preventing the request from being sent or received).

This distinction is vital for correct error handling.

---

## Example: Handling the `fetch` Promise

Because `fetch()` returns a **Promise**, you handle its eventual outcome (the `Response` object or a network error) using either `.then()` chaining or the `async/await` syntax within an `async` function.

**Using `.then()` chaining:**

```javascript
fetch(url)
  .then(response => {
    // This first '.then' block runs as soon as headers are received, even on HTTP errors (404, 500).
    // 'response' is the Response object.
    console.log('Response received, status:', response.status);

    // Check response.ok (true for 200-299 statuses) to handle HTTP errors explicitly.
    if (!response.ok) {
      // If it's an HTTP error, throw an Error. This will cause the Promise chain to skip
      // subsequent '.then' blocks and jump directly to the nearest '.catch' block.
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    // If status is OK, parse the response body as JSON. response.json() also returns a Promise.
    // Returning this Promise ensures the next '.then' waits for JSON parsing to complete.
    return response.json();
  })
  .then(data => {
    // This second '.then' block runs only if the fetch was successful AND JSON parsing succeeded.
    // 'data' contains the parsed JavaScript object/value from the JSON body.
    console.log('Data parsed successfully:', data);
    // Use the data to update UI, etc.
  })
  .catch(error => {
    // This '.catch' block handles any errors that occurred in the chain:
    // 1. Network errors that caused the original fetch() Promise to reject.
    // 2. Errors thrown explicitly in any preceding '.then' block (like the HTTP error check).
    // 3. Errors during JSON parsing (if response.json() failed).
    console.error('An error occurred during the fetch operation:', error);
    // Handle the error, e.g., display an error message in the UI.
  });
```

**Using `async/await` (inside an `async` function):**

```javascript
async function fetchData(url) { // Define an async function to use await
  try { // Use try...catch for error handling, similar to synchronous code
    // 'await fetch(url)' pauses the async function execution until the fetch Promise settles (network completes).
    // If a network error occurs, it throws, jumping to the catch block.
    // If successful (even with HTTP error), it resolves with the Response object.
    const response = await fetch(url);
    console.log('Response received, status:', response.status);

    // Check response.ok to handle HTTP errors explicitly.
    if (!response.ok) {
      // If it's an HTTP error, throw an Error. This makes 'await' re-throw it, jumping to the catch block.
      throw new Error(`HTTP error: ${response.status} ${response.statusText}`);
    }

    // 'await response.json()' pauses until the response body is read and parsed as JSON.
    // If JSON parsing fails, it throws, jumping to the catch block.
    const data = await response.json();
    console.log('Data parsed successfully:', data);
    // Use the data.
    return data; // Optionally return the data from the async function
  } catch (error) {
    // This catch block handles:
    // 1. Network errors from 'await fetch'.
    // 2. Errors thrown by the explicit HTTP error check.
    // 3. Errors from 'await response.json()' if JSON is invalid.
    console.error('An error occurred during the fetch operation:', error);
    // Handle the error.
    throw error; // Optionally re-throw the error
  }
}

// Call the async function to initiate the fetch. This call itself is non-blocking and returns a Promise.
// You can optionally chain .then/.catch onto this call to handle the final outcome of the async function.
fetchData(url)
  .then(data => console.log("Fetch process complete."))
  .catch(error => console.error("Fetch process ended with an error."));
```
The `async/await` syntax often results in code that is easier to read and reason about when sequencing multiple asynchronous steps or handling errors, as it resembles traditional synchronous try/catch logic.

---

## The `Response` Object

When the **Promise** returned by `fetch` fulfills successfully (meaning the network request completed and the server responded), its value is a `Response` object. This object contains detailed information about the server's response.

Main Properties of the `Response` object:

*   `Response.ok` (boolean): A boolean indicating whether the HTTP status code is in the 200-299 range (inclusive).
*   `Response.status` (number): The numeric HTTP status code returned by the server (e.g., 200, 404, 500).
*   `Response.statusText` (string): The corresponding HTTP status message (e.g., "OK", "Not Found", "Internal Server Error").
*   `Response.headers` (`Headers` object): An object providing access to the response headers using methods like `.get('Header-Name')`.
*   `Response.url` (string): The final URL of the request after any redirects.
*   `Response.redirected` (boolean): A boolean indicating if the request was redirected.
*   `Response.body` (`ReadableStream`): Provides a low-level interface to access the response body content as a stream. (Less commonly used directly than the body reading methods below).

Reference: More details about the `Response` object are available on MDN ([https://developer.mozilla.org/en-US/docs/Web/API/Response](https://developer.mozilla.org/en-US/docs/Web/API/Response)).

---

## Accessing Response Headers: Example

You can access individual response headers using the `get()` method on the `response.headers` object, which is an instance of the `Headers` interface.

```javascript
fetch(url)
  .then(response => {
    console.log('Content-Type header:', response.headers.get('Content-Type'));
    console.log('Date header:', response.headers.get('Date'));
    console.log('Server status:', response.status, response.statusText);
    console.log('Is response OK (status 200-299)?', response.ok);
    // Continue processing the response body...
    // return response.json(); // or other body reading methods
  });
```

---

## Error Handling with `fetch`

As highlighted earlier, handling errors correctly with `fetch` requires understanding its **Promise** behavior:

*   The **Promise** returned by `fetch()` **only rejects** for network-level failures (like connection refused, request timeout, server down, or a browser-enforced CORS policy preventing the request/response).
*   The **Promise** returned by `fetch()` **fulfills** for *all* valid HTTP status codes returned by the server, including error statuses like 404 Not Found or 500 Internal Server Error.

Therefore, simply checking if the `fetch` **Promise** fulfilled is **not** enough to determine if the API call was successful from an application standpoint. You must check the HTTP status code within the fulfillment handler.

Suggested Approach for robust error handling with `fetch`:
1.  In the first `.then()` block (or immediately after `await fetch`), check the `response.ok` property or inspect `response.status`. If the status indicates an application-level error (e.g., not in the 200s range), **throw a new Error** with relevant status information. This will propagate the error down the **Promise** chain.
2.  (Optional but Recommended) You might also want to check the `Content-Type` header to ensure the response is in the expected format (e.g., `application/json`) before attempting to parse the body. Throw an error if the type is incorrect.
3.  Include a `.catch()` block at the end of your **Promise** chain (or use a `try...catch` block with `async/await`). This single block will then handle all types of errors: the initial network errors that caused the `fetch` Promise to reject, any errors explicitly thrown in the `.then()` blocks (e.g., for HTTP status or wrong content type), and potentially errors that occur during body parsing (e.g., if `response.json()` fails because the body isn't valid JSON).

---

## Example: Error Handling Implementation

This example demonstrates implementing the suggested error handling approach using Promise chaining:

```javascript
fetch(url)
  .then(response => {
    // Step 1: Check for non-OK HTTP status codes
    if (!response.ok) {
      // Throw an error with details if the status is outside 200-299
      throw new Error(`HTTP Error: ${response.status} ${response.statusText}`);
    }

    // Step 2 (Optional): Check the Content-Type header if you expect specific format
    const contentType = response.headers.get('Content-Type');
    if (contentType && !contentType.includes('application/json')) {
      // Throw an error if the content type is not as expected
      throw new TypeError(`Expected JSON, received ${contentType}`);
    }

    // If all checks pass, parse the response body as JSON.
    // response.json() returns a Promise, which is returned to the next .then().
    return response.json();
  })
  .then(data => {
    // This block is reached only if the fetch was successful AND the response was parsed as JSON without errors.
    console.log('Success:', data);
    // Process the successful data here.
  })
  .catch(err => {
    // This catch block handles ANY error that occurred previously in the chain:
    // - Network errors (fetch Promise rejected initially)
    // - Errors thrown in the first .then (HTTP status, Content-Type checks)
    // - Errors during response.json() if the body is invalid JSON
    console.error('Fetch failed:', err);
    // Display an error message in the UI or handle the error appropriately.
  });
```

---

## Fetch Options (`init` object)

The optional second argument to the `fetch()` method, the `init` object, allows you to customize the request significantly.

`fetch(url, initObject)`

Main properties within the `init` object:

*   `method`: Specifies the HTTP method to use, as a string (e.g., `'POST'`, `'PUT'`, `'DELETE'`, `'PATCH'`, `'HEAD'`, `'OPTIONS'`). Defaults to `'GET'` if omitted.
*   `headers`: An object or a `Headers` object to set custom HTTP headers for the request (e.g., `{ 'Content-Type': 'application/json', 'Authorization': 'Bearer token' }`).
*   `body`: Contains the data to be sent in the request body. Used with methods like `POST`, `PUT`, and `PATCH`. The value can be a string, `Blob`, `FormData`, `URLSearchParams`, `ReadableStream`, or byte array. For sending JSON, you must use `JSON.stringify(yourJsObject)`.
*   `mode`: Controls the mode of the request, influencing CORS behavior. Common values: `'cors'` (default, standard cross-origin requests), `'no-cors'` (allows certain simple cross-origin requests but with restricted access to the response), `'same-origin'` (requests only to the same origin).
*   `credentials`: Specifies whether to send cookies, authorization headers, or TLS client certificates with cross-origin requests. Values: `'omit'` (default, do not send), `'same-origin'` (send only for same-origin URLs), `'include'` (always send).
*   `signal`: Takes an `AbortSignal` object from an `AbortController`. This allows you to cancel the fetch request programmatically (e.g., if a component unmounts).

Reference: See MDN fetch init documentation for a full list of parameters ([https://developer.mozilla.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch#parameters](https://developer.mozilla.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch#parameters)).

---

## Example: `POST` Request with JSON Content

Sending data to the server using methods like POST or PUT requires providing a request body and setting the appropriate `Content-Type` header.

```javascript
let dataToSend = { name: 'New Task', done: false };
let url = '/api/tasks'; // Using a relative URL assumes frontend and backend are on the same origin

fetch(url, {
  method: 'POST', // Explicitly set the HTTP method to POST
  headers: {
    // Set the Content-Type header to indicate the body format
    'Content-Type': 'application/json'
  },
  // Provide the request body. For JSON, stringify your JS object.
  body: JSON.stringify(dataToSend)
})
.then(response => {
  // Handle HTTP status check
  if (!response.ok) {
    // Read response body for error details if available
    return response.json().then(errorData => { throw new Error(`HTTP error: ${response.status}`, { cause: errorData }); });
    // Or just throw new Error(`HTTP error: ${response.status}`);
  }
  // If OK, parse the successful response body as JSON
  return response.json();
})
.then(data => {
  console.log('Task created successfully:', data); // 'data' is the parsed response body from the server
  // Update UI with created task info
})
.catch(error => {
  console.error('POST request failed:', error); // Handle any errors (network, HTTP, parsing)
  // Display error to user
});
```

---

## Reading The Response Body

Once you have a `Response` object (after the `fetch` **Promise** fulfills and you've potentially checked the status), you can read the response body content. The `Response` object provides several methods for this, all of which return **Promises** because reading the body content is itself an asynchronous operation (especially for large responses).

Common methods for reading the response body:

*   `response.text()`: Reads the body and resolves with the content as a plain string.
*   `response.json()`: Reads the body, attempts to parse it as JSON, and resolves with the resulting JavaScript object/value. It will reject if the body content is not valid JSON.
*   `response.formData()`, `response.blob()`, `response.arrayBuffer()`: Used for reading the body as other data types (`FormData`, `Blob`, or `ArrayBuffer`, respectively).

**Important Limitation:** You can use **only one** of these body-reading methods (`.text()`, `.json()`, `.blob()`, etc.) on a given `Response` object. Once a body method is called, the body is considered "consumed" and cannot be read again by another method.

Reference: More details on Response body methods can be found on MDN ([https://developer.mozilla.org/en-US/docs/Web/API/Response/body#methods](https://developer.mozilla.com/en-US/docs/Web/API/Response/body#methods)).

---

## Sequential Fetches (Dependent Requests)

Sometimes, one API request depends on the result of a previous one. For instance, you might need to fetch a user's ID based on their username, and then use that ID to fetch detailed profile information. You sequence these dependent asynchronous operations using **Promises**. `async/await` syntax is highly recommended for this scenario as it makes the sequential flow look much cleaner than deeply nested `.then()` chains.

```javascript
// Assume /api/users?username=:username returns [{ id: ..., ... }]
// Assume /api/users/:id returns { ..., id: ... }

async function getUserIdAndDetails(username) {
  try {
    // Step 1: Fetch user list to find the user's ID
    const usersResponse = await fetch(`/api/users?username=${username}`);
    if (!usersResponse.ok) {
        // Handle failure to get user list (e.g., network error, server error)
        throw new Error('Failed to fetch user list.');
    }
    const users = await usersResponse.json(); // Parse the response body

    // Check if user was found
    if (!users || users.length === 0) {
        throw new Error(`User with username "${username}" not found.`);
    }
    const userId = users[0].id; // Extract the user ID from the first result

    // Step 2: Use the obtained user ID to fetch detailed information
    const detailsResponse = await fetch(`/api/users/${userId}`);
    if (!detailsResponse.ok) {
        // Handle failure to get user details
        throw new Error(`Failed to fetch details for user ID ${userId}.`);
    }
    const userDetails = await detailsResponse.json(); // Parse the details response body

    // Return the final result
    return userDetails;
  } catch (error) {
    // Catch block handles any errors from either fetch call, parsing, or checks
    console.error("Sequential fetch process failed:", error);
    throw error; // Re-throw for the code that called getUserIdAndDetails
  }
}

// Example Usage:
// getUserIdAndDetails('alice')
//   .then(userDetails => {
//     console.log("Successfully fetched user details:", userDetails);
//     // Update UI with userDetails
//   })
//   .catch(error => {
//     console.error("Handling error from getUserIdAndDetails:", error);
//     // Display error message
//   });
```

---

## Parallel Fetches (Independent Requests)

When you need to fetch multiple resources that are independent of each other (i.e., the second request does not need data from the first), you can make these requests concurrently to reduce the total loading time. The static `Promise.all()` method is ideal for managing a collection of **Promises** that should run in parallel.

*   `Promise.all(iterable)`: Takes an iterable (like an array) of **Promises**. It returns a single **Promise** that resolves when *all* of the Promises in the input iterable have successfully fulfilled. The fulfillment value is an array containing the fulfillment values of the input Promises, in the same order as the original input iterable. It rejects immediately if *any* of the input Promises reject, with the reason of the first **Promise** that rejected.

```javascript
// Define URLs for independent resources
const urls = ['/api/resource1', '/api/resource2', '/api/resource3'];

// Create an array of fetch Promises, ensuring each handles its own HTTP error check and JSON parsing.
// Using map() to transform URLs into fetch Promises.
const fetchPromises = urls.map(url =>
  fetch(url) // Initiate fetch for each URL concurrently
    .then(response => {
      // Handle HTTP error for THIS specific fetch request
      if (!response.ok) {
        throw new Error(`Fetch failed for ${url}: ${response.status}`);
      }
      // Parse the response body as JSON. This returns a Promise.
      // This Promise is what Promise.all waits for from this specific fetch.
      return response.json();
    })
);

// Use Promise.all() to wait for all promises in the array to settle.
Promise.all(fetchPromises)
  .then(results => {
    // This block runs only if ALL fetch and JSON parsing operations succeeded.
    // 'results' is an array where results[0] is the parsed data from urls[0], results[1] from urls[1], etc.
    console.log('All resources fetched successfully:', results);
    // Process the results from all fetches.
  })
  .catch(error => {
    // This catch block runs if ANY of the fetch or parsing Promises in the array rejected.
    // 'error' will be the reason of the first rejection encountered.
    console.error('At least one fetch failed:', error);
    // Handle the error, e.g., display a general error message.
  });
```
Other useful `Promise` static methods for parallel operations include:

*   `Promise.allSettled(iterable)` (ES2020): Waits for all promises to settle (fulfill or reject) and returns an array describing the outcome of each. Useful when you want to know the result of all requests, even if some fail.
*   `Promise.any(iterable)` (ES2021): Fulfills with the value of the first promise that fulfills. Rejects only if all promises reject.

---

## Basic `fetch` vs. Other Libraries (e.g., Axios)

While the native `fetch` API is powerful and sufficient for many common asynchronous HTTP tasks, third-party libraries like **Axios** are widely used in the React ecosystem because they provide additional features and conveniences that simplify common workflows.

Advantages of using a library like Axios over basic `fetch`:

*   **Automatic JSON Conversion:** Axios automatically transforms request data to JSON (if passing an object) and parses response data as JSON by default.
*   **Easier Request Cancellation:** Built-in support for cancelling requests.
*   **Built-in Timeout:** Simple configuration for request timeouts.
*   **Easier Progress Bar Support:** Provides events for tracking upload/download progress.
*   **Request/Response Data Transformation:** Easily define transformations before sending requests or after receiving responses.
*   **Interceptor Support:** Allows adding global handlers that run before requests are sent or after responses are received (e.g., for adding auth tokens or error logging).
*   **Easier Parallel Request Handling:** Often provides helper methods like `axios.all()` which work similarly to `Promise.all()` but within the Axios context.
*   **Seamless in Node.js:** Axios can be used for server-side HTTP calls in Node.js, whereas the browser's `fetch` API is not natively available in Node.js (though polyfills exist).

For simple fetch calls and moderate complexity, `fetch` is perfectly capable. For more complex scenarios, standardizing with a library like Axios can reduce boilerplate and improve maintainability.

---

<p align="center">
| Property             | Description                                                                                                |
| :------------------- | :--------------------------------------------------------------------------------------------------------- |
| `method`             | HTTP method (e.g., `'POST'`).                                                                              |
| `headers`            | Custom HTTP headers object.                                                                              |
| `body`               | Request body content (e.g., `JSON.stringify(data)`).                                                       |
| `mode`               | Request mode, influencing CORS (`'cors'`, `'no-cors'`, `'same-origin'`).                                  |
| `credentials`        | Include cookies/auth headers (`'omit'`, `'same-origin'`, `'include'`).                                     |
| `signal`             | `AbortSignal` for request cancellation.                                                                    |
</p>