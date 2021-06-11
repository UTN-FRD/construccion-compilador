import React, { useContext } from 'react';
import { Redirect, Route } from 'react-router-dom';
import { AuthContext } from '../Auth';
import routes from './routes';

const PublicRoute = ({ component: Component, restricted = false, ...rest }) => {
  const { currentUser } = useContext(AuthContext);

  const render = (componentProps) => {
    // if there's a user and route is restricted
    // (e.g. signup or login)
    if (currentUser !== null && restricted) {
      return <Redirect to={routes.HOME} />;
    }

    // else, render the requested component
    return <Component {...componentProps} />;
  };

  return <Route {...rest} render={render} />;
};

export default PublicRoute;
