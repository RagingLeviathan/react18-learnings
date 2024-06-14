import { useState } from "react";

//using react-icons, which are essentially components
import { FaHeartCrack } from "react-icons/fa6";
import { FaGrinHearts } from "react-icons/fa";

interface LikeProps {
 onClick: () => void;
}

const Like = ({ onClick}: LikeProps) => {
    // in a real application, this status will be fetched from a database, set dynamically, etc.
  const [status, setStatus] = useState(true);

    //encapsulate the logic for the button in a toggle function
    const toggle = () => {
        setStatus(!status); //whatever status is, pass the inverted value
        onClick(); //called to notify the parent component (the consumer) that the button has been clicked
    }

  if (status) return <FaHeartCrack color="#ff6b81" size={20} onClick={toggle}  />;
    return <FaGrinHearts color="lightblue" size={20} onClick={toggle} />;

};

export default Like;
