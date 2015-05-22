/* This is a basic implementation of a regular expression matcher,
 * based on Henry Spencer's virtual-machine approach to regular
 * expression matching outlined by Russ Cox here:
 *   http://swtch.com/~rsc/regexp/regexp2.html
 *
 * For ease of parsing, I'm using a highly non-standard Polish
 * notation for regular expressions, in which . and | are
 * prefix binary operators for catenation and choice, respectively,
 * and * is a prefix unary operator for repetition. */
use re::compile;
mod re;

fn as_chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn main() {
    /* our sample regexp corresponds to /ab*c/ in
     * the usual notation.
     * These two lines can be collapsed into one once
     * this RFC lands: https://github.com/rust-lang/rfcs/pull/66
     */
    let instrs = &compile("..a*bc");

    println!("Recursive:");
    println!("  match(re, \"abbbc\")\t== {}",
             ::re::recursive::eval(instrs, &as_chars("abbbc")));
    println!("  match(re, \"ac\")\t== {}",
             ::re::recursive::eval(instrs, &as_chars("ac")));
    println!("  match(re, \"abd\")\t== {}",
             ::re::recursive::eval(instrs, &as_chars("abd")));

    println!("Manual Stack:");
    println!("  match(re, \"abbbc\")\t== {}",
             ::re::stack::eval(instrs, &as_chars("abbbc")));
    println!("  match(re, \"ac\")\t== {}",
             ::re::stack::eval(instrs, &as_chars("ac")));
    println!("  match(re, \"abd\")\t== {}",
             ::re::stack::eval(instrs, &as_chars("abd")));
}
