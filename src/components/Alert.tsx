
//we wanna make the text of the alert component dynamic, so we will pass the text as a prop to the component, using a interface to define the prop type and shape

interface AlertProps {
    //children: string; // special prop that all components support, which allows us to pass text as children to this component when Alert is used in App.tsx.

    //using string works, but there is a better way; ReactNode
    children: React.ReactNode; //this allows us to pass any type of data as children to the component
}

//remember to destructure
const Alert = ({ children }: AlertProps) => {
  return (
    <div className="alert alert-primary">{children}</div>
  )
}

export default Alert