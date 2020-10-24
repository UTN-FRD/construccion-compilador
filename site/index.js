import * as lisp from "frd_lisp";

let replInput = document.getElementById("repl-input");
let replDefinitions = document.getElementById("repl-extra-code");
let output = document.getElementById("repl-output");

replInput.addEventListener("keyup", event => {
    if (event.code === "Enter") {
        event.preventDefault();

        // get input from console
        let input = replInput.value;

        // get input from definitions
        let definitions = replDefinitions.value;

        // parse definitions + user input
        let parsed_value = new lisp.LispVal(definitions + input);

        // show parsed value from replInput
        output.innerHTML = parsed_value.toString();

        // clear the input
        replInput.value = "";
    }
});
