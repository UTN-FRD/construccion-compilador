import * as lisp from "frd_lisp";

let input = document.getElementById("repl-input");

let extracode = document.getElementById("repl-extra-code");
let output = document.getElementById("repl-output");

input.addEventListener("keyup", event => {
    if (event.code === "Enter") {
        event.preventDefault();
        let content = input.value;
        let extra = extracode.value;
        console.log(extra + content);
        let parsed_value = new lisp.LispVal(extra + content);

        output.innerHTML = parsed_value.toString();

        // clear the input
        input.value = "";
    }
});