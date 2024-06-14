import styles from './ListGroup.module.css'
import { useState } from "react";
import styled from 'styled-components';

//creating a styled component
//this replaces regular html elements with a styled component
const List = styled.ul`
  list-style-type: none;
  padding: 0;
  margin: 0;
`;

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
      {/* {items.length === 0 ? <p>There are no items in the list.</p> : null} */}
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* paragraph tag will be returned if true; for false, nothing will be rendered on screen */}
      {/* <ul className={styles.list-group}> // can't use dot notation with hyphens, throws compilation error */}
      {/* <ul className={styles['list-group']}>  this is how you can access a property with a hyphen in the name. but it be kinda ugly */}
      <ul className={[styles.listGroup, styles.container].join(' ')}>  {/*so use camel notation instead */}
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
