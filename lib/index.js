"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.annotateSnippet = void 0;
const bindings = require("../pkg");
function annotateSnippet(title, footer, slices, options) {
    return bindings.annotate_snippet(title, footer, slices, options);
}
exports.annotateSnippet = annotateSnippet;
