
interface ButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  //colour?: string; //question mark denotes optional property
  //add as many bootstrap colours here as intended
  //this implementation restricts the colour prop to one of these
  colour?: 'primary' | 'secondary' | 'danger'; //it's called a union type
}

const Button = ( { children, onClick, colour = 'primary'}: ButtonProps ) => {
  return (
    <button className={"btn btn-" + colour} onClick={onClick}>{children}</button>
  )
}

export default Button