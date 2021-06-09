import { useEffect, useState } from 'react';
import { BrowserRouter as Router, Route } from "react-router-dom";
import { AuthProvider } from './components/Auth';
import Home from './components/Home'
import SignUp from './components/SignUp'
import Login from './components/Login'
import PrivateRoute from './components/PrivateRoute'
import Private from './components/Private'
import Editor from './components/Editor';

function App() {
  const [wasm, setWasm] = useState({});

  useEffect(() => {
    const loadWasm = async () => {
      try {
        const wasm = await import('./pkg');
        setWasm(wasm)
      } catch(err) {
        console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
      }
    };
    loadWasm();
  }, []);

  useEffect(() => {
    if (Object.values(wasm).length) {
      const parsedValue = new wasm.LispVal("(+ 2 5)").toString();
      console.log(parsedValue);
    }
  }, [wasm])
  
  return (
    <AuthProvider>
      <Router>
        <div>
          <Route exact path="/" component={Home} />
          <Route exact path="/login" component={Login} />
          <Route exact path="/signup" component={SignUp} />
          <Route exact path="/editor" component={() => <Editor wasm={wasm} />} />
          <PrivateRoute exact path="/private" component={Private} />
        </div>
      </Router>
    </AuthProvider>
  );
}

export default App;
