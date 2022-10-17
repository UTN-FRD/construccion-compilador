import React, { useContext, useState } from 'react';
import { useHistory } from 'react-router-dom';
import { Controlled as CodeMirror } from 'react-codemirror2';
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/material.css';
import 'codemirror/theme/bespin.css';
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
import { generateTreeJSON } from '../parseAst';

const useStyles = makeStyles((theme) => ({
  buttons: {
    '& > *:not(:last-of-type)': {
      marginRight: 6,
    },
  },
  // TODO: change to a flex value
  editorWrapper: {
    height: '600px',
    width: '100%',

    justifyContent: 'space-evenly',
  },
  editor: {
    height: '100%',
    width: '48%',

    marginHorizontal: "3%",
    '& > .CodeMirror': {
      height: '100%',
      width: '100%',
    },
  },
}));

const Editor = ({ wasm }) => {
  const history = useHistory();
  const classes = useStyles();
  const { currentUser } = useContext(AuthContext);
  const [editorValue, setEditorValue] = useState('');
  const [consoleValue, setConsoleValue] = useState('');
  const [tree, setTree] = useState('');

  const isEditorEmpty = editorValue === '';

  const handleWasm = () => {
    try {
        const interpreter = new wasm.Interpreter(editorValue);
        const ast = JSON.parse(interpreter.getAST());
        const treeJSON = generateTreeJSON(ast);
        console.log(ast);
        // console.log("TreeJSON", JSON.stringify(treeJSON));
        setTree(treeJSON);
        setEditorValue(prevValue => [...prevValue].join('') + '\n' + interpreter.getResult() + '\n');
    } catch (error) {
      console.log({ error });
      // refresh the page if an error is caught
      // history.go(0);
    }
  };

  const resetEditorValue = () => {
    setEditorValue('');
  }

  const handleKeyDown = (editor, event) => {
    switch (event.code) {
      case "ArrowUp":
      case "ArrowDown":
        event.preventDefault();
        break;
      case "Enter":
        const lastLine = editor.getLine(editor.lastLine())

        if (lastLine !== "") {
          setConsoleValue(editor.getValue() + '\n' + '1111')
        }

        alert([...editorValue].join(''))
    }
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
        <Box className={classes.editorWrapper} display="flex" flexDirection="row">
          <CodeMirror
            variant="contained" 
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
          <CodeMirror
            variant="contained" 
            className={classes.editor}
            value={consoleValue}
            options={{
              mode: 'javascript',
              theme: 'bespin',
              lineNumbers: true,
              lineNumberFormatter: v => "$",
            }}
            onKeyDown={(editor, event) => handleKeyDown(editor, event)}
            onBeforeChange={(editor, data, value) => {
              setConsoleValue(value)
            }}
          />
        </Box>
      <hr />
      <div>
        <pre>{JSON.stringify(tree, null, 2)}</pre>
      </div>
      <hr/>
      {tree && <div style={{ width: '1000px', height: '1000px' }}>
        <TreeView key={JSON.stringify(tree)} data={tree} />
      </div>}
    </Box>
  )
}

export default Editor
