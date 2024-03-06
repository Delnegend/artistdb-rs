/* tslint:disable */
/* eslint-disable */
/**
* If the `Vec<u8>` is a valid alias, remove the `@` and return it as a `String`.
* @param {Uint8Array} data
* @returns {string | undefined}
*/
export function get_alias(data: Uint8Array): string | undefined;
/**
*/
export class Artist {
  free(): void;
/**
* @returns {Uint8Array}
*/
  to_bitcode(): Uint8Array;
/**
* wasm-pack intepreted this as `Artist | undefined`, using Result is unnecessary
* @param {Uint8Array} bytes
* @returns {Artist | undefined}
*/
  static from_bitcode(bytes: Uint8Array): Artist | undefined;
/**
*/
  alias?: (string)[];
/**
*/
  avatar?: string;
/**
*/
  flag?: string;
/**
*/
  name?: string;
/**
*/
  socials?: (Social)[];
}
/**
*/
export class Social {
  free(): void;
/**
*/
  code?: string;
/**
*/
  desc?: string;
/**
*/
  link?: string;
/**
*/
  name?: string;
/**
*/
  special?: boolean;
}
