import {
  CLOSE_ANGLE_BRACKET,
  EXTRA_SPACE_REGEX,
  OPEN_ANGLE_BRACKET,
} from "./constants.js";

function openBracketNeedsLF(prev, index) {
  return (
    prev && index - 1 !== 0 && prev !== "\n" && prev !== CLOSE_ANGLE_BRACKET
  );
}

export function trimInput(inp) {
  let res = "";
  const markup = inp.replaceAll(EXTRA_SPACE_REGEX, "");
  for (let index = 0; index < markup.length; index++) {
    const element = markup[index];
    if (element === CLOSE_ANGLE_BRACKET) {
      const next = markup[index + 1];
      if (!next || next === "\n") {
        res += element;
        continue;
      }
      res += element + "\n";
    } else if (element === OPEN_ANGLE_BRACKET) {
      const prev = markup[index - 1];
      if (!openBracketNeedsLF(prev, index)) {
        res += element;
        continue;
      }
      res += "\n" + element;
    } else {
      res += element;
    }
  }
  return res;
}

export function isSelfClosingHTMLTag(token) {
  const len = token.length;
  return (
    token[0] === OPEN_ANGLE_BRACKET &&
    token[len - 1] === CLOSE_ANGLE_BRACKET &&
    !isFirstLetterLarge(token) &&
    token[len - 2] === "/"
  );
}

export function isHTMLTag(token) {
  const len = token.length;
  return (
    token[0] === OPEN_ANGLE_BRACKET &&
    token[len - 1] === CLOSE_ANGLE_BRACKET &&
    token[len - 2] !== "/" &&
    !isFirstLetterLarge(token)
  );
}

export function isTextNode(token) {
  return (
    !isHTMLTag(token) && !isSelfClosingHTMLTag(token) && !isComponent(token)
  );
}

function isFirstLetterLarge(token) {
  // "".charCodeAt()
  const code = token.charCodeAt(1);
  //   console.log(token[1], code, token[1] & 31);
  return code >= 65 && code <= 90;
}
export function isComponent(token) {
  const len = token.length;
  return (
    token[0] === OPEN_ANGLE_BRACKET &&
    token[len - 1] === CLOSE_ANGLE_BRACKET &&
    token[len - 2] === "/" &&
    isFirstLetterLarge(token)
  );
}
