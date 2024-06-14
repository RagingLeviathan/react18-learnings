import { useState } from "react";
import NavBar from "./components/NavBar";
import Cart from "./components/Cart";


function App() {
  //use state hook to store list of shopping items
  //real world example of a shopping list;
  // const [shoppingItems, setShoppingItems] = useState([
  //   { id: 1, name: "Milk" },
  //   { id: 2, name: "Bread" },
  //   { id: 3, name: "Cheese" },
  // ]);

  //but for this example, we will use a simple list;
  //and with the state defined, we can pass it to the child components as props
  //as a rule of thumb, remember; the component that owns / holds the state should be the one modifying it, is the one responsible for updating the state
  const [cartItems, setCartItems] = useState([
    "Product 1",
    "Product 2",
    "Product 3",
  ]);

  //because this is where we're maintaing state, all the changes and updates to the cart items state should be done in this component.
  
  //by the same token, if we wanted to allow the user to remove an item from the cart, we would need to pass a function prop to the Cart component that would allow it to remove an item from the cart, like onRemove

  return <div>
    <NavBar cartItemsCount={cartItems.length} />
    <Cart cartItems={cartItems} onClear={() => setCartItems([])} />
  </div>;
}

export default App;
