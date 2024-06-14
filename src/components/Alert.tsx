//we wanna make the text of the alert component dynamic, so we will pass the text as a prop to the component, using a interface to define the prop type and shape

interface AlertProps {
  //children: string; // special prop that all components support, which allows us to pass text as children to this component when Alert is used in App.tsx.

  //using string works, but there is a better way; ReactNode
  children: React.ReactNode; //this allows us to pass any type of data as children to the component
  onClose: () => void;
}

//remember to destructure
const Alert = ({ children, onClose }: AlertProps) => {
  return (
    <div className="alert alert-primary alert-dismissible fade show">
      {children}
      {/* REMEMBER the onClose function, we are just passing the onClose function through, NOT INVOKING IT RIGHT NOW. only when the user clicks on the close button will react call the function for us */}
      <button type="button" onClick={onClose} className="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
    </div>
  );
};

export default Alert;
