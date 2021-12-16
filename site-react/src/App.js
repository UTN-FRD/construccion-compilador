import React, { useContext, useEffect, useState } from 'react';
import { BrowserRouter as Router } from "react-router-dom";
import { PublicRoute, PrivateRoute, ROUTES } from './components/routes'
import { AuthContext } from './components/Auth';
import Editor from './components/Editor';
import SignUp from './components/SignUp';
import LogIn from './components/Login';
import firebase from './firebase';
import TopBar from './components/TopBar/TopBar';
import Container from '@material-ui/core/Container';

function App() {
  const [wasm, setWasm] = useState({});
  const [loading, setLoading] = useState(true);
  const { setCurrentUser } = useContext(AuthContext);

  useEffect(() => {
    const loadWasm = async () => {
      try {
        // first, load wasm package
        const wasm = await import('./pkg');
        // then, get user state (logged in or logged out)
        firebase.auth().onAuthStateChanged(user => {
          setWasm(wasm);
          setCurrentUser(user);
          setLoading(false);
        });
      } catch (error) {
        console.error(`Unexpected error. [Message: ${error.message}]`);
        setLoading(false);
      }
    };

    loadWasm();
  }, []);
  
  if (loading) return <p>Loading...</p>

  return (
    !loading && 
    <Router>
      <TopBar />
      <Container maxWidth="lg">
        <PublicRoute exact path={ROUTES.HOME} component={() => <Editor wasm={wasm} />} />
        <PublicRoute exact restricted path={ROUTES.SIGNUP} component={SignUp} />
        <PublicRoute exact restricted path={ROUTES.LOGIN} component={LogIn} />
      </Container>
    </Router>
  );
}

export default App;
