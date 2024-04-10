/**
 * Imports a js module using the `path` parameter and returns its default export. Error handling is
 * left to the caller.
 * NOTE: If -for some reason- place of this file was changed, update the path specified in `parser_mod` as well.
 * @param {String} path Path of the js module
 * @returns Returns a promise which contains the default export of specified module.
 */

export async function module_resolver(path) {
  return await import(path).then((mod) => mod.default);
}
