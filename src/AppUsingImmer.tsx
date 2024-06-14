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
