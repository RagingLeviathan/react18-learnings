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
