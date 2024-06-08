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
with es7+ react extension installed, you can type 'rafce' and it will automatically generate a functional component for you. it is short for react arrow function component export.


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