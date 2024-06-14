//LATEST VERSION IS IN LISTGROUP FOLDER

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
//  NON OF THE PROP VALUES ABOVE SHOULD BE CHANGED IN THIS FUNCTION
//this is an anti-pattern in react, based on functional programming principles


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

  //putting multiline function here rather than nested in the jsx markup
  //but hold up, if we write it like this, we'll get a typescript error
  //const handleClick = (event) => { console.log(event); }; //this is a function that takes an event as a parameter and logs it to the console
  //the error thrown is 'Parameter 'event' implicitly has an 'any' type.ts(7006)', and the reason why is because
  //the typescript compiler doesn't know what type of parametre (event) is being passed in, so it assumes it's of type 'any'
  //which in turn breaks the event. (event + dot) operator. this is a case where we need to be explicit about the type of the parametre(event), in order to still get autocomplete and type checking/safety, which when used in ajsx thing, is (parameter) event: React.MouseEvent<HTMLLIElement, MouseEvent>. and because we are declaring a brand new function, the typescript compiler doesn't know where we're gonna use this function, so it doesn't know what type of event to expect, so we have to be explicit about it.
  //this can be solved by importing mouseevent from react, and then using it as a type for the event parameter, and rewriting the declared function like so;
  const handleClick = (event: MouseEvent) => {
    console.log(event);
  }; //this is an event handler, because its job is handling an event, in this case, the click event
  //this is an example of type annoation in typescript. we type annotation and specify the type of our variables, parameters, and functions, so that the typescript compiler can check for errors and give us warnings, and also provide us with autocomplete and intellisense.
  //now with this annotation, if we use the dot operator, we can see all the properties and methods that are available on the (mouse) event object, and we can use them in our function. this is one of the beauties of typescript, and why it's so powerful and useful.
  //we get autocompletion, we get type safety, and it's easier to refactor or restructure our code

  //this logic can also be moved inside a function, and then we can call that function inside the jsx
  // const message =
  //   items.length === 0 ? <p>There are no items in the list.</p> : null; //use ternary operator here to conditionally render a message

  //like so (using arrow function syntax)
  // const getMessage = () => {
  //   return items.length === 0 ? <p>There are no items in the list.</p> : null; //use ternary operator here to conditionally render a message
  // };

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
      {/* <h1>List</h1> */}
      <h1>{heading}</h1>
      {/* { items.length === 0 ? <p>There are no items in the list.</p> : null }  {use ternary operator here to conditionally render a message } */}
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
      {/* {ListGroupProps.items.length === 0 && <p>There are no items in the list.</p>}{" "} */}
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

              //older notes, don't remove
              // onClick={(event) => console.log(item, index, event)} //similarly, onclick can have an event parameter.
              //and when using inline here, the typescript compiler knows that the event is of type React.MouseEvent<HTMLLIElement, MouseEvent>, so doesn't throw an error/give a warning
              //but having declared a function above, we can replace this inline event with the following;
              //onClick={handleClick} //note that the function is NOT BEING CALLED, we just want to pass a reference. in essence you are telling react that whenever the user clicks on this event, this function should be caught. so calling this function will be done later at runtime, when the event is triggered
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
