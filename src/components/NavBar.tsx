import React from 'react'

//define the shape of the props
interface NavBarProps {
    //there's a few options here
    //1. pass all the shopping cart items as an array
    //cartItems: string[]
    //2. pass the number/count of items in the shopping cart
    cartItemsCount: number
}
//destructuring the props
const NavBar = ( {cartItemsCount}: NavBarProps) => {
  return (
    <div>NavBar: {cartItemsCount}</div>
  )
}

export default NavBar;