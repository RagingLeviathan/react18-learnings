import { useState } from "react";
import MessagePure from "./components/Message/MessagePure";
import MessageImpure from "./components/Message/MessageImpure";

function App() {
  //properties of a person object, so better to combine them into a single, person object
  // const [firstName, setFirstName] = useState('');
  // const [lastName, setLastName] = useState('');
  //so create the object by defining a new state variable. preferable over using two
  //BUT WARNING, AVOID DEEPLY NESTED STRUCTURES; KEEPING IT SHALLOW/FLAT IS BETTER
  const [person, setPerson] = useState({
    firstName: "",
    lastName: "",
  });

  //but sometimes, our state variables are related. in such cases, it is better to combine them into a single state variable/object, like so;

  //about the state of our page
  const [isLoading, setIsLoading] = useState(false);

  return (
    <div>
      <input
        type="text"
        value={person.firstName}
        onChange={(e) => setPerson({ ...person, firstName: e.target.value })}
      />
      <input
        type="text"
        value={person.lastName}
        onChange={(e) => setPerson({ ...person, lastName: e.target.value })}
      />
      <div>
        {person.firstName} {person.lastName}
        <br/>
        <h4>Pure Components</h4>
        <MessagePure />
        <MessagePure />
        <MessagePure />
        <h4>Impure Components</h4>
        <MessageImpure />
        <MessageImpure />
        <MessageImpure />
        {/* above will render as 2 4 6, due to strict mode in react. in development, react will render each component twice */}
      </div>
    </div>
  );
}

export default App;
