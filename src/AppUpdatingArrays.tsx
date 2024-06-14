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
