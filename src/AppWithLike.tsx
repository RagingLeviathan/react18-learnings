import Button from "./components/Button";
import FaHeartCrack from "./components/Like/Like";


function App() {
  //determine the initial state of the alert (initially hidden)

  return (
    <div>
      <Button colour="danger" onClick={() => {}}>Ugly 🦆🦆</Button>
      <br/>
      {/* setting onClick prop for Like.tsx, as well as an error function */}
      <FaHeartCrack onClick={() => console.log('clicked!')} />
    </div>
  );
}

export default App;
