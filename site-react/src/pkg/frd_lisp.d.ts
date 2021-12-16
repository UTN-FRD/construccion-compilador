/* tslint:disable */
/* eslint-disable */
/**
*/
export class Interpreter {
  free(): void;
/**
* @param {any} value
*/
  constructor(value: any);
/**
* @returns {any}
*/
  getTokens(): any;
/**
* @returns {any}
*/
  getAST(): any;
/**
* @returns {string}
*/
  getResult(): string;
}
/**
*/
export class LispVal {
  free(): void;
/**
* @param {any} value
*/
  constructor(value: any);
/**
* @returns {string}
*/
  toString(): string;
}
