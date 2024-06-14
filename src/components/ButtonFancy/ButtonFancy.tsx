import styles from './ButtonFancy.module.css'

interface ButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  //colour?: string; //question mark denotes optional property
  //add as many bootstrap colours here as intended
  //this implementation restricts the colour prop to one of these
  colour?: 'primary' | 'secondary' | 'danger'; //it's called a union type
}


export const ButtonFancy = () => ( { children, onClick, colour = 'primary'}: ButtonProps ) => {
  return (
    // need to set classname to array as we want to reference two css classes, with some dynamicism
    <button className={[styles.btn, styles['btn-' + colour]].join(' ')} onClick={onClick}>{children}</button>
  )
}

export default ButtonFancy