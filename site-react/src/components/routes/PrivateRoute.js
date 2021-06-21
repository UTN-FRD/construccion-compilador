import React, { useContext } from 'react';
import { Redirect, Route } from 'react-router-dom';
import { AuthContext } from "../Auth";
import routes from './routes';

const PrivateRoute = ({
  component: Component,
  restricted = false,
  ...rest
}) => {
  const { currentUser } = useContext(AuthContext);

  const render = (props) => {
    // if no user is logged in, redirect to login page
    if (currentUser === null) {
      return <Redirect to={routes.LOGIN} />;
    }

    // else, render the requested component
    return <Component {...props} />;
  };

  return <Route {...rest} render={render} />;
};

export default PrivateRoute;
