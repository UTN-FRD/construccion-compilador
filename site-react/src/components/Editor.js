import React, { useState } from 'react'
import { Controlled as CodeMirror } from 'react-codemirror2'
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/material.css';
import { useHistory } from 'react-router-dom';

const Editor = ({ wasm }) => {
  const history = useHistory();
  const [editorValue, setEditorValue] = useState('FRD LISP');

  const handleWasm = (value) => {
    try {
      const parsedValue = new wasm.LispVal(editorValue).toString();
      setEditorValue(prevValue => [...prevValue].join('') + '\n' + parsedValue + '\n');
    } catch (error) {
      console.log({ error });
      // refresh the page if an error is caught
      history.go(0);
    }
  };

  return (
    <div>
      <button
        type="button"
        style={{ marginBottom: '5rem', border: 0, backgroundColor: "#333", color: "#fff", padding: '10px' }}
        onClick={handleWasm}
      >
        RUN
      </button>
      <CodeMirror
        style={{ height: '500px' }}
        value={editorValue}
        options={{
          mode: 'xml',
          theme: 'material',
          lineNumbers: true
        }}
        onBeforeChange={(editor, data, value) => {
          setEditorValue(value)
        }}
        onChange={(editor, data, value) => {
          console.log({editor, data, value})
        }}
      />
    </div>
  )
}

export default Editor
