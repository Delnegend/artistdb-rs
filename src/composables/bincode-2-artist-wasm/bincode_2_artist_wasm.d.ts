/* tslint:disable */
/* eslint-disable */
/**
* Deserialize a `Vec<u8>` into an `Artist` struct.
* @param {Uint8Array} data
* @returns {Artist | undefined}
*/
export function decode(data: Uint8Array): Artist | undefined;
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
  code: string;
/**
*/
  desc?: string;
/**
*/
  link?: string;
/**
*/
  name?: string;
}
