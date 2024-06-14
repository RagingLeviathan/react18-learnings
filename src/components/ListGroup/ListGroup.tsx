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
      <h1> {heading}</h1>
      {items.length === 0 && <p>There are no items in the list.</p>}{" "}
      {/* this html element........ */}
      {/* <ul className={[styles.listGroup, styles.container].join(' ')}>  so use camel notation instead */}
      {/* ...will be replaced with a styled component */}
      {/* this line also features use of inline css styling - but use sparingly */}
      <List className={styles.listGroup} style={{ backgroundColor: 'greenyellow' }}>
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
