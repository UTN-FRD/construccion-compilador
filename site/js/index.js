import * as lisp from "frd_lisp";
import { elements } from "./elements";

const codeCodeMirror = CodeMirror.fromTextArea(elements.replDefinitions, {
  lineNumbers: true,
  mode: "haskell",
});
const shell = CodeMirror.fromTextArea(elements.replInput, {
  lineNumbers: true,
  theme: "ayu-dark",
  mode: "shell",
});
shell.setOption("theme", "ayu-dark");

elements.execute.addEventListener("click", () => {
  try {
    // get shell input
    const shellValue = shell.getValue();

    // parse shell input
    const parsed_value = new lisp.LispVal(shellValue);
    elements.output.innerHTML = parsed_value.toString();

    // clear shell value
    shell.setValue("");
  } catch (e) {
    elements.output.innerHTML = new Date() + " ERROR - " + e;
  }
});
