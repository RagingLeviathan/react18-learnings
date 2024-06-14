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
