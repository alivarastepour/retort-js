export async function module_resolver(path) {
  return await import(path).then((mod) => mod.default);
}
