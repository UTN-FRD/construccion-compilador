function generateTreeJSON(ast) {
    return ast.map((node) => parseExpr(node))
}

function parseExpr(node) {
    let nodeType = Object.keys(node)[0];
    let nodeValue = Object.values(node)[0];
    let result;
    switch (nodeType) {
        case "DefineFunction":
            result = parseDefineFunction(nodeValue);
            break;
        case "DefineVariable":
            result = parsedDefineVariable(nodeValue);
            break;
        case "List":
            result = parseList(nodeValue);
            break;
        case "Atom":
            result = parseAtom(nodeValue);
            break;
        case "If":
            result = parseIf(nodeValue);
            break;
        default:
            console.log("Failed to parse the expression.")
            break;
    }
    return result;
}

function parseIf(nodeValues) {
    let condition = parseExpr(nodeValues[0]);
    let positive = parseExpr(nodeValues[1])
    let negative = parseExpr(nodeValues[2]);

    return generateIf(condition, positive, negative);
}

function generateIf(cond, positive, negative) {
    return {
        "name": "If",
        "children": [
            cond,
            positive,
            negative,
        ]
    }
}

function parsedDefineVariable(nodeValues) {
    let name = nodeValues[0];
    let value = parseExpr(nodeValues[1]);

    return {
        "name": name,
        "children": [
            value
        ]
    }
}

function parseAtom(value) {
    return {
        "name": Object.values(value)[0] // Peligro!!
    }
}

function parseList(nodeValues) {
    let children = nodeValues.map((elem) => parseExpr(elem));

    return generateList(children);
}

function generateList(children) {
    return {
        "name": "",
        "children": children
    }
}

function parseDefineFunction(nodeValues) {
    let name = nodeValues[0];
    let args = nodeValues[1];
    let body = nodeValues[2].map((exprs) => parseExpr(exprs));

    return generateDefineFunction(name, args, body)
}

// 
function generateDefineFunction(name, args, body) {
    let argsObjects = args.map((arg) => ({ "name": arg }));
    console.log("Body", JSON.stringify(body))
    return {
        "name": "DefineFunction",
        "children": [
            { "name": name },
            {
                "name": "argumentos",
                "children": argsObjects
            },
            {
                "name": "cuerpo",
                "children": body
            }
        ]
    }
}


module.exports = {
    "generateTreeJSON": generateTreeJSON
}