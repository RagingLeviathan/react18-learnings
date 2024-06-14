//example of impure component

let count = 0;

const MessagePure = () => {
  count++; //the part that makes this impure, this will increment the count every time the component is rendered, which is twice.

  //1. rendered first to check for issues and impurities
  //2. rendered again to show the actual component

  //we can see the double rendering in the console, as the count will be incremented twice;
  console.log(`MessagePure rendered ${count} times`);

  return (
    <div>MessagePure {count} </div>
  )
}

export default MessagePure