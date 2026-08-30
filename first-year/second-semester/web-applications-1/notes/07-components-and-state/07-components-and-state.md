# Components and State in React

---

## HOOKS

### Enhancing Function Components

**React Hooks** are a significant addition to the React library. They are special JavaScript functions that enable **function components** to "hook into" or utilize advanced React capabilities, such as managing **state** and performing side effects, which were previously only available within **class components**. Hooks were introduced to make function components fully capable alternatives to class components, facilitating a more functional programming style in React.

---

## Limitations of Function Components (Before Hooks)

Prior to the introduction of Hooks (before React version 16.8), function components were considerably more limited compared to **class components**:

*   They could only perform simple rendering logic based on the `props` they received.
*   They were often designed as **pure functions** (meaning they had no internal state and did not produce **side effects**).
*   There was no built-in mechanism for managing mutable **state** directly within a function component (unlike `this.state` in classes).
*   There was no standard way to perform **side effects** after rendering, such as fetching data, setting up subscriptions, or directly manipulating the DOM.
*   They lacked **lifecycle methods** (like `componentDidMount`, `componentDidUpdate`, `componentWillUnmount`) that are available in class components for running code at specific points in a component's life.

---

## Introduction to React Hooks

Hooks were officially introduced in React 16.8 in February 2019. They are distinct from regular functions because they are specifically designed to connect function components to the React state and lifecycle features. You call Hooks directly inside the body of a function component. It is crucial to follow the **Rules of Hooks**: Hooks can *only* be called inside React function components or inside other custom Hooks that you create. They must not be called in regular JavaScript functions or in class components.

---

## Most Popular Built-in Hooks

React provides several built-in Hooks for common tasks:

<p align="center">

| Hook Name            | Purpose and Use Case                                                                                                                            |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| **`useState`**       | Adds a mutable state variable to a function component. It returns an array containing the current state value and a function to update it, triggering re-renders. |
| **`useEffect`**      | Performs side effects after a component renders. Useful for data fetching, subscriptions, manually changing the DOM, logging, etc. You specify dependencies to control when it runs. |
| **`useContext`**     | Allows a function component to subscribe to and access the value provided by a React **Context**. This is used to share data across the component tree without passing props down manually at every level (**"prop drilling"**). |
| **`useReducer`**     | An alternative Hook for state management, often preferred for complex state logic involving multiple sub-values or when the next state depends on the previous one. It's similar to the pattern used in Redux. |
| **`useMemo`**        | Memoizes (caches) the result of an expensive calculation. The calculation is re-run only if the Hook's dependencies change, preventing unnecessary recalculations on every render. |
| **`useCallback`**    | Memoizes (caches) a function definition. This prevents the function from being re-created on every render. Useful for optimizing child components that rely on reference equality (e.g., within `useEffect` dependencies or when passing callbacks to memoized children). |
| **`useRef`**         | Creates a mutable `ref` object whose `.current` property can hold a value that persists across renders without causing re-renders when changed. Commonly used for accessing DOM nodes directly or storing values that don't participate in rendering. |
| **`useLayoutEffect`**| Similar to `useEffect`, but its callback function runs synchronously *after* DOM mutations are calculated by React, but *before* the browser has a chance to paint. Use for reading DOM layout or performing DOM manipulations that should be visible before the user sees the updated UI. |
| **`useDebugValue`**  | Used within **custom Hooks** to display a label for debugging purposes in the React Developer Tools, making it easier to inspect the values managed by your custom Hooks. |

</p>

---

## COMPONENTS: PROPS AND STATE

React components manage and exchange data using three main mechanisms: **Props**, **State**, and **Context**.

### Props, State, and Context Defined

Let's reiterate the core concepts:

*   **Props:** Data that is passed **from a parent component down to its child component**. Props are considered **immutable** (read-only) within the child component that receives them; the child should never modify its own props directly. They are used to configure how a child component looks or behaves (passing data, settings, or callback functions).
*   **State:** Data that is managed **locally** and is **private** to a specific instance of a component. Unlike props, state is **mutable** and is intended to change over time in response to user interactions or other events. Changes to a component's state are the primary way to trigger a **re-render** of that component and its descendants. State must be updated using the designated **setter functions** provided by Hooks like `useState`.
*   **Context:** A mechanism designed to share data (like themes, authentication status, or language settings) that is needed by many components throughout the component tree, without requiring props to be passed down manually through every intermediate level (**"prop drilling"**). Data is provided by a Context Provider higher in the tree and consumed by components lower down using the `useContext` Hook.

---

## Passing Props

You pass **props** to a component using a syntax that resembles HTML attributes when using **JSX**: `<MyComponent propName={value} anotherProp="stringValue" />`. These attributes are collected by React and passed to the component function as the properties of the single `props` object argument. The values passed using curly braces `{}` can be any valid JavaScript expression, including numbers, strings, objects, arrays, functions, or even other React elements. String literals can also be passed directly without curly braces.

```javascript
// Greeting is a function component expecting a 'name' prop
function Greeting(props) {
  return <p>Hello, {props.name}!</p>; // Access the prop value using props.propName
}

// Example usage in JSX:
<Greeting name="Alice" /> // Passes the string "Alice" as the 'name' prop

// Example passing different types of values:
<Counter initial={10} onIncrement={handleCounterIncrement} isVisible={true} data={{ id: 5 }} />
// Inside Counter, props will be { initial: 10, onIncrement: [function], isVisible: true, data: { id: 5 } }
```

---

## Defining State with `useState`

The `useState` Hook is the standard way to add one or more **state variables** to a function component and get a function to update them.

**Example: Implementing Expand/Collapse Functionality Using State**

```javascript
import { useState } from 'react'; // Import the useState Hook

// ShortText component that truncates text and can expand
function ShortText(props) {
  // Declare a state variable 'hidden' and a setter function 'setHidden'.
  // Initialize 'hidden' to true.
  const [hidden, setHidden] = useState(true); // Initial state: text is hidden

  // Use the 'hidden' state variable to decide how to render the text.
  // Attach an event handler to the link that calls the setter function to toggle the state.
  return (
    <span>
      {/* Conditionally render based on the 'hidden' state */}
      {hidden ? `${props.text.substr(0, props.maxLength)}...` : props.text}
      {/* Add a link. When clicked, it calls setHidden, toggling the 'hidden' state. */}
      {/* Calling setHidden requests React to re-render the component. */}
      <a href="#" onClick={(e) => { e.preventDefault(); setHidden(!hidden); }}>
        {hidden ? 'more' : 'less'} {/* Link text depends on the 'hidden' state */}
      </a>
    </span>
  );
}
// Usage: <ShortText text="This is a long text that needs truncation." maxLength={20} />
```

---

## Creating a State Variable with `useState`

The process for adding state with `useState` is:

1.  Make sure to `import { useState } from 'react';` at the top of your component file.
2.  Call `useState(initialValue)` directly inside the body of your function component.
3.  Use array destructuring to unpack the return value of `useState` into two variables: `const [stateVariable, setterFunction] = useState(initialValue);`.
    *   `stateVariable`: This variable holds the current value of the state during the component's current render cycle.
    *   `setterFunction`: This is a function that you call to request that React updates the value of the state variable and triggers a **re-render** of the component.
    *   `initialValue`: This argument is used by React only during the *initial* render of the component to set the starting value of the state. It is ignored on subsequent renders.

The state managed by `useState` is tied to the specific instance of the component and persists across its **re-renders**.

---

## Updating the State

To change the value of a state variable and trigger a **re-render**, you **must** call the `setterFunction` returned by `useState` with the desired new value: `setterFunction(newValue)`. Directly modifying the state variable itself (e.g., `stateVariable = newValue;`) will **not** work and will not cause a re-render. State updates initiated by calling a setter function are typically **asynchronous** and **batched** by React for performance reasons (React might group multiple state updates together and apply them before the next render).

*   **Functional Updates (`setter(prevState => newState)`):** This pattern is strongly recommended when the new state value depends on the *previous* state value (e.g., incrementing a counter, toggling a boolean, adding an item to an array). You pass a function to the setter. React will call this function with the latest previous state value (`prevState`) and use the function's return value as the `newState`. This guarantees you are working with the most up-to-date state value, avoiding potential issues with stale state in asynchronous updates.
    ```javascript
    // Correct: Using functional update to increment based on previous count
    setCount(prevCount => prevCount + 1);

    // Correct: Using functional update to add to an array immutably
    setItems(prevItems => [...prevItems, newItem]); // Creates a new array with old items + newItem
    ```
*   Direct value updates (`setter(newValue)`) are suitable when the new state does not depend on the previous state (e.g., setting the result of a fetch call).

---

## Default Values in `useState`

The argument provided to the `useState()` Hook is used as the initial value for the state variable, but **only on the first render** of the component instance. This initial value can be a static value (like `false`, `0`, `[]`), a value computed directly during the first render, or it can be the result of a **function call for lazy initialization**. Passing a function (`useState(() => computeInitialValue())`) is useful for performing expensive calculations or complex setup only once on the first render, avoiding unnecessary work on subsequent renders.

```javascript
// Example using lazy initialization:
// The function computeExpensiveInitialData will only run on the very first render of this component instance.
const [data, setData] = useState(() => computeExpensiveInitialData(props.config));

function computeExpensiveInitialData(config) {
  console.log("Performing expensive initial computation..."); // This log only appears once per instance
  // ... perform complex calculation ...
  return { result: "initial data" };
}
```

---

## Multiple State Variables

You can call the `useState` Hook multiple times within a single function component to manage independent pieces of **state**. Each call to `useState` declares a separate state variable and its corresponding setter function.

```javascript
import { useState } from 'react';

function MultiStateExample() {
  // Declare three independent state variables
  const [isHidden, setIsHidden] = useState(false);
  const [count, setCount] = useState(0);
  const [items, setItems] = useState([]);

  // ... component logic using isHidden, count, items ...

  // Update each piece of state independently using their respective setters:
  // setIsHidden(true);
  // setCount(prevCount => prevCount + 1);
  // setItems(prevItems => [...prevItems, newItem]);

  return (/* ... JSX ... */);
}
```
Consider grouping related data into a single state object if those pieces of data are highly cohesive or tend to update together.

---

## Can Children Mutate Parent’s State?

No, a fundamental principle of React is that a component **cannot directly change** the state or props of its parent component, any sibling components, or any other ancestor components. A component is only allowed to manage and change its **own** internal **state**.

However, a child component can **request** that its parent (or an ancestor) perform a state change. The standard pattern for this is by the parent component defining a **callback function** (using `useState` setter or `useReducer` dispatch) and passing that function down to the child component as a **prop**. The child component then invokes this callback function when a relevant event or action occurs (e.g., a button click). Calling the parent's callback function triggers the parent's state update logic.

```javascript
// Parent component: manages which button is currently selected
function ButtonGroup() {
  const [selectedIndex, setSelectedIndex] = useState(null); // State to track selected index

  // Define a callback function that the child buttons will call
  // This function updates the parent's state
  const handleButtonClick = (index) => {
    setSelectedIndex(index); // Parent updates its own state based on child's request
  };

  // Render several SimpleButton children
  return (
    <div>
      {/* Pass state slice (isSelected) and the callback (choose) down as props */}
      <SimpleButton index={0} isSelected={selectedIndex === 0} choose={handleButtonClick} text="Easy"/>
      <SimpleButton index={1} isSelected={selectedIndex === 1} choose={handleButtonClick} text="Medium"/>
      <SimpleButton index={2} isSelected={selectedIndex === 2} choose={handleButtonClick} text="Hard"/>
    </div>
  );
}

// Child component: displays a button and signals clicks
function SimpleButton(props) { // Receives props: { index, isSelected, choose, text }
  const { index, isSelected, choose, text } = props; // Destructure props for easier access

  return (
    // Attach the parent's callback (props.choose) to the button's onClick event
    <button
      onClick={() => choose(index)} // Child calls parent's callback, passing its index
      style={{ backgroundColor: isSelected ? 'lightblue' : '' }} // Style based on prop derived from parent state
    >
      {text}
    </button>
  );
}
```
When a `SimpleButton` is clicked, it calls the `choose` prop function, passing its `index`. `choose` is actually the `handleButtonClick` function from the `ButtonGroup` parent. `handleButtonClick` updates the `selectedIndex` state in `ButtonGroup`. This state change triggers `ButtonGroup` to **re-render**. During re-render, `ButtonGroup` passes updated `isSelected` props down to its children (`selectedIndex === index`). Children receive the new `isSelected` prop and re-render accordingly (e.g., updating their background color). This demonstrates the **unidirectional data flow**: child requests change via callback, parent updates state, state change propagates down via props.

---

## React Design Hints Summary

To effectively build UIs with React:

*   Favor creating **Simple, Stateless (Presentational)** components that focus purely on rendering UI based on the `props` they receive.
*   When multiple components need access to the same state or the state is used to coordinate them, **Lift State Up** to their closest common ancestor component.
*   Always pass data **downwards from parent to child** using `props`.
*   When a child needs to initiate a change in shared state managed by an ancestor, pass **callback functions down as `props`** from the state owner to allow descendants to request state updates.
*   Use the **`useState`** Hook for managing state that is local and specific to a single component instance.
*   When updating state where the new value depends on the previous value (e.g., toggling, incrementing, adding to list), use **functional updates** with setters (`setter(prevState => newState)`) to ensure correctness.
*   Treat the **Props** object received by a component as **Immutable**; never modify the props object or its contents directly within the receiving component.