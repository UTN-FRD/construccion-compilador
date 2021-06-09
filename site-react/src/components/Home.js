import React, { useContext } from 'react'
import { Link } from 'react-router-dom';
import app from '../firebase'
import { AuthContext } from "./Auth.js";

const Home = () => {
  const { currentUser } = useContext(AuthContext);

  return (
    <div>
      <h1>Home</h1>
      <p>Logeado: {currentUser ? "logeado" : "deslogeado"}</p>
      {currentUser ? <button onClick={() => app.auth().signOut()}>Sign Out</button> : <Link to="/login">Login</Link>}
    </div>
  )
}

export default Home
