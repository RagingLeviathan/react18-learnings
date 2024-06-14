//define the shape of the props
interface CartProps {
  cartItems: string[];
  onClear: () => void;
  //onRemove: (index: number) => void; //if we wanted to allow the user to remove an item from the cart
} //and remember to treat them as immutable objects / read-only

//destructuring the props (not going to modify them here)
const Cart = ({ cartItems, onClear }: CartProps) => {
  return (
    //remember, we are using a fragment here, so we don't have to wrap the elements in a div
    <>
      <div>Cart</div>
      <ul>
        {/* grabbing the cart items and mapping them to list items */}
        {cartItems.map((item, index) => (
          <li key={index}>{item}</li>
        ))}
      </ul>
      {/* instead, we need to notify the app.tsx component that the user clicked on the clear button. so all changes to the state should be done inside the parent component / app.tsx.  */}
      <button onClick={onClear}>Checkout</button> 
      {/* passing a reference */}
    </>
  );
};

export default Cart;
