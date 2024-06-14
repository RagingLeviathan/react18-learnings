use react.ts for plain typescript files
use react.tsx for react typescript files

two ways to create a react component;
- use a js class
- function 
these days, we use functions because they are easier to read and write

pascal casing is used for naming components (eg. PascalCasing)

example of first react typescript file component;
/**
 two ways to create a react component;
- use a js class
- function 
these days, we use functions because they are easier to read and write
 */

function Message() {
    //exanple of jsx: javascript xml
    return <h1>Hello, World!</h1>;
    //this code, under the hood, will get compiled down into javascript
}

//to use it, we export it
export default Message;

to use it in another file, we import it, like so;
//my version;
import Message from "./Message";

function App() {
  return <div><Message></Message></div>
}

checking back to the terminal where npm run dev was used in, you can see this when making changes;
12:30:53 [vite] hmr update /src/App.tsx
12:30:56 [vite] page reload src/App.tsx
hmr = hot module replacement, which vite refreshes automatically

so with jsx, we can easily describe the user interface of our application with html and javascript

it also allows for the creation of dynamic content

React DOM is a library that provides DOM-specific methods that can be used at the top level of a web app to enable an efficient way of managing updates to the DOM. It's part of the React ecosystem and works alongside the core React library.

Here are some key points about ReactDOM:

ReactDOM.render(): This is the most important method provided by ReactDOM. It's used to render a React element into a root DOM node. If the React element was previously rendered into the container, this will perform an update on it and only mutate the DOM as necessary to reflect the latest React element.

ReactDOM.unmountComponentAtNode(): This method removes a mounted React component from the DOM and cleans up its event handlers and state. If no component was mounted in the container, calling this function does nothing.

ReactDOM.findDOMNode(): If this method is called on a component instance, it returns the DOM node where the component was rendered. However, using this method is generally discouraged because it breaks component encapsulation.

ReactDOM.createPortal(): This method is used for rendering children into a DOM node that exists outside the DOM hierarchy of the parent component.

Server-side rendering: ReactDOM also provides methods like renderToString() and renderToStaticMarkup() for server-side rendering of React components.

Remember, React and ReactDOM work together. React handles the creation and management of components, while ReactDOM handles rendering and updating these components in the DOM.

example from main.tsx ;
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

//we are taking this component and rendering it to the root element in the index.html file
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode> //built in component, no visual representation, but it helps with debugging
    <App />
  </React.StrictMode>,
)

chatgpt explanation of what it's doing; 
This is a piece of TypeScript code using React and ReactDOM libraries. Here's a breakdown of what it does:

ReactDOM.createRoot(document.getElementById('root') as HTMLElement): This line is using the new concurrent mode API introduced in React 18. It creates a root node where your React application will be attached. The document.getElementById('root') as HTMLElement part is getting a DOM element with the id 'root' and treating it as an HTMLElement.

.render(): This is a method provided by ReactDOM. It's used to render a React element into the root DOM node created in the previous step.

Inside the render() method, a React.StrictMode component is being rendered. This is a wrapper component that checks for potential problems in the application during the development build. It does not impact the production build.

<App />: This is the main component of your React application. It's being wrapped in React.StrictMode to ensure that potential problems are caught during development.

In summary, this code is setting up a React application, creating a root node in the DOM, and rendering the App component into that root node. It's using the new concurrent mode API (createRoot) introduced in React 18, which allows for better handling of priority-based rendering in React.

react is platform agnostic, meaning it can be used on the web, mobile, and desktop

northworthy point;
library vs framework;
- a library is a collection of functions that you can call, these functions will perform a task, but you are in control of the flow of the application
- it's a tool that provides specific functionality, but you are in control of the flow of the application

- a framework is a collection of libraries that provide a structure for your application
- it's a set of tools/toolset and conventions that dictate the flow of the application. it provides guidelines on how to build your application

*react is just a tool for building user interfaces, it's not a full-fledged framework*
the only thing that it does, and is good at, is creating dynamic and interactive user interfaces
but we hardly use only react to build modern applications. we often need additional tools for concerns such as routing, which means allowing the user to go from one page to another, make initiative calls, managing the application state, internationalization, form validation, animations and so on.

the great thing about react is tat it dosn't have an opinion about the additional tools you use for these concerns, so we can pick and choose the tools we want to use

now, observe;
when creating this react typescript component, which will retrn a bootstrap component ;
function ListGroup() {
    return (
        <ul class="list-group">
  <li class="list-group-item">An item</li>
  <li class="list-group-item">A second item</li>
  <li class="list-group-item">A third item</li>
  <li class="list-group-item">A fourth item</li>
  <li class="list-group-item">And a fifth one</li>
</ul>
    );
}

export default ListGroup;
----
....typescript will throw errors - why? because class is a reserved keyword in typescript and javascript. so here, we need to rename all these classes to class name, like so;
function ListGroup() {
    return (
        <ul className="list-group">
  <li className="list-group-item">An item</li>
  <li className="list-group-item">A second item</li>
  <li className="list-group-item">A third item</li>
  <li className="list-group-item">A fourth item</li>
  <li className="list-group-item">And a fifth one</li>
</ul>
    );
}

export default ListGroup;
---
a quick way to rename is to use ctrl + d and continue until all class values are selected, then delete and rewrite

now... if you were wanting to return a header too in this component, like so;
function ListGroup() {
  return (
    <h1>List</h1>
    <ul className="list-group">
      <li className="list-group-item">An item</li>
      <li className="list-group-item">A second item</li>
      <li className="list-group-item">A third item</li>
      <li className="list-group-item">A fourth item</li>
      <li className="list-group-item">And a fifth one</li>
    </ul>
  );
}

export default ListGroup;
------
you'll note that you'll get an error... because react CANNOT return more than one element from a component. this is because the h1 element will get compiled into javascript (something like React.createElement('h1')), with h1 being the type of the elemen. the same thing will happen for the second element (ul) so in this function. ListGroup, we are returning multiple elements, which is not allowed.

so, to solve this, there are a few options. one is to wrap the entire expression in a div, like so;
- highlight the entire expression
-then press ctrl + p to bring up the command palette
- then type '>wrap' (> is an abbreviation here)
- then type div, and hit enter

like so;
function ListGroup() {
  return (
    <div>
        <h1>List</h1>
        <ul className="list-group">
          <li className="list-group-item">An item</li>
          <li className="list-group-item">A second item</li>
          <li className="list-group-item">A third item</li>
          <li className="list-group-item">A fourth item</li>
          <li className="list-group-item">And a fifth one</li>
        </ul>
    </div>
  );
}

export default ListGroup;

.....however, using this, we are adding one extra element into the DOM, purely for the purposes of making react happy and the code valid. this is not a good practice, and unnecessary, because it can affect the layout of the page. so, we can use a fragment instead, like so;

import { Fragment } from 'react';
function ListGroup() {
  return (
    <Fragment>
        <h1>List</h1>
        <ul className="list-group">
          <li className="list-group-item">An item</li>
          <li className="list-group-item">A second item</li>
          <li className="list-group-item">A third item</li>
          <li className="list-group-item">A fourth item</li>
          <li className="list-group-item">And a fifth one</li>
        </ul>
    </Fragment>
  );
}

export default ListGroup;
---
...again, selecting div and going ctrl + d as much as necessary to replace with Fragment. also remember to import Fragment from react

with this change, when this component is rendered on the screen, we are not gonna have an additional element like a div, but the fragment will be removed from the DOM, and the children of the fragment will be rendered directly to the DOM.

BUT ---
there is an even better way to achieve the same result!!!!
there is a shorter syntax. we do not have to import the fragment component from react, we can use an empty tag or empty angle brackets, like so;

function ListGroup() {
  return (
    <>
        <h1>List</h1>
        <ul className="list-group">
          <li className="list-group-item">An item</li>
          <li className="list-group-item">A second item</li>
          <li className="list-group-item">A third item</li>
          <li className="list-group-item">A fourth item</li>
          <li className="list-group-item">And a fifth one</li>
        </ul>
    </>
  );
}

export default ListGroup;
---
... which tells react to not render anything, but to still return the jsx/ use a fragment to wrap all these children

in jsx, we don't have a for loop :(
hence, a different technique is required
in javascript, arrays have a method called map for mapping and converting/transforming each item into an item/something else.

something like this :)

const items = [
"An item",
"A second item",
"A third item",
"A fourth item",
"And a fifth one",
];

// items.map(item => <li>{item}</li>) //this will return an array of list items

but, if we want to return this in a fragment;
function ListGroup() {
  const items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  return (


 
    <>


{/* //take each item and convert it to an item of a different type.
  //so here, we wanna convert each item to an li element.
  //and inside the tags we want to render, or display the item itself
  //remember that curly braces {} are used to render data dynamically
  // so here, we render the item itself */}
  <h1>List</h1>
      <ul className="list-group">
  items.map(item => <li>{item}</li>)
  </ul>
      
    </>
  );
}

export default ListGroup;
------
... but this will not work, because we'll get a compilation error. because this expression is not allowed in the middle of a jsx mark-up. in jsx, we can only use html elements, or other react components. so, to render data, dynamically we need to wrap this expression in braces, to tell react that we are writing javascript code, like so;

function ListGroup() {
  const items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  return (
    <>
      <h1>List</h1>
      <ul className="list-group">
        {items.map((item) => (
          <li>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
---
(the extension prettier added the backets, making it more readable)

and this works!!! BUT!!!!
rightclicking on our react app in-browser, the console tab will give this warning;
react_jsx-dev-runtime.js?v=09301dfa:62 Warning: Each child in a list should have a unique "key" prop.

Check the render method of `ListGroup`. See https://reactjs.org/link/warning-keys for more information.
    at li
    at ListGroup
    at div
    at App

so what does it mean? it means that , in our mapping function above, each list item should have a unique key prop. this is because react uses this key prop to keep track of the elements in the list, and to know which elements have changed, and which elements have been added or removed. so, when rendering a list of items using the map method, we should give each item a unique key. 

to fix this, we need to add a key prop to each list item. since here, each item is a unique string, each item can itself be used as a unique key. but in a real world app, we'd retrieve items from an api, with its own ids (as in a database ) ;
function ListGroup() {
  const items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  return (
    <>
      <h1>List</h1>
      <ul className="list-group">
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;


ALSO ---commenting in react jsx or tsx files 
it is different from regular javascript commenting. because it gets compiled, it is closer to javascript than html, in spite of appearances, so in jsx, we use curly braces, like so;
In a React JSX or TSX file, you can leave comments inside a fragment using the JavaScript comment syntax 
<>
  {/* This is a comment */}
  <YourComponent />
</>

 The curly braces are used to switch back into JavaScript context within the JSX/TSX code.


conditional rendering;
sometimes, we want to render different content based on certain conditions., e.g., in a ListGroup, we can add an if statement to check if the list is empty, and if it is, we can render a message saying that the list is empty. like so;

function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  items = [];

  //using conditional rendering; if there are no items in the list, we want to display a message
  if (items.length === 0)
    return (
      <>
        <h1>List</h1>
        <p>There are no items in the list.</p>;
      </>
    );

  return (
    <>
      {/* //take each item and convert it to an item of a different type.
  //so here, we wanna convert each item to an li element.
  //and inside the tags we want to render, or display the item itself
  //remember that curly braces {} are used to render data dynamically
  // so here, we render the item itself */}
      <h1>List</h1>
      <ul className="list-group">
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
----
.but this will lead to code duplication, which is not good coding practice. so, a better way is to render things conditionally, inside our jsx expression;
//import { Fragment } from 'react';
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  items = [];

  return (
    <>
      {/* //take each item and convert it to an item of a different type.
  //so here, we wanna convert each item to an li element.
  //and inside the tags we want to render, or display the item itself
  //remember that curly braces {} are used to render data dynamically
  // so here, we render the item itself */}
      <h1>List</h1>
      { items.length === 0 ? <p>There are no items in the list.</p> : null } {/*use ternary operator here to conditionally render a message */ }
      <ul className="list-group">
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
------
...sometimes, though, this logic can get a little too complicated, and polute our jsx markup. in such cases, we can extract this logic and store it in a separate variable or constant, like so;
//import { Fragment } from 'react';
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  items = [];

 //this logic can also be moved inside a function, and then we can call that function inside the jsx
  const message = items.length === 0 ? <p>There are no items in the list.</p> : null ; //use ternary operator here to conditionally render a message

  return (
    <>
      <h1>List</h1>
      {message} {/*version 2.0*/ }
      <ul className="list-group">
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
------
....and here is a version where the variable is called from a function; 
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  items = [];

  //like so (using arrow function syntax)
  const getMessage = () => {
    return items.length === 0 ? <p>There are no items in the list.</p> : null ; //use ternary operator here to conditionally render a message
  }

  return (
    <>
      <h1>List</h1>
      {getMessage()} { /* example of calling a function */}
      <ul className="list-group">
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
---
... the benefit of this is that it makes our jsx markup cleaner and easier to read. it also makes it easier to test this logic, because we can write tests for this function, and make sure that it works as expected. 
as well as that, one of the benefits means that our functions can have parameters, and different arguments basded on specific conditions, and we can pass data to them, and they can return data back to us. and be reused.

if missing this kind of scenario - consts are preferred.

there's an even better way to make code more concise;
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  items = [];

  return (
    <>

      <h1>List</h1>
      {/*now for a more concise way to write code*/}
      {/*1.) this way is fine, but the null part is problematic, so there is a better way; */}
      {items.length === 0 ? <p>There are no items in the list.</p> : null  }  
      {/*2.) start the same, but instead of tenerary operator or quesiton mark, use ; a logical AND */}
      {items.length === 0 && <p>There are no items in the list.</p> } {/* paragraph tag will be returned if true; for false, nothing will be rendered on screen */}
      <ul className="list-group">
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;


(remember to use braces {} to render anything dynamically)

onclick things (handling events) --;
in react, each element has a property, or a prop, called onClick, which is a function that gets called when the element is clicked. like so;
//import { Fragment } from 'react';
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  return (
    <>
      <h1>List</h1>
      <ul className="list-group">
        {/* observe that you're mapping each items to a list item 
        each item, we have access to, and is used as a key */}
        {items.map((item, index) => ( //when mapping items, you can optinally pass in the index of the item
          <li
            className="list-group-item"
            key={item}
            onClick={(event) => console.log(item, index, event)} //similarly, onclick can have an event parameter.
          >
            {item}
          </li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
----

when printing out the params, including event param there, we can see in the console an object like this;
A third item 2 
SyntheticBaseEvent {_reactName: 'onClick', _targetInst: null, type: 'click', nativeEvent: PointerEvent, target: li.list-group-item, …}
altKey
: 
false
bubbles
: 
true
button
: 
0
buttons
: 
0
cancelable
: 
true
clientX
: 
158
clientY
: 
148
ctrlKey
: 
false
currentTarget
: 
null
defaultPrevented
: 
false
detail
: 
1
eventPhase
: 
3
getModifierState
: 
ƒ modifierStateGetter(keyArg)
isDefaultPrevented
: 
ƒ functionThatReturnsFalse()
isPropagationStopped
: 
ƒ functionThatReturnsFalse()
isTrusted
: 
true
metaKey
: 
false
movementX
: 
0
movementY
: 
0
nativeEvent
: 
PointerEvent {isTrusted: true, pointerId: 1, width: 1, height: 1, pressure: 0, …}
pageX
: 
158
pageY
: 
148
relatedTarget
: 
null
screenX
: 
1438
screenY
: 
-133
shiftKey
: 
false
target
: 
li.list-group-item
timeStamp
: 
2355.7999999970198
type
: 
"click"
view
: 
Window {window: Window, self: Window, document: document, name: '', location: Location, …}
_reactName
: 
"onClick"
_targetInst
: 
null
[[Prototype]]
: 
Object

SyntheticBaseEvent is a react specific event object that wraps the native event object, and provides additional functionality. it is a cross-browser wrapper for the native event object, and it is used to normalize the event handling in react. it is a synthetic event, because it is not a real event, but a wrapper around the native event object.

also rule of thumbs for funcitons;
for oneliners, it's cool to include in the middle of a jsx thing. but any longer, and it should be moved into a separate function. like so;

more on handling events;
//import { Fragment } from 'react';
import { MouseEvent } from "react";
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  //putting multiline function here rather than nested in the jsx markup
  //but hold up, if we write it like this, we'll get a typescript error
  //const handleClick = (event) => { console.log(event); }; //this is a function that takes an event as a parameter and logs it to the console
  //the error thrown is 'Parameter 'event' implicitly has an 'any' type.ts(7006)', and the reason why is because
  //the typescript compiler doesn't know what type of parametre (event) is being passed in, so it assumes it's of type 'any'
  //which in turn breaks the event. (event + dot) operator. this is a case where we need to be explicit about the type of the parametre(event), in order to still get autocomplete and type checking/safety, which when used in ajsx thing, is (parameter) event: React.MouseEvent<HTMLLIElement, MouseEvent>. and because we are declaring a brand new function, the typescript compiler doesn't know where we're gonna use this function, so it doesn't know what type of event to expect, so we have to be explicit about it.
  //this can be solved by importing mouseevent from react, and then using it as a type for the event parameter, and rewriting the declared function like so;
  const handleClick = (event: MouseEvent) => { console.log(event); }; //this is an event handler, because its job is handling an event, in this case, the click event
  //this is an example of type annoation in typescript. we type annotation and specify the type of our variables, parameters, and functions, so that the typescript compiler can check for errors and give us warnings, and also provide us with autocomplete and intellisense.
  //now with this annotation, if we use the dot operator, we can see all the properties and methods that are available on the (mouse) event object, and we can use them in our function. this is one of the beauties of typescript, and why it's so powerful and useful.
  //we get autocompletion, we get type safety, and it's easier to refactor or restructure our code


  //this logic can also be moved inside a function, and then we can call that function inside the jsx
  const message =
    items.length === 0 ? <p>There are no items in the list.</p> : null; //use ternary operator here to conditionally render a message

  //like so (using arrow function syntax)
  const getMessage = () => {
    return items.length === 0 ? <p>There are no items in the list.</p> : null; //use ternary operator here to conditionally render a message
  };

  // items.map(item => <li>{item}</li>)

  //   //using conditional rendering; if there are no items in the list, we want to display a message
  //   if (items.length === 0)
  //     return (
  //       <>
  //         <h1>List</h1>
  //         { items.length === 0 ? <p>There are no items in the list.</p> : null }
  //         <p>There are no items in the list.</p>;
  //       </>
  //     );

  return (
    //version with fragment
    // <Fragment>
    //     <h1>List</h1>
    //     <ul className="list-group">
    //       <li className="list-group-item">An item</li>
    //       <li className="list-group-item">A second item</li>
    //       <li className="list-group-item">A third item</li>
    //       <li className="list-group-item">A fourth item</li>
    //       <li className="list-group-item">And a fifth one</li>
    //     </ul>
    // </Fragment>

    <>
      {/* //version with empty angle brackets, which tells react to not render anything, but to still return the jsx/ use a fragment to wrap all these children; */}
      {/* <h1>List</h1>
      <ul className="list-group">
        <li className="list-group-item">An item</li>
        <li className="list-group-item">A second item</li>
        <li className="list-group-item">A third item</li>
        <li className="list-group-item">A fourth item</li>
        <li className="list-group-item">And a fifth one</li>
      </ul> */}
      {/* //take each item and convert it to an item of a different type.
  //so here, we wanna convert each item to an li element.
  //and inside the tags we want to render, or display the item itself
  //remember that curly braces {} are used to render data dynamically
  // so here, we render the item itself */}
      <h1>List</h1>
      {/* { items.length === 0 ? <p>There are no items in the list.</p> : null }  {use ternary operator here to conditionally render a message } */}
      {/* {message} */}
      {/* {getMessage() */} {/* example of calling a function */}
      {/*now for a more concise way to write code*/}
      {/*1.) this way is fine, but the null part is problematic, so there is a better way; */}
      {items.length === 0 ? <p>There are no items in the list.</p> : null}
      {/*2.) start the same, but instead of tenerary operator or quesiton mark, use ; a logical AND */}
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* paragraph tag will be returned if true; for false, nothing will be rendered on screen */}
      <ul className="list-group">
        {/* observe that you're mapping each items to a list item 
        each item, we have access to, and is used as a key */}
        {items.map((item, index) => ( //when mapping items, you can optinally pass in the index of the item as a second argument
          <li
            className="list-group-item"
            key={item}
            // onClick={(event) => console.log(item, index, event)} //similarly, onclick can have an event parameter.
            //and when using inline here, the typescript compiler knows that the event is of type React.MouseEvent<HTMLLIElement, MouseEvent>, so doesn't throw an error/give a warning
            //but having declared a function above, we can replace this inline event with the following;
            onClick={handleClick} //note that the function is NOT BEING CALLED, we just want to pass a reference. in essence you are telling react that whenever the user clicks on this event, this function should be caught. so calling this function will be done later at runtime, when the event is triggered
          >
            {item}
          </li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
----

type annotation;
in typescript, we can use type annotations to specify the type of a variable, parameter, or function. this is useful because it allows the typescript compiler to check for errors and give us warnings, and also provide us with autocomplete and intellisense. this is one of the reasons why typescript is so powerful and useful.

Type annotation in TypeScript is a way to explicitly tell the TypeScript compiler what the type of a variable or function parameter should be. This is useful for enhancing code readability and predictability, and it allows the TypeScript compiler to catch potential type-related errors at compile time.

Here's a simple example of type annotation in TypeScript:
let name: 

string = 'John Doe'; // Here, ': string' is the type annotation

In the case of functions, you can annotate both the parameter types and the return type:

function greet(name: string): string {
  return `Hello, ${name}`;
} // ': string' after 'name' is the type annotation for the parameter, and ': string' after the parentheses is the type annotation for the return value

In your provided code, the type annotation is used in the handleClick function:

const handleClick = (event: MouseEvent) => { console.log(event); };

Here, (event: MouseEvent) is telling TypeScript that the event parameter that handleClick receives should be of type MouseEvent. This allows TypeScript to understand the structure of event and provide autocompletion, type checking, and other benefits.

----

note, always pay attention to cleaning up your code.

NOW;
functional hooks (like useState)
React Hooks are a feature introduced in React 16.8 that allow you to use state and other React features without writing a class. They let you "hook into" React state and lifecycle features from functional components. Here are some of the most commonly used hooks:

useState: This hook lets you add state to functional components. It returns a pair: the current state value and a function that lets you update it.

const [count, setCount] = useState(0);

useEffect: This hook lets you perform side effects in functional components. It's a close replacement for componentDidMount, componentDidUpdate, and componentWillUnmount in class components.

useEffect(() => {
  document.title = `You clicked ${count} times`;
}, [count]); // Only re-run the effect if count changes

useContext: This hook lets you subscribe to React context without introducing nesting.

const theme = useContext(ThemeContext);

useReducer: This hook is usually preferable to useState when you have complex state logic that involves multiple sub-values or when the next state depends on the previous one.

const [state, dispatch] = useReducer(reducer, initialArg, init);

useRef: This hook can be used to create mutable variables in functional components, similar to instance variables in class components.

const myRef = useRef(null);

useMemo and useCallback: These hooks are used to optimize performance by memoizing values and functions.

const memoizedValue = useMemo(() => computeExpensiveValue(a, b), [a, b]);
const memoizedCallback = useCallback(() => doSomething(a, b), [a, b]);

Remember, hooks are only available in function components and custom hooks. They cannot be used in class components or in regular JavaScript functions

more on useState hook;

React's useState is a Hook that lets you add state to your functional components. It returns a pair: the current state value and a function that lets you update it.

Here's the basic usage of useState:

const [state, setState] = useState(initialState);

In this code:

initialState is the initial value for the state. It can be of any type: number, string, object, array, etc. The initial state argument is only used during the first render.

state is the current state value. It will be equal to initialState on the first render, and it will hold the updated state value after subsequent re-renders if setState is used.

setState is a function that you can use to update the state. When you call this function with a new value, the component will re-render, and state will be equal to the new value.

Here's an example of useState in action:
import React, { useState } from 'react';

function Counter() {
  const [count, setCount] = useState(0);

  return (
    <div>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}

GitHub Copilot
React's useState is a Hook that lets you add state to your functional components. It returns a pair: the current state value and a function that lets you update it.

Here's the basic usage of useState:

In this code:

initialState is the initial value for the state. It can be of any type: number, string, object, array, etc. The initial state argument is only used during the first render.

state is the current state value. It will be equal to initialState on the first render, and it will hold the updated state value after subsequent re-renders if setState is used.

setState is a function that you can use to update the state. When you call this function with a new value, the component will re-render, and state will be equal to the new value.

Here's an example of useState in action:

In this example, useState is used to keep track of the count state in a Counter component. The count is initially 0, and every time the button is clicked, setCount is called with count + 1, which updates the state and causes the component to re-render with the new count.


here's a massive example of a functional component managing state;
//import { Fragment } from 'react';
import { MouseEvent, useState } from "react";
function ListGroup() {
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  //initialise to -1; this means no item is gonna be selected at the start
  //let selectedIndex = -1;
  //let selectedIndex = 0; //this means the first item is selected at the start
  //but be aware that the variable declared here is in a local scope (this functional component); react is not aware of it, it's like a little secret in this component
  //to solve this problem, you need to tell react  that this component is going to have data or a state that might change over time.
  //to do so, you have to use one of the built in functions in react called use state

  //this a function called a hook, and it's a way to hook into the react lifecycle. a hook is a function that allows us to tap into built in react features
  //this is the state hook, and it allows us to add data or state to our functional components, that will change over time
  //so instead of declaring a variable like this;
  //let selectedIndex = 0; which is local in scope to this component
  //you'll call this function, by first initinig it;
//const arr = useState(-1); //returns an array
//this will have two elements;
//arr[0] //variable (selectedIndex)
//arr[1] //updater function. using the updater function, we can update this variable. and at that point, react will be notified. so it knows that the state of our component is changed. and it will re-render the component, and update the ui to reflect the new state, and causes the dom to be updated under the hood

//.......but instead of working with individual array elements, it is easier to destructure the array.
//like so;
const [selectedIndex, setSelectedIndex] = useState(-1); //this is a destructured array, and it's a common pattern in react to destructure arrays or objects, so that you can work with the individual elements directly, without having to access them through the array index
//first element is a variable (state variable)
//second element is a function
//this is a common pattern in react, and it's called array destructuring
//all this to say, this is how we tell react that a component can have state that changes over time, and we can use the state hook to add state to our functional components



//another example can be;
const [name, setName] = useState(''); //this is a string, and it's empty at the start


  //putting multiline function here rather than nested in the jsx markup
  //but hold up, if we write it like this, we'll get a typescript error
  //const handleClick = (event) => { console.log(event); }; //this is a function that takes an event as a parameter and logs it to the console
  //the error thrown is 'Parameter 'event' implicitly has an 'any' type.ts(7006)', and the reason why is because
  //the typescript compiler doesn't know what type of parametre (event) is being passed in, so it assumes it's of type 'any'
  //which in turn breaks the event. (event + dot) operator. this is a case where we need to be explicit about the type of the parametre(event), in order to still get autocomplete and type checking/safety, which when used in ajsx thing, is (parameter) event: React.MouseEvent<HTMLLIElement, MouseEvent>. and because we are declaring a brand new function, the typescript compiler doesn't know where we're gonna use this function, so it doesn't know what type of event to expect, so we have to be explicit about it.
  //this can be solved by importing mouseevent from react, and then using it as a type for the event parameter, and rewriting the declared function like so;
  const handleClick = (event: MouseEvent) => { console.log(event); }; //this is an event handler, because its job is handling an event, in this case, the click event
  //this is an example of type annoation in typescript. we type annotation and specify the type of our variables, parameters, and functions, so that the typescript compiler can check for errors and give us warnings, and also provide us with autocomplete and intellisense.
  //now with this annotation, if we use the dot operator, we can see all the properties and methods that are available on the (mouse) event object, and we can use them in our function. this is one of the beauties of typescript, and why it's so powerful and useful.
  //we get autocompletion, we get type safety, and it's easier to refactor or restructure our code


  //this logic can also be moved inside a function, and then we can call that function inside the jsx
  const message =
    items.length === 0 ? <p>There are no items in the list.</p> : null; //use ternary operator here to conditionally render a message

  //like so (using arrow function syntax)
  const getMessage = () => {
    return items.length === 0 ? <p>There are no items in the list.</p> : null; //use ternary operator here to conditionally render a message
  };

  // items.map(item => <li>{item}</li>)

  //   //using conditional rendering; if there are no items in the list, we want to display a message
  //   if (items.length === 0)
  //     return (
  //       <>
  //         <h1>List</h1>
  //         { items.length === 0 ? <p>There are no items in the list.</p> : null }
  //         <p>There are no items in the list.</p>;
  //       </>
  //     );

  return (
    //version with fragment
    // <Fragment>
    //     <h1>List</h1>
    //     <ul className="list-group">
    //       <li className="list-group-item">An item</li>
    //       <li className="list-group-item">A second item</li>
    //       <li className="list-group-item">A third item</li>
    //       <li className="list-group-item">A fourth item</li>
    //       <li className="list-group-item">And a fifth one</li>
    //     </ul>
    // </Fragment>

    <>
      {/* //version with empty angle brackets, which tells react to not render anything, but to still return the jsx/ use a fragment to wrap all these children; */}
      {/* <h1>List</h1>
      <ul className="list-group">
        <li className="list-group-item">An item</li>
        <li className="list-group-item">A second item</li>
        <li className="list-group-item">A third item</li>
        <li className="list-group-item">A fourth item</li>
        <li className="list-group-item">And a fifth one</li>
      </ul> */}
      {/* //take each item and convert it to an item of a different type.
  //so here, we wanna convert each item to an li element.
  //and inside the tags we want to render, or display the item itself
  //remember that curly braces {} are used to render data dynamically
  // so here, we render the item itself */}
      <h1>List</h1>
      {/* { items.length === 0 ? <p>There are no items in the list.</p> : null }  {use ternary operator here to conditionally render a message } */}
      {/* {message} */}
      {/* {getMessage() */} {/* example of calling a function */}
      {/*now for a more concise way to write code*/}
      {/*1.) this way is fine, but the null part is problematic, so there is a better way; */}
      {items.length === 0 ? <p>There are no items in the list.</p> : null}
      {/*2.) start the same, but instead of tenerary operator or quesiton mark, use ; a logical AND */}
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* paragraph tag will be returned if true; for false, nothing will be rendered on screen */}
      <ul className="list-group">
        {/* observe that you're mapping each items to a list item 
        each item, we have access to, and is used as a key */}
        {items.map((item, index) => ( //when mapping items, you can optinally pass in the index of the item as a second argument
          <li
            className={ selectedIndex === index ? "list-group-item active" : "list-group-item"}
            key={item}
            //writing a simple error function
            onClick={() => { setSelectedIndex(index); }}

            //older notes, don't remove
            // onClick={(event) => console.log(item, index, event)} //similarly, onclick can have an event parameter.
            //and when using inline here, the typescript compiler knows that the event is of type React.MouseEvent<HTMLLIElement, MouseEvent>, so doesn't throw an error/give a warning
            //but having declared a function above, we can replace this inline event with the following;
            //onClick={handleClick} //note that the function is NOT BEING CALLED, we just want to pass a reference. in essence you are telling react that whenever the user clicks on this event, this function should be caught. so calling this function will be done later at runtime, when the event is triggered
          >
            {item}
          </li>
        ))}
      </ul>
    </>
  );
}

export default ListGroup;
---------


REMINDER; that with react, we almost never have to touch the dom directly
we think in terms of components that have state, when the state of a component changes, react will automatically update the dom to reflect that change / match the new component statw. this is the power of react, and why it's so popular and widely used.

and a note about components in react;
each component will have its own state. this allows multiple reuses of the same components of a page to be independent of each other

NOW; props
props, or properties, are the inputs to our components. they are used to pass data

props are used to pass data from a parent component to a child component. they are similar to function arguments in regular javascript functions. props are read-only, which means that a child component cannot modify the props it receives from its parent. this makes components more predictable and easier to debug.

-- typescript interface feature
Interfaces in TypeScript are a powerful way to define contracts within your code and they define the syntax for classes to follow. They are used to define the shape of an object, function, class, or even primitive types. They are purely a TypeScript feature and do not have a JavaScript equivalent.

Here's a simple example of an interface in TypeScript:
interface User {
  name: string;
  age: number;
}

In this example, User is an interface that represents an object with a name property of type string and an age property of type number.

You can use this interface to type-check objects:
const user: User = {
  name: 'John Doe',
  age: 25,
};

In this case, if you try to assign an object to `user` that doesn't conform to the `User` interface, TypeScript will give a compile-time error.

Interfaces can also be used to define the shape of a function:

interface GreetFunction {
  (name: string): string;
}

const greet: GreetFunction = (name) => {
  return `Hello, ${name}`;
};

In this example, GreetFunction is an interface that represents a function that takes a name parameter of type string and returns a string.

Interfaces are a fundamental part of TypeScript and are key to taking full advantage of the type-checking capabilities the language provides.

largeee example of passing data via props.
using;
ListGroup.tsx react component
//import { Fragment } from 'react';
import { MouseEvent, useState } from "react";
//so, let's use props. instead of defining a list of items here, we'll pass them in as props, an input to this component. just like how we can call a function and give it an argument.
//so first, decide the shape of the input to this component
//so let's pass an object with two properties;
// { items: [], heading: string }
// to do this, we use the typescript feature called interfaces

//convention is to use props, or prefix with name of the component
//interface Props
interface ListGroupProps {
  //now define various properties and their types
  items: string[]; //array of strings
  heading: string; //string
  //this is an example of type annotation
}

//now, we can use this interface to type our props
//but now, typescript compiler will remind us if, like in app,tsxm when we use this component, if we forget to pass in the required props, or if we pass in the wrong type of props
//function ListGroup(ListGroupProps: ListGroupProps) {
//....and this is a nice way, but we can destructure it, giving us access to the individual properties anywhere within this function scope;
function ListGroup({ items, heading }: ListGroupProps) {
  //initialise to -1; this means no item is gonna be selected at the start
  //let selectedIndex = -1;
  //let selectedIndex = 0; //this means the first item is selected at the start
  //but be aware that the variable declared here is in a local scope (this functional component); react is not aware of it, it's like a little secret in this component
  //to solve this problem, you need to tell react  that this component is going to have data or a state that might change over time.
  //to do so, you have to use one of the built in functions in react called use state

  //this a function called a hook, and it's a way to hook into the react lifecycle. a hook is a function that allows us to tap into built in react features
  //this is the state hook, and it allows us to add data or state to our functional components, that will change over time
  //so instead of declaring a variable like this;
  //let selectedIndex = 0; which is local in scope to this component
  //you'll call this function, by first initinig it;
  //const arr = useState(-1); //returns an array
  //this will have two elements;
  //arr[0] //variable (selectedIndex)
  //arr[1] //updater function. using the updater function, we can update this variable. and at that point, react will be notified. so it knows that the state of our component is changed. and it will re-render the component, and update the ui to reflect the new state, and causes the dom to be updated under the hood

  //.......but instead of working with individual array elements, it is easier to destructure the array.
  //like so;
  //const [selectedIndex, setSelectedIndex] = useState(-1); //this is a destructured array, and it's a common pattern in react to destructure arrays or objects, so that you can work with the individual elements directly, without having to access them through the array index
  //first element is a variable (state variable)
  //second element is a function
  //this is a common pattern in react, and it's called array destructuring
  //all this to say, this is how we tell react that a component can have state that changes over time, and we can use the state hook to add state to our functional components

  //another example can be;
  const [name, setName] = useState(""); //this is a string, and it's empty at the start

  return (
    <>
      {/* //version with empty angle brackets, which tells react to not render anything, but to still return the jsx/ use a fragment to wrap all these children; */}

      {/* //take each item and convert it to an item of a different type.
  //so here, we wanna convert each item to an li element.
  //and inside the tags we want to render, or display the item itself
  //remember that curly braces {} are used to render data dynamically in jsx */}
      <h1>{heading}</h1>
      {/* {message} */}
      {/* {getMessage() */} {/* example of calling a function */}
      {/*now for a more concise way to write code*/}
      {/*1.) this way is fine, but the null part is problematic, so there is a better way; */}
      {/* {items.length === 0 ? <p>There are no items in the list.</p> : null} */}
      {/* now that props are being used, prefix items with props name to access .. but this can get repetitive. so solution is to destructure this parametre in the ListGroup function signature above.*/}
      {/* {ListGroupProps.items.length === 0 ? <p>There are no items in the list.</p> : null} */}
      {items.length === 0 ? <p>There are no items in the list.</p> : null}
      {/*2.) start the same, but instead of tenerary operator or quesiton mark, use ; a logical AND */}
      {/* {items.length === 0 && <p>There are no items in the list.</p>}{" "} */}
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* paragraph tag will be returned if true; for false, nothing will be rendered on screen */}
      <ul className="list-group">
        {/* observe that you're mapping each items to a list item 
        each item, we have access to, and is used as a key */}
        {items.map(
          (
            item,
            index //when mapping items, you can optinally pass in the index of the item as a second argument
          ) => (
            <li
              className={
                selectedIndex === index
                  ? "list-group-item active"
                  : "list-group-item"
              }
              key={item}
              //writing a simple error function
              onClick={() => {
                setSelectedIndex(index);
              }}
            >
              {item}
            </li>
          )
        )}
      </ul>
    </>
  );
}

export default ListGroup;

...and using this functional component in App.tsx, where we pass in props ;
//initial state
// import { useState } from 'react'
// import reactLogo from './assets/react.svg'
// import './App.css'

// function App() {
//   const [count, setCount] = useState(0)

//   return (
//     <div className="App">
//       <div>
//         <a href="https://vitejs.dev" target="_blank">
//           <img src="/vite.svg" className="logo" alt="Vite logo" />
//         </a>
//         <a href="https://reactjs.org" target="_blank">
//           <img src={reactLogo} className="logo react" alt="React logo" />
//         </a>
//       </div>
//       <h1>Vite + React</h1>
//       <div className="card">
//         <button onClick={() => setCount((count) => count + 1)}>
//           count is {count}
//         </button>
//         <p>
//           Edit <code>src/App.tsx</code> and save to test HMR
//         </p>
//       </div>
//       <p className="read-the-docs">
//         Click on the Vite and React logos to learn more
//       </p>
//     </div>
//   )
// }

// export default App

//my version;
//import Message from "./Message";
import ListGroup from "./components/ListGroup";

function App() {
  // return <div><Message /></div>
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  return (
    <div>
        {/* <Message /> */}
        {/* now to pass in props. also possible to wrap in braces. unnecessary for static values */}
      <ListGroup items={items} heading="Cities" />
    </div>
  );
}

export default App;

now; remember, we want to make components reuseable
keep logic out of reuseable components, and keep them as simple as possible
so we will need a mechanism to notify the consumer, or the parent of this component that an item is selected. in the case above, the parent or consumer of the component is App.tsx - where the ListGroup is used. so when an item on the frontend is selected, we should notify the app component that an item is selected.

implementation is as follows, which also features a function passed as a prop;
ListGroup.tsx --
//import { Fragment } from 'react';
import { MouseEvent, useState } from "react";
//so, let's use props. instead of defining a list of items here, we'll pass them in as props, an input to this component. just like how we can call a function and give it an argument.
//so first, decide the shape of the input to this component
//so let's pass an object with two properties;
// { items: [], heading: string }
// to do this, we use the typescript feature called interfaces

//convention is to use props, or prefix with name of the component
//interface Props
interface ListGroupProps {
  //now define various properties and their types
  items: string[]; //array of strings
  heading: string; //string
  //this is an example of type annotation
  //we can also add a function, something like;
 // (item: string) => void
 //so by convention, we'd do something like 
 onSelectItem: (item: string) => void;
 //this is a property, the type of which is a function, that takes a string as a parameter, and returns void
}

//now, we can use this interface to type our props
//but now, typescript compiler will remind us if, like in app,tsxm when we use this component, if we forget to pass in the required props, or if we pass in the wrong type of props
//function ListGroup(ListGroupProps: ListGroupProps) {
//....and this is a nice way, but we can destructure it, giving us access to the individual properties anywhere within this function scope;
//now also added onSelectItem prop, which is a function
function ListGroup({ items, heading, onSelectItem }: ListGroupProps) {
  //initialise to -1; this means no item is gonna be selected at the start
  //let selectedIndex = -1;
  //let selectedIndex = 0; //this means the first item is selected at the start
  //but be aware that the variable declared here is in a local scope (this functional component); react is not aware of it, it's like a little secret in this component
  //to solve this problem, you need to tell react  that this component is going to have data or a state that might change over time.
  //to do so, you have to use one of the built in functions in react called use state

  //this a function called a hook, and it's a way to hook into the react lifecycle. a hook is a function that allows us to tap into built in react features
  //this is the state hook, and it allows us to add data or state to our functional components, that will change over time
  //so instead of declaring a variable like this;
  //let selectedIndex = 0; which is local in scope to this component
  //you'll call this function, by first initinig it;
  //const arr = useState(-1); //returns an array
  //this will have two elements;
  //arr[0] //variable (selectedIndex)
  //arr[1] //updater function. using the updater function, we can update this variable. and at that point, react will be notified. so it knows that the state of our component is changed. and it will re-render the component, and update the ui to reflect the new state, and causes the dom to be updated under the hood

  //.......but instead of working with individual array elements, it is easier to destructure the array.
  //like so;
  const [selectedIndex, setSelectedIndex] = useState(-1); //this is a destructured array, and it's a common pattern in react to destructure arrays or objects, so that you can work with the individual elements directly, without having to access them through the array index
  //first element is a variable (state variable)
  //second element is a function
  //this is a common pattern in react, and it's called array destructuring
  //all this to say, this is how we tell react that a component can have state that changes over time, and we can use the state hook to add state to our functional components

  //another example can be;
  const [name, setName] = useState(""); //this is a string, and it's empty at the start

  return (
    <>
      <h1>{heading}</h1>
      {items.length === 0 ? <p>There are no items in the list.</p> : null}
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* paragraph tag will be returned if true; for false, nothing will be rendered on screen */}
      <ul className="list-group">
        {/* observe that you're mapping each items to a list item 
        each item, we have access to, and is used as a key */}
        {items.map(
          (
            item,
            index //when mapping items, you can optinally pass in the index of the item as a second argument
          ) => (
            <li
              className={
                selectedIndex === index
                  ? "list-group-item active"
                  : "list-group-item"
              }
              key={item}
              //writing a simple error function
              onClick={() => {
                setSelectedIndex(index);
                onSelectItem(item); //calling the function that was passed in as a prop from the parent component (app.tsx), where listgroup is being used
              }}
            >
              {item}
            </li>
          )
        )}
      </ul>
    </>
  );
}

export default ListGroup;
---

and 
app.tsx
//initial state
// import { useState } from 'react'
// import reactLogo from './assets/react.svg'
// import './App.css'

// function App() {
//   const [count, setCount] = useState(0)

//   return (
//     <div className="App">
//       <div>
//         <a href="https://vitejs.dev" target="_blank">
//           <img src="/vite.svg" className="logo" alt="Vite logo" />
//         </a>
//         <a href="https://reactjs.org" target="_blank">
//           <img src={reactLogo} className="logo react" alt="React logo" />
//         </a>
//       </div>
//       <h1>Vite + React</h1>
//       <div className="card">
//         <button onClick={() => setCount((count) => count + 1)}>
//           count is {count}
//         </button>
//         <p>
//           Edit <code>src/App.tsx</code> and save to test HMR
//         </p>
//       </div>
//       <p className="read-the-docs">
//         Click on the Vite and React logos to learn more
//       </p>
//     </div>
//   )
// }

// export default App

//my version;
//import Message from "./Message";
import ListGroup from "./components/ListGroup";

function App() {
  // return <div><Message /></div>
  let items = [
    "An item",
    "A second item",
    "A third item",
    "A fourth item",
    "And a fifth one",
  ];

  //passing a item function as a prop
  const handleSelectItem = (item: string) => {
    console.log(item);
  }

  return (
    <div>
        {/* <Message /> */}
        {/* now to pass in props. also possible to wrap in braces. unnecessary for static values */}
      <ListGroup items={items} heading="Cities" onSelectItem={handleSelectItem} />
    </div>
  );
}

export default App;

TIP--
go to view -> problems, and it will show if the typescript compiler has spotted any issues

now, to discuss the differences between props and state;
--props;
* the inputs, or argumets, passed to a component. 
* they are immutable (unchangeable), which means that a component cannot modify the props it receives from its parent. this makes components more predictable and easier to debug. props are used to pass data from a parent component to a child component.
* are like function arguments

-- state;
* the internal data managed by a component that can change over time
* state is mutable (changeable), which means that a component can modify its own state. this allows components to be dynamic and interactive. this is the whole purpose of using state variables. we want to tell react that this component has data that can change over time
* state is like local variables inside a function

handy;
with es7+ react extension installed, you can type 'rafce' in vs code/ visual studio code and it will automatically generate a functional component for you, or at least a boilerplate / template. it is short for react arrow function component export.


now, how to create a react component that accepts children;
* children are a special prop that allows you to pass components or elements as children to another component. this is useful for creating reusable components that can render different content based on the children passed to them.

there is a special prop that all components support, called children. this prop allows you to pass components or elements as children to another component. this is useful for creating reusable components that can render different content based on the children passed to them.

for example;
in Alert.tsx

//we wanna make the text of the alert component dynamic, so we will pass the text as a prop to the component, using a interface to define the prop type and shape

interface AlertProps {
    children: string; // special prop that all components support, which allows us to pass text as children to this component when Alert is used in App.tsx
}

//remember to destructure
const Alert = ({ children }: AlertProps) => {
  return (
    <div className="alert alert-primary">{children}</div>
  )
}

export default Alert
----

and used in App.tsx ;
import Alert from "./components/Alert";


function App() {
  
  return (
    <div>
     <Alert children="Hello world." />
    </div>
  );
}

export default App;
---

..but what if we wanted to pass html content? it will give a compilation error, because we had told the typescript compiler that the children prop is a string, and it's expecting a string, but we are passing html content, a more complex structure. so to fix this, we need to change the type of children from string to ReactNode, which is a type provided by react that allows us to pass any type of content as children to a component.

more on children in react;
In React, children is a special prop that allows components to be passed as data to other components, providing a way to use composition of components. It's used to pass elements from parent components to child components.

This is useful for creating reusable components. For example, you might have a Card component that has some styling and layout code. You can use the children prop to pass in the content of the card, making the Card component reusable with different content.

Here's an example:
function Card({ children }) {
  return <div className='card'>{children}</div>;
}

function App() {
  return (
    <Card>
      <h2>Hello, world!</h2>
      <p>Welcome to my app.</p>
    </Card>
  );
}

In this example, the Card component is a reusable component that takes in children as a prop. In the App component, we use the Card component and pass in some JSX as its children. The Card component then renders its children inside a div with a class of 'card'. This way, we can reuse the Card component with different content.

and here is our own example of using reactnode, to pass a complex type (html) as children to a component;

defined in Alert.tsx

//we wanna make the text of the alert component dynamic, so we will pass the text as a prop to the component, using a interface to define the prop type and shape

interface AlertProps {
    //children: string; // special prop that all components support, which allows us to pass text as children to this component when Alert is used in App.tsx.

    //using string works, but there is a better way; ReactNode
    children: React.ReactNode; //this allows us to pass any type of data as children to the component
}

//remember to destructure
const Alert = ({ children }: AlertProps) => {
  return (
    <div className="alert alert-primary">{children}</div>
  )
}

export default Alert
------

....and used in App.tsx
import Alert from "./components/Alert";

function App() {
  return (
    <div>
      {/* passing just a string */}
      {/* <Alert children="Hello world." /> */}
      {/* but now, with reactnode in use, we can pass a complex type */}
      {/* better to include components like this */}
      <Alert>
        Hello <span>world.</span>
      </Alert>
    </div>
  );
}

export default App;

----

REMEMBER to use react devtools in the browser to inspect the components and see the props and state of each component. it will also show the component tree that react takes and renders to the dom.

in interfaces, a union type is;
In the React context, a union type allows a prop to accept multiple specific types rather than a single type. This is particularly useful for props that can have a limited set of acceptable values. In the provided Button.tsx example, the colour prop in the ButtonProps interface uses a union type to restrict its possible values to either 'primary' or 'secondary'. This ensures that when a <Button /> component is used within a React application, it can only receive colour props that match one of these two specified values, enhancing type safety and predictability in the component's behavior.

Here's a breakdown of the union type in the given context:

Union Type ('primary' | 'secondary'): This syntax specifies that the colour prop can only be one of two string literals: 'primary' or 'secondary'. It's a way to enforce that the prop adheres to a specific set of values, which can be useful for component styling or behavior that depends on a limited set of options.

Optional Prop (colour?): The question mark (?) after the prop name indicates that the colour prop is optional. This means a <Button /> component can be used with or without specifying a colour prop. If the colour prop is provided, it must be either 'primary' or 'secondary'.

React Context: In the broader context of a React application, using union types for props like this can help ensure components are used correctly throughout the application. It provides clear documentation through code about what values a prop can accept, making the component's API easier to understand and use. Additionally, it allows TypeScript to catch errors during development if a component receives a prop value outside of the specified union, reducing runtime errors and improving the overall quality of the application.

This approach leverages TypeScript's static type checking to enhance code quality and developer experience in React applications.

an example of a union type in typescript is;
Button.tsx

interface ButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  //colour?: string; //question mark denotes optional property
  //add as many bootstrap colours here as intended
  //this implementation restricts the colour prop to one of these
  colour?: 'primary' | 'secondary' | 'danger'; //uit's called a union type
}

const Button = ( { children, onClick, colour = 'primary'}: ButtonProps ) => {
  return (
    <button className={"btn btn-" + colour} onClick={onClick}>{children}</button>
  )
}

export default Button
-------

and in App.tsx;
import Alert from "./components/Alert";
import Button from "./components/Button";

function App() {
  return (
    <div>
      {/* passing just a string */}
      {/* <Alert children="Hello world." /> */}
      {/* but now, with reactnode in use, we can pass a complex type */}
      {/* better to include components like this */}
      <Alert>
        Hello <span>world.</span>
      </Alert>
      <Button  onClick={() => console.log('Clicked!!!')}>Press the button.</Button>
    </div>
  );
}

export default App;
------

more, demonstrating how to use a union type in typescript, to make an alert appear and disappear with buttons;
alert component Alert.tsx ;
//we wanna make the text of the alert component dynamic, so we will pass the text as a prop to the component, using a interface to define the prop type and shape

interface AlertProps {
  //children: string; // special prop that all components support, which allows us to pass text as children to this component when Alert is used in App.tsx.

  //using string works, but there is a better way; ReactNode
  children: React.ReactNode; //this allows us to pass any type of data as children to the component
  onClose: () => void;
}

//remember to destructure
const Alert = ({ children, onClose }: AlertProps) => {
  return (
    <div className="alert alert-primary alert-dismissible fade show">
      {children}
      {/* REMEMBER the onClose function, we are just passing the onClose function through, NOT INVOKING IT RIGHT NOW. only when the user clicks on the close button will react call the function for us */}
      <button type="button" onClick={onClose} className="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
    </div>
  );
};

export default Alert;
-----

button component Button.tsx ;

interface ButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  //colour?: string; //question mark denotes optional property
  //add as many bootstrap colours here as intended
  //this implementation restricts the colour prop to one of these
  colour?: 'primary' | 'secondary' | 'danger'; //it's called a union type
}

const Button = ( { children, onClick, colour = 'primary'}: ButtonProps ) => {
  return (
    <button className={"btn btn-" + colour} onClick={onClick}>{children}</button>
  )
}

export default Button
-----

and now, how they're used in App.tsx ;
import { useState } from "react";
import Alert from "./components/Alert";
import Button from "./components/Button";

function App() {
  //determine the initial state of the alert (initially hidden)
  const [alertVisible, setAlertVisibility] = useState(false); //this returns an array that we can destructure

  return (
    <div>
      {/* passing just a string */}
      {/* <Alert children="Hello world." /> */}
      {/* but now, with reactnode in use, we can pass a complex type */}
      {/* don't forget to set an error function here */}
      { alertVisible && <Alert onClose={() => setAlertVisibility(false)}> My alert message.</Alert> }
      <Button colour="danger" onClick={() => setAlertVisibility(true)}>
        Press the button.
      </Button>
    </div>
  );
}

export default App;


a note about App.tsx and how multiple react apps can be included in something like a larael project;
GitHub Copilot
No, App.tsx is not the only entry point for all React apps. It is a common convention in React projects, especially those created with Create React App, to have a single App component that serves as the root component. However, you can structure your React application differently if needed.

You can indeed have multiple React apps within a single project, including a Laravel project. Each React app would have its own root component and could be mounted to a different element in the DOM. This is useful if you want to enhance different parts of a traditional server-rendered app with React or if you have multiple independent React applications within a single project.

Here's a simplified example of how you might include multiple React apps in a Laravel project:

Create Multiple Entry Points: In your React project, create separate entry points for each app. For example, you might have App1.tsx and App2.tsx as the root components for your two React apps.

Compile Your React Apps: Use a bundler like Webpack to compile each of your React apps into separate bundles. Configure your bundler to have multiple entry points corresponding to each of your React apps.

Include the Compiled Scripts in Your Laravel Views: In your Laravel project, include the compiled JavaScript files for your React apps in the appropriate blade templates. You can use Laravel Mix to manage and compile your assets, including React apps, for a smoother integration.

Mount Each App to a Different DOM Element: In the HTML that's rendered by Laravel (e.g., in your blade templates), include different div elements with unique IDs where you want your React apps to mount. Then, in each React app's entry point, use ReactDOM.render to mount the app to the corresponding element.

<!-- In a Laravel blade template -->
<div id="react-app-1"></div>
<div id="react-app-2"></div>

// In App1.tsx
ReactDOM.render(<App1 />, document.getElementById('react-app-1'));

// In App2.tsx
ReactDOM.render(<App2 />, document.getElementById('react-app-2'));

This setup allows you to enhance different parts of your Laravel application with React, giving you the flexibility to use React as much or as little as you need. It's a common approach for integrating React into existing server-rendered applications.

programming concept of cohesion----
Cohesion is a programming concept that refers to how closely related and focused the responsibilities of a module or component are. High cohesion means that a module or component has a single, well-defined purpose and performs a specific set of related tasks. Low cohesion, on the other hand, means that a module or component has multiple unrelated responsibilities and performs a variety of tasks that are not closely related.

aka., things that are related should be together, and things that are not related should be apart. this is a key principle in software design and architecture, as it helps to create more maintainable, understandable, and scalable code.

CSS TIME!!!!
what is a barrel file in typescript?
A barrel file in TypeScript (and by extension, in projects like React that use TypeScript) is a way to simplify imports from multiple modules by re-exporting all of their exports from a single place. Essentially, it's a file (commonly named index.ts or index.tsx) that consolidates exports of other files within the same directory. This allows you to import multiple modules from a single location, making your import statements cleaner and more manageable.

For example, if you have several components in a folder and you want to import them elsewhere in your project, instead of importing each component individually like this:
import ComponentA from './components/ComponentA';
import ComponentB from './components/ComponentB';
import ComponentC from './components/ComponentC';

you can create a barrel file named index.ts inside the components folder that re-exports all of these components

// In components/index.ts
export { default as ComponentA } from './ComponentA';
export { default as ComponentB } from './ComponentB';
export { default as ComponentC } from './ComponentC';

Then, you can import the components with a single line in your other files:

import { ComponentA, ComponentB, ComponentC } from './components';

This approach not only makes your import statements more concise but also helps with maintaining your imports when you add or remove exported members in your modules.


barrel file; index.ts
import ListGroup from "./ListGroup";

export default ListGroup;

and it can be used like this in App.tsx;
import ListGroup from "./components/ListGroup"; //with this, by default, the compiler will look for an index.ts file in the folder, and if it finds one, it will use that file as the default export

css module;
a css file in which all class names and animation names are scoped locally by default. this means that class names are unique to the component, and not global. this is useful for avoiding naming conflicts and making styles more maintainable. this helps avoid clashes
makes styles be

REMEMBER ESPECIALLY hyphenated class names in css are not valid in javascript and typescript, so you can't use them in react. so instead of using hyphenated class names, use camel case. so instead of using something like;

and as part of bundling our application, vite will take all the css modules and create a unique class name for each of them. it'll then bundle them into a single css file, which will be included in the final build of our application. this is done automatically by vite, so we don't have to worry about it.

such as, in a fragment, you can return;
<>
<ul className="styles['list-group']">  {/*this way, but it ugly */} </ul>
<ul className='{styles.listGroup}>  {/*so use camel notation instead */} </ul>
</>

multiple styles, can use this-- put into an array then join
<ul className={[styles.listGroup, styles.container].join(' ')}>

CSS IN JS
this is another approach to css.
it involves writing css styles directly in javascript files, using a library like styled-components or emotion. this allows you to define styles as javascript objects or template literals, and then use them as components in your react application. this approach can make it easier to manage styles, especially when working with component-based architectures.

this requires a new package called
npm install styled-components

and an example of styled components used is this here, in ListGroup.tsx;
import styles from "./ListGroup.module.css";
import { useState } from "react";
import styled from "styled-components";

//creating a styled component
//this replaces regular html elements with a styled component
//components like these allow for the grouping of styles, and can be reused
const List = styled.ul`
  list-style-type: none;
  padding: 0;
  margin: 0;
`;

//the li styled component needs its own shape / interface
interface ListItemProps {
  active: boolean;
}

//pass the interface that represents the shape of the props to the styled component
const ListItem = styled.li<ListItemProps>`
  padding: 10px;
  cursor: pointer;
  background: ${(props) =>
    props.active ? "blue" : "none"}; // Adjusted to use transient prop
`;

interface ListGroupProps {
  //now define various properties and their types
  items: string[]; //array of strings
  heading: string; //string
  //this is an example of type annotation
  //we can also add a function, something like;
  // (item: string) => void
  //so by convention, we'd do something like
  onSelectItem: (item: string) => void;
  //this is a property, the type of which is a function, that takes a string as a parameter, and returns void
}

function ListGroup({ items, heading, onSelectItem }: ListGroupProps) {
  const [selectedIndex, setSelectedIndex] = useState(0);
  // const [name, setName] = useState("");

  return (
    <>
      <h1>{heading}</h1>
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* this html element........ */}
      {/* <ul className={[styles.listGroup, styles.container].join(' ')}>  so use camel notation instead */}
      {/* ...will be replaced with a styled component */}
      <List className={styles.listGroup}>
        {/* observe that you're mapping each items to a list item 
        each item, we have access to, and is used as a key */}
        {items.map(
          (
            item,
            index //when mapping items, you can optinally pass in the index of the item as a second argument
          ) => (
            // <li
            //once more regular html elements are replaced with styled components
            <ListItem
              active={index === selectedIndex}
              key={item}
              //writing a simple error function
              onClick={() => {
                setSelectedIndex(index);
                onSelectItem(item); //calling the function that was passed in as a prop from the parent component (app.tsx), where listgroup is being used
              }}
            >
              {item}
              {/* </li> */}
            </ListItem>
          )
        )}
        {/* </ul> */}
      </List>
    </>
  );
}

export default ListGroup;

----
template literals in javascript;
A template literal in JavaScript is a way to output variables in a string. It allows for embedded expressions, which can include variables, operations, and function calls. Template literals are enclosed by backticks (`) instead of single or double quotes, and expressions within the string are identified by the dollar sign and curly braces (${expression}).

Here's a basic example:
const name = "John";
const greeting = `Hello, ${name}!`;
console.log(greeting); // Output: Hello, John!

Template literals can span multiple lines, which makes creating multi-line strings easier:
const item = "coffee";
const price = 3.5;
const multiLineString = `This is a multi-line string:
The price of ${item} is $${price}.`;
console.log(multiLineString);

They can also include expressions:
Template literals provide a more readable and convenient syntax for creating complex strings and can be particularly useful in generating dynamic content.

NOW -- separating concerns in react;
divide a program into distinct sections where each section handles a specific functionality or aspect of the program. this makes the code easier to understand, maintain, and test. in react, this can be achieved by separating concerns into components, hooks, and other reusable units of code.

this will allow apps to be;
- modular
- easier to understand
- easier to maintain
- easier to modify

modularity in particular brings a numbe rof benefits. if you have a large application, it can be difficult to manage all the code in a single file. by breaking the code into smaller, more manageable pieces, you can make it easier to understand and maintain. it also makes it easier to reuse code, as you can use the same components in multiple places in your application.

if our apps are modular, we can build and test thes modules independently, which makes it easier to identify and fix bugs. it also makes it easier to add new features to the app, as you can add new modules without having to change the existing code.

each module should have a single responsibility (like chefs in a kitchen), and should be loosely coupled with other modules. this means that each module should do one thing, and do it well. it should not depend on other modules, and should be easy to replace or modify without affecting other parts of the app.

in a module, all the complexity and implementation details are hidden behind a well-defined interface (like the remote control of a tv). this makes it easier to understand and use the module, as you only need to know how to interact with the interface, and not how the module works internally.

inline css styles --
not recommended in react. save for very rare / special occasions
inline styling uses camel case, and not hyphenated class names
example;
{/* this line also features use of inline css styling - but use sparingly */}
<List className={styles.listGroup} style={{ backgroundColor: 'greenyellow' }}/>
BUT ONLY USE AS LAST RESORT

material ui - a popular library, built by google, for react that provides pre-built components and styles that you can use in your applications. it's a great way to quickly build user interfaces without having to write a lot of css code. it also provides a theming system that allows you to customize the look and feel of your app.

tailwind - a utility-first css framework that provides a set of pre-built utility classes that you can use to style your components. it's a great way to quickly style your app without having to write a lot of custom css code by hand. it also provides a theming system that allows you to customize the design of your app.

daisy ui - a collection of components and styles, similar to bootstrap , built on top of tailwind css. it provides a set of pre-built components that you can use to quickly build user interfaces. it also provides a theming system that allows you to customize the design of your app.

chakra ui - a simple, modular and accessible component library, built on top of tailwind, that provides a set of pre-built components and styles that you can use to quickly build user interfaces. it's built on top of styled-components and emotion, and provides a theming system that allows you to customize the design of your app.

now; react icons;
react-icons is a popular library that provides a set of pre-built icons that you can use in your react applications. it includes a wide range of icons from popular icon libraries like font awesome, material icons, and many others. you can easily include these icons in your app by importing them as react components.
example usage;
import { FaCalendarWeek } from "react-icons/fa";

function App() {
  return (
    <div>
      <FaCalendarWeek color="red" size="40"/>
    </div>
  );
}

export default App;
-------

exercise, using css in jss styling;
ButtonFancy.tsx ;
import styles from './ButtonFancy.module.css'

interface ButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  //colour?: string; //question mark denotes optional property
  //add as many bootstrap colours here as intended
  //this implementation restricts the colour prop to one of these
  colour?: 'primary' | 'secondary' | 'danger'; //it's called a union type
}


export const ButtonFancy = () => ( { children, onClick, colour = 'primary'}: ButtonProps ) => {
  return (
    // need to set classname to array as we want to reference two css classes, with some dynamicism
    <button className={[styles.btn, styles['btn-' + colour]].join(' ')} onClick={onClick}>{children}</button>
  )
}

export default ButtonFancy
------

and in App.tsx ;
import Button from "./components/Button";

//using react-icons, which are essentially components
import { FaCalendarWeek } from "react-icons/fa";

function App() {
  //determine the initial state of the alert (initially hidden)

  return (
    <div>
      <Button colour="danger" onClick={() => {}}>Ugly 🦆🦆</Button>
    </div>
  );
}

export default App;
----

another example of using css in js styling -- creating a toggable like button;
Like.tsx;
import { useState } from "react";

//using react-icons, which are essentially components
import { FaHeartCrack } from "react-icons/fa6";
import { FaGrinHearts } from "react-icons/fa";

interface LikeProps {
 onClick: () => void;
}

const Like = ({ onClick}: LikeProps) => {
    // in a real application, this status will be fetched from a database, set dynamically, etc.
  const [status, setStatus] = useState(true);

    //encapsulate the logic for the button in a toggle function
    const toggle = () => {
        setStatus(!status); //whatever status is, pass the inverted value
        onClick(); //called to notify the parent component (the consumer) that the button has been clicked
    }

  if (status) return <FaHeartCrack color="#ff6b81" size={20} onClick={toggle}  />;
    return <FaGrinHearts color="lightblue" size={20} onClick={toggle} />;

};

export default Like;
---------

and in App.tsx ;
import Button from "./components/Button";
import FaHeartCrack from "./components/Like/Like";


function App() {
  //determine the initial state of the alert (initially hidden)

  return (
    <div>
      <Button colour="danger" onClick={() => {}}>Ugly 🦆🦆</Button>
      <br/>
      {/* setting onClick prop for Like.tsx, as well as an error function */}
      <FaHeartCrack onClick={() => console.log('clicked!')} />
    </div>
  );
}

export default App;
------

state management
-- a fundamental concept in react, and it refers to the management of data that changes over time. in react, state is used to store data that can change over time, such as user input, api responses, and other dynamic data. state management is important in react because it allows you to build dynamic and interactive user interfaces.

so we know with state hook, we can add state to functional components, and we can use the state hook to add state to our functional components. but what if we have multiple components that need to share the same state, or if we have complex state that needs to be shared across multiple components? this is where state management comes in.

refresher on state hook;
- react updates state asynchronously (not immediately). done for performance reasons
- state is stored outside of the component function, thanks to this hook (state hook), and is preserved between re-renders
- hooks can only be called at the top level of a functional component, and not inside loops, conditions, or nested functions. this is a rule of hooks in react, and so it means that you can't call hooks inside loops, conditions, or nested functions. you can only call hooks at the top level of a functional component, and this is because react relies on the order in which hooks are called to keep track of the state of the component. if you call hooks inside loops, conditions, or nested functions, react won't be able to keep track of the state of the component, and this can lead to bugs and unexpected behavior.

and when choosing the state structure;
- avoid redundant state variables
- group related variables inside an object
- avoid deeply nested state objects (hard to manage)

AND just like props, we should treat state objects as immutable, and avoid directly modifying them. instead, we should use the setter function provided by the state hook to update the state object. this is because react relies on the immutability of state objects to optimize the rendering process, and avoid unnecessary re-renders.

now for - purity ;
- a pure function is a function that always returns the same output for the same input, and has no side effects. this means that a pure function does not modify any external state, and only depends on its input arguments. in react, pure functions are used to calculate derived state, and are called inside the render method of a component. this allows react to optimize the rendering process, and avoid unnecessary re-renders.
- if given the same input, it will always return the same output, and it has no side effects. this means that it does not modify any external state, and only depends on its input arguments. in react, pure functions are used to calculate derived state, and are called inside the render method of a component. this allows react to optimize the rendering process, and avoid unnecessary re-renders.

e.g.;
const result = myFunction(1); // result = 'b'
react is designed around his concept, it expects every component to be a pure function of its props and state. this means that given the same props and state, a component should always render the same output. this is important for performance, because react can optimize the rendering process by comparing the output of a component to the previous output, and only re-rendering the component if the output has changed.

but how to keep components pure?
keep any changes out of the render phase
and we should not change any objects that existed before rendering
but note that IT IS FINE to update an object that was created during rendering, e.g., in the return function of a component

now - strict mode;
- a react feature that helps identify impure components
so remember this in main.tsx? ;
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
one of the thinga th strict mode does is that it helps identify impure components, and it does this by running the component twice, and comparing the output of the first and second runs. if the output is different, it means that the component is impure, and react will log a warning to the console. this is useful for identifying components that are not pure, and can help you optimize your app for performance.

so for example, a 'pure' component will look like;
MessagePure.tsx;
//example of pure component

let count = 0;

const MessagePure = () => {
  return (
    <div>MessagePure {count} </div>
  )
}

export default MessagePure

-----

whilst an 'impure' component will look like this;
MessageImpure.tsx;

let count = 0;

const MessagePure = () => {
  count++; //the part that makes this impure, this will increment the count every time the component is rendered, which is twice.

  //1. rendered first to check for issues and impurities
  //2. rendered again to show the actual component

  //we can see the double rendering in the console, as the count will be incremented twice;
  console.log(`MessagePure rendered ${count} times`);


  return (
    <div>MessagePure {count} </div>
  )
}

export default MessagePure


----
more on why react renders impure components twice in development mode;

The behavior you're observing, where React renders components twice in development mode, is due to React's StrictMode. React's StrictMode is a tool for highlighting potential problems in an application. It activates additional checks and warnings for its descendants.

StrictMode does not render components twice in production; it only does this in development mode. The double rendering happens only for class components and function components (including hooks) that are children of the StrictMode component.

Here are the main reasons why React might render a component twice in development mode when using StrictMode:

Identify Side Effects: Rendering a component twice helps identify side effects in the render phase. A side effect is any change that can be observed outside the scope of the function being executed. In React, the render phase should be pure and without side effects. By rendering twice, React can help developers spot and fix side effects in the render phase.

Detecting Unexpected Behavior: It helps in detecting unexpected behavior in the application that might not be visible. For example, if a component relies on a side effect from the render phase (which it shouldn't), rendering it twice might expose that reliance.

Preparation for Concurrent Mode: React's future Concurrent Mode relies on the ability to interrupt rendering and start it over. This means components might render multiple times before the screen gets updated. By rendering components twice in development, StrictMode prepares developers for this model by ensuring their components can handle such scenarios without issues.

The reason React takes the result of the second rendering is to ensure that the component's behavior is consistent and does not rely on side effects from the first rendering. This is a development aid to help enforce best practices in writing components but does not affect production builds.

In your specific case, the comment suggests that the MessageImpure components might be relying on some form of state or side effect that changes with each render, leading to the observed behavior of rendering as "2 4 6". This is a demonstration of how StrictMode can help identify and rectify impurities or side effects in component renders.

in other words;
-- the first render is used primarily for detecting and reporting potential issues with the code. it is to identify side effects, and the second render is used to verify that the component is pure. if the output of the first and second renders is different, it means that the component is impure, and react will log a warning to the console. this is useful for identifying components that are not pure, and can help you optimize your app for performance.
-- the second render is used to actually render the component to the dom. this is the render that the user sees, and it is the one that is used to update the dom when the state of the component changes. it is used to actually update the user interface

from react 18, strict mode is turned on by default. needs to be turned off for production

now... updating normal objects in react;
example;
App.tsx
import { useState } from "react";

function App() {
  //remember, when dealing with objects an arrays, treat them as immutable  / read-only
  const [drink, setDrink] = useState({
    title: "Capuccino",
    price: 5,
  });

  const handleClick = () => {
    //don't want to mutate the state directly, so create a new object
    setDrink({
      //but copying all these properties is a bit tedious, so use the spread operator
      // title: drink.title === "Capuccino" ? "Latte" : "Capuccino",
      ...drink, //copies all properties of drink object into newDrink object
      price: drink.price === 5 ? 6 : 5, //an example of changing a property
    });
  };

  return (
    <div>
      {drink.title} - ${drink.price}
      <br></br>
      <button onClick={handleClick}>Change Drink</button>
    </div>
  );
}

export default App;
-----

we used the SPREAD OPERATOR ( ... ) above, this is;
- a javascript feature that allows you to spread the elements of an array or the properties of an object into another array or object. this can be useful for copying arrays and objects, merging arrays and objects, and creating new arrays and objects with additional elements or properties.
- but note that it is a shallow copy, and not a deep copy. this means that it only copies the top-level properties of an object, and not the nested properties. if you need to copy nested properties, you will need to use a deep copy method, such as JSON.parse(JSON.stringify(obj)).
- when updating state in react apps, we should always create a new object or array, and not modify the existing object or array directly. this is because react relies on the immutability of state objects to optimize the rendering process, and avoid unnecessary re-renders. by creating a new object or array, we ensure that react can detect the changes to the state object, and update the component accordingly.

example of updating a nested object;
App.tsx
import { useState } from "react";

function App() {
  const [customer, setCustomer] = useState({
    name: "John Doe",
    age: 25,
    address: {
      street: "123 Main St",
      city: "Anytown",
      state: "NY",
      zip: "12345",
    },
  });

  //remember to use spread operator to copy the object, and then change the property.
  //also keep a flat structure, and avoid deeply nested objects
  //and also, when updating state in react apps, we should always create a new object or array, and not modify the existing object or array directly. this is because react relies on the immutability of state objects to optimize the rendering process, and avoid unnecessary re-renders. by creating a new object or array, we ensure that react can detect the changes to the state object, and update the component accordingly.
  const handleClick = () => {
    setCustomer({
      ...customer,
      address: { ...customer.address, city: "Nueva York" }, //this is the correct way to update nested objects in react. by setting the address property to a new object, we ensure that the address object is replaced with a new object, and not modified directly. this allows react to detect the change, and update the component accordingly.
    });
  };

  return (
    <div>
      {customer.name} is {customer.age} years old.
      <br />
      Residence: {customer.address.street}, {customer.address.city},{" "}
      {customer.address.state} {customer.address.zip}
      <br />
      <button onClick={handleClick}>Change City</button>
    </div>
  );
}

export default App;
------

the same is true for arrays--
we should always create a new array when updating state in react apps, and not modify the existing array directly. this is because react relies on the immutability of state objects to optimize the rendering process, and avoid unnecessary re-renders. by creating a new array, we ensure that react can detect the changes to the state array, and update the component accordingly.

how to update an array in react;
App.tsx
import { useState } from "react";

function App() {

  const [tags, setTags] = useState(["react", "angular", "vue"]);

  const handleClick = () => {
    //add
    setTags([...tags, "svelte"]);

    //remove
    setTags(tags.filter((tag) => tag !== "angular"));

    //update
    setTags(tags.map((tag) => (tag === "react" ? "reactjs" : tag)));
  };

  return (
    <div>
      <button onClick={handleClick}>Change City</button>
    </div>
  );
}

export default App;
----

and as for updating an array of objects;
App.tsx
import { useState } from "react";

function App() {
  //updating an array of bugs
  const [bugs, setBugs] = useState([
    { id: 1, description: "Bug 1", fixed: false },
    { id: 2, description: "Bug 2", fixed: false },
    { id: 3, description: "Bug 3", fixed: false },
  ]);

  const handleClick = () => {
    setBugs(bugs.map((bug) => (bug.id === 1 ? { ...bug, fixed: true } : bug)));
  };

  return (
    <div>
      <button onClick={handleClick}>Change City</button>
    </div>
  );
}

export default App;
----

...but this can get repetitive, so we can simplify things;

using immer library; --
immer is a popular library that simplifies the process of updating nested objects and arrays in react. it allows you to write code that looks like mutable code, but is actually immutable under the hood. this makes it easier to update state in react components, and can help reduce the amount of boilerplate code needed to manage state.
like so;
App.tsx
import { useState } from "react";
import { produce } from 'immer';

function App() {
  //updating an array of bugs
  const [bugs, setBugs] = useState([
    { id: 1, description: "Bug 1", fixed: false },
    { id: 2, description: "Bug 2", fixed: false },
    { id: 3, description: "Bug 3", fixed: false },
  ]);

  const handleClick = () => {
    //the classic way to update an array of objects
    //setBugs(bugs.map((bug) => (bug.id === 1 ? { ...bug, fixed: true } : bug)));
    //using immer to update an array of objects
    setBugs(produce(draft => {
        const bug = draft.find(bug => bug.id === 1);
        if (bug) {
            bug.fixed = true; //here we are updating the object in the array/mutating the object
        }
      }));
  };

  return (
    <div>
      {bugs.map(bug => <p key={bug.id}>{bug.description} - {bug.fixed ? 'Fixed' : 'New'}</p>)}
      <button onClick={handleClick}>Change Bug</button>
    </div>
  );
}

export default App;
--------

OH!!! REMEMBER! ;
as a rule of thumb,  the component that owns / holds the state should be the one modifying it, is the one responsible for updating the state


..... NOW, prop drilling;

now, sometimes we need to share state between components, and this is where state management comes in. state management is the process of managing the state of an application, and it involves storing, updating, and sharing state between different components. in react, state management is typically done using context, redux, or other state management libraries.

for example; 
            App.tsx
          /        \
        /           \
    NavBar.tsx    Cart.tsx
now, we have used the state hook in the Cart.tsx component to store the list of sohpping items. to share the state with the NavBar.tsx component, we have to lift the state up to the closest parent, the App.tsx component, and then pass the state down to the child components as props / share it as needed. this is called prop drilling, and it can be cumbersome and error-prone, especially when dealing with deeply nested components.

prop drilling example (as well as sharing state between components);
CHILD COMPONENTS ---
Cart.tsx
//define the shape of the props
interface CartProps {
  cartItems: string[];
  onClear: () => void;
  //onRemove: (index: number) => void; //if we wanted to allow the user to remove an item from the cart
} //and remember to treat them as immutable objects / read-only

//destructuring the props (not going to modify them here)
const Cart = ({ cartItems, onClear }: CartProps) => {
  return (
    //remember, we are using a fragment here, so we don't have to wrap the elements in a div
    <>
      <div>Cart</div>
      <ul>
        {/* grabbing the cart items and mapping them to list items */}
        {cartItems.map((item, index) => (
          <li key={index}>{item}</li>
        ))}
      </ul>
      {/* instead, we need to notify the app.tsx component that the user clicked on the clear button. so all changes to the state should be done inside the parent component / app.tsx.  */}
      <button onClick={onClear}>Checkout</button> 
      {/* passing a reference */}
    </>
  );
};

export default Cart;
-----

NavBar.tsx ;
import React from 'react'

//define the shape of the props
interface NavBarProps {
    //there's a few options here
    //1. pass all the shopping cart items as an array
    //cartItems: string[]
    //2. pass the number/count of items in the shopping cart
    cartItemsCount: number
}
//destructuring the props
const NavBar = ( {cartItemsCount}: NavBarProps) => {
  return (
    <div>NavBar: {cartItemsCount}</div>
  )
}

export default NavBar;
-----

and PARENT, from where the state is managed;
App.tsx ;
import { useState } from "react";
import NavBar from "./components/NavBar";
import Cart from "./components/Cart";


function App() {
  //use state hook to store list of shopping items
  //real world example of a shopping list;
  // const [shoppingItems, setShoppingItems] = useState([
  //   { id: 1, name: "Milk" },
  //   { id: 2, name: "Bread" },
  //   { id: 3, name: "Cheese" },
  // ]);

  //but for this example, we will use a simple list;
  //and with the state defined, we can pass it to the child components as props
  //as a rule of thumb, remember; the component that owns / holds the state should be the one modifying it, is the one responsible for updating the state
  const [cartItems, setCartItems] = useState([
    "Product 1",
    "Product 2",
    "Product 3",
  ]);

  //because this is where we're maintaing state, all the changes and updates to the cart items state should be done in this component.
  
  //by the same token, if we wanted to allow the user to remove an item from the cart, we would need to pass a function prop to the Cart component that would allow it to remove an item from the cart, like onRemove

  return <div>
    <NavBar cartItemsCount={cartItems.length} />
    <Cart cartItems={cartItems} onClear={() => setCartItems([])} />
  </div>;
}

export default App;

