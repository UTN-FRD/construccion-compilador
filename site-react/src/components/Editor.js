import React, { useContext, useState } from 'react';
import { useHistory } from 'react-router-dom';
import { Controlled as CodeMirror } from 'react-codemirror2';
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/material.css';
import { makeStyles } from '@material-ui/core';
import Button from '@material-ui/core/Button';
import PlayArrowIcon from '@material-ui/icons/PlayArrow';
import ClearIcon from '@material-ui/icons/Clear';
import SaveIcon from '@material-ui/icons/Save';
import Box from '@material-ui/core/Box';
import Paper from '@material-ui/core/Paper';
import firebase from '../firebase';
import { AuthContext } from './Auth';
import TreeView from './TreeView/TreeView';

const useStyles = makeStyles((theme) => ({
  buttons: {
    '& > *:not(:last-of-type)': {
      marginRight: 6,
    },
  },
  // TODO: change to a flex value
  editorWrapper: {
    height: '600px',
  },
  editor: {
    height: '100%',
    '& > .CodeMirror': {
      height: '100%',
    },
  },
}));

const Editor = ({ wasm }) => {
  const history = useHistory();
  const classes = useStyles();
  const { currentUser } = useContext(AuthContext);
  const [editorValue, setEditorValue] = useState('');

  const isEditorEmpty = editorValue === '';

  const handleWasm = () => {
    try {
      // use wasm `toString` function
      const parsedValue = new wasm.LispVal(editorValue).toString();
      setEditorValue(prevValue => [...prevValue].join('') + '\n' + parsedValue + '\n');
    } catch (error) {
      console.log({ error });
      // refresh the page if an error is caught
      history.go(0);
    }
  };

  const resetEditorValue = () => {
    setEditorValue('');
  }

  const saveToFirebase = () => {
    if (!currentUser) return alert('Tenes que iniciar sesión para guardar código.');

    const codeRef = firebase.database().ref('Code');
    const code = {
      user: currentUser.email,
      value: editorValue,
    }
    codeRef.push(code);

    alert('Código guardado con éxito.');
  }

  return (
    <Box my={6}>
      <Box className={classes.buttons} mb={2} display="flex" justifyContent="center">
        <Button
          variant="contained"
          color="primary"
          startIcon={<PlayArrowIcon />}
          disabled={isEditorEmpty}
          onClick={handleWasm}
        >
          Run
        </Button>
        <Button
          variant="contained"
          color="primary"
          startIcon={<ClearIcon />}
          disabled={isEditorEmpty}
          onClick={resetEditorValue}
        >
          Clear
        </Button>
        <Button
          variant="contained"
          color="primary"
          startIcon={<SaveIcon />}
          disabled={isEditorEmpty}
          onClick={saveToFirebase}
        >
          Save
        </Button>
      </Box>
      <Paper elevation={16} className={classes.editorWrapper}>
        <CodeMirror
          className={classes.editor}
          value={editorValue}
          options={{
            mode: 'javascript',
            theme: 'material',
            lineNumbers: true
          }}
          onBeforeChange={(editor, data, value) => {
            setEditorValue(value)
          }}
        />
      </Paper>
      <hr />
      <div style={{ width: '1000px', height: '1000px' }}>
        <TreeView />
      </div>
    </Box>
  )
}

export default Editor
