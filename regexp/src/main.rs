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

fn main() {
    /* our sample regexp corresponds to /ab*c/ in
     * the usual notation. */
    let re = compile("..a*bc");
    println!("Recursive:");
    println!("  match(re, \"abbbc\")\t== {}",
             ::re::recursive::eval(re.as_slice(), "abbbc"));
    println!("  match(re, \"ac\")\t== {}",
             ::re::recursive::eval(re.as_slice(), "ac"));
    println!("  match(re, \"abd\")\t== {}",
             ::re::recursive::eval(re.as_slice(), "abd"));
    println!("Manual Stack:");
    println!("  match(re, \"abbbc\")\t== {}",
             ::re::stack::eval(re.as_slice(), "abbbc"));
    println!("  match(re, \"ac\")\t== {}",
             ::re::stack::eval(re.as_slice(), "ac"));
    println!("  match(re, \"abd\")\t== {}",
             ::re::stack::eval(re.as_slice(), "abd"));
}
