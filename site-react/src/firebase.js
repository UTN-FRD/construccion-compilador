import firebase from 'firebase/app';
import "firebase/auth";
import "firebase/database";

// TODO: move to .env file
const firebaseConfig = {
  apiKey: "AIzaSyCW2-SgurLRBA1ZqdAzw9eQ-SohY54nKTc",
  authDomain: "compilador-funcional.firebaseapp.com",
  projectId: "compilador-funcional",
  storageBucket: "compilador-funcional.appspot.com",
  messagingSenderId: "742433397587",
  appId: "1:742433397587:web:34473484e228244db8dfa6",
  measurementId: "G-FLYCKTP55K"
};

firebase.initializeApp(firebaseConfig);

export default firebase;
