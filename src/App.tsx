import Alert from "./components/Alert";

function App() {
  return (
    <div>
      {/* passing just a string */}
      {/* <Alert children="Hello world." /> */}
      {/* but now, with reactnode in use, we can pass a complex type */}
      {/* better to include components like this */}
      <Alert>
        Hello <span>world.</span>
      </Alert>
    </div>
  );
}

export default App;
