import { CLOSE_ANGLE_BRACKET, OPEN_ANGLE_BRACKET } from "./constants.js";
import {
  isComponent,
  isHTMLTag,
  isSelfClosingHTMLTag,
  isTextNode,
  trimInput,
} from "./util.js";

const markup = `
    <div>
        salam  
        <img/>
        

      <button   id="my_btn">add</button>
      <div>qqejlvzd233</div>
    </div>
    <div><HelloWorld/></div>
`;

function transpiler(jsxLikeSyntax) {
  if (!jsxLikeSyntax.split) {
    throw Error("argument should be a string.");
  }
  const parsedMarkup = trimInput(jsxLikeSyntax);
  const parsedMarkupItems = parsedMarkup.split("\n");
  for (const line of parsedMarkupItems) {
    if (isTextNode(line)) {
      console.log("text: ", line);
    } else if (isSelfClosingHTMLTag(line)) {
      console.log("self closing tag: ", line);
    } else if (isHTMLTag(line)) {
      console.log("tag: ", line);
    } else if (isComponent(line)) {
      console.log("component: ", line);
    } else {
      console.log("wtf: ", line);
    }
  }
  //   console.log(res);
  //   let res = "";

  //   const markup = jsxLikeSyntax.replaceAll(EXTRA_SPACE_REGEX, "");
  //   for (let index = 0; index < markup.length; index++) {
  //     const element = markup[index];
  //     if (element === ">") {
  //       const next = markup[index + 1];
  //       if (!next || next === "\n") {
  //         res += element;
  //         continue;
  //       }
  //       res += element + "\n";
  //       //   else res += element;
  //     } else if (element === "<") {
  //       const prev = markup[index - 1];
  //       if (!prev || index - 1 == 0 || !(prev !== "\n" && prev !== ">")) {
  //         res += element;
  //         continue;
  //       }
  //       res += "\n" + element;
  //       //   else res += element;
  //     } else {
  //       res += element;
  //     }
  //   }
  //   console.log(res);
  //   console.log(m);
}
const LogoutButton = () => "";
const LoginButton = () => "";
const a = `
<label htmlFor="username">Username:</label>
<input type="text" id="username" name="username" />

`;

transpiler(a);
