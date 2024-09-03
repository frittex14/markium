import { all, createStarryNight } from "https://esm.sh/@wooorm/starry-night@3?bundle";
import { toDom } from "https://esm.sh/hast-util-to-dom@4?bundle";

const starryNight = await createStarryNight(all);
const prefix = "language-";

// Find all code blocks
const nodes = Array.from(document.body.querySelectorAll("code"));

for (const node of nodes) {
  // Check if code block has the language class prefix
  const className = Array.from(node.classList).find(function(d) {
    return d.startsWith(prefix);
  });

  // If not, cancel
  if (!className) continue;

  // Try to build a scope based on the language prefix
  const scope = starryNight.flagToScope(className.slice(prefix.length));

  // If not possible, cancel
  if (!scope) continue;

  // Replace the code block with the highlighted code
  const tree = starryNight.highlight(node.textContent, scope);
  node.replaceChildren(toDom(tree, { fragment: true }));
}
