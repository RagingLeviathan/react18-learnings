import { useState } from "react";
import Alert from "./components/Alert";
import Button from "./components/Button";
//import ListGroup from "./components/ListGroup/ListGroup"; //we can use this but it's a bit verbose/unwieldy
//a solution is to use a barrel file, which is an index.ts file in the components folder
import ListGroup from "./components/ListGroup"; //with this, by default, the compiler will look for an index.ts file in the folder, and if it finds one, it will use that file as the default export

//using react-icons, which are essentially components
import { FaCalendarWeek } from "react-icons/fa";


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

      <ListGroup
        heading="Fruits"
        items={["Apple", "Banana", "Cherry", "Date"]}
        onSelectItem={(item: any) => console.log(item)}
      />

      <FaCalendarWeek color="red" size="40"/>
    </div>
  );
}

export default App;
