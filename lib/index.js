"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.annotate_snippet = void 0;
const bindings = require("../pkg");
function annotate_snippet(title, footer, slices, options) {
    return bindings.annotate_snippet(title, footer, slices, options);
}
exports.annotate_snippet = annotate_snippet;
