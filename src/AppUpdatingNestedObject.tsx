import { useState } from "react";

function App() {
  const [customer, setCustomer] = useState({
    name: "John Doe",
    age: 25,
    address: {
      street: "123 Main St",
      city: "Anytown",
      state: "NY",
      zip: "12345",
    },
  });

  //remember to use spread operator to copy the object, and then change the property.
  //also keep a flat structure, and avoid deeply nested objects
  //and also, when updating state in react apps, we should always create a new object or array, and not modify the existing object or array directly. this is because react relies on the immutability of state objects to optimize the rendering process, and avoid unnecessary re-renders. by creating a new object or array, we ensure that react can detect the changes to the state object, and update the component accordingly.
  const handleClick = () => {
    setCustomer({
      ...customer,
      //copy the customer object, and then change the city property
      address: { ...customer.address, city: "Nueva York" }, //this is the correct way to update nested objects in react. by setting the address property to a new object, we ensure that the address object is replaced with a new object, and not modified directly. this allows react to detect the change, and update the component accordingly.

      //because spread (... ) operator is a shallow copy,  it means when we use it to copy the customer object, we are only copying the first level of properties. so, the address property is still pointing to the same object in memory. so, when we update the city property of the address object, we are actually modifying the original address object, and not creating a new one. this is why react does not detect the change, and does not update the component. this is not the correct way to update nested objects in react. by setting the address property to a new object, we ensure that the address object is replaced with a new object, and not modified directly. nothing should be shared. this allows react to detect the change, and update the component accordingly.

    });
  };

  return (
    <div>
      {customer.name} is {customer.age} years old.
      <br />
      Residence: {customer.address.street}, {customer.address.city},{" "}
      {customer.address.state} {customer.address.zip}
      <br />
      <button onClick={handleClick}>Change City</button>
    </div>
  );
}

export default App;
