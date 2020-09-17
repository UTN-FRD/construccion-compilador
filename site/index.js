import * as lisp from "frd_lisp";

let input = document.getElementById("repl-input");
let output = document.getElementById("repl-output");

input.addEventListener("keyup", event => {
    if (event.code === "Enter") {
        event.preventDefault();
        let content = input.value;

        let parsed_value = new lisp.LispVal(content);

        output.innerHTML = parsed_value.toString();

        // clear the input
        input.value = "";
    }
});