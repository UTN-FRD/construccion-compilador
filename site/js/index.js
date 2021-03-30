import * as lisp from "frd_lisp";
import { elements } from "./elements";

const shell = CodeMirror.fromTextArea(elements.replInput, {
  lineNumbers: true,
  theme: "ayu-dark",
  mode: "shell",
  extraKeys: {
    "Enter": onNewLine
  }
});

shell.setOption("theme", "ayu-dark");


function onNewLine(e){
  // get shell input
  const shellValue = shell.getValue();

  shell.replaceSelection("\n" ,"end");
  var doc = shell.getDoc();
  var cursor = doc.getCursor();

  try {
    // parse shell input
    const parsed_value = new lisp.LispVal(shellValue);
    const result = parsed_value.toString();

    doc.replaceRange(result+"\n", cursor);
  } catch (e) {
    doc.replaceRange("ERROR > "+e+"\n", cursor);
  }

}
