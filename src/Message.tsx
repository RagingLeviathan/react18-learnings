/**
 two ways to create a react component;
- use a js class
- function 
these days, we use functions because they are easier to read and write
 */

function Message() {
    const name = '';
    //exanple of jsx: javascript xml
    if (name) //if truthy
        return <h1>Hello, {name}!</h1>; //using a js expression; a piece of code that gets evaluated and returns a value; can also use a function, like {name.toUpperCase()} , or any piece of js code that returns a value
        //this code, under the hood, will get compiled down into javascript
    return <h1>Hello, stranger!</h1>;
}

//to use it, we export it
export default Message;