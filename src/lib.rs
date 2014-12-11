#![feature(phase, plugin_registrar)]

extern crate rustc;
extern crate syntax;

#[phase(plugin, link)] extern crate log;

use rustc::plugin::Registry;
use syntax::ast::{Attribute, Item, MetaItem};
use syntax::ext::base::{ExtCtxt, ItemModifier, SyntaxExtension};
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ptr::P;

use std::error::FromError;
use std::io::{mod, IoResult};

enum AttrError {
    IoError(io::IoError),
    Path(Span),
    Syntax(Span)
}

impl FromError<io::IoError> for AttrError {
    fn from_error(e: io::IoError) -> AttrError {
        AttrError::IoError(e)
    }
}

fn extract_doc_path(meta: &MetaItem) -> Option<Result<Path, AttrError>> {
    use syntax::ast::MetaItem_::MetaNameValue as NameValue;
    use syntax::ast::MetaItem_::MetaList as List;
    use syntax::ast::Lit_::LitStr;

    match meta.node {
        List(_, ref vec) => {
            let mut found = None;
            for item in vec.iter() {
                if let NameValue(ref name, ref lit) = item.node {
                    if *name == "file" {
                        found = Some(lit.clone())
                    }
                }
            }
            let lit = match found {
                Some(l) => l,
                None => return None
            };
            match lit.node {
                LitStr(ref path, _) => {
                    let path = Path::new_opt(path);
                    let res: Result<Path, _> = path.ok_or(AttrError::Path(lit.span));
                    Some(res)
                },
                _ => Some(Err(AttrError::Syntax(lit.span)))
            }
        },
        _ => None
    }
}

fn slurp_doc_file(cx: &mut ExtCtxt, path: Path, sp: Span) -> IoResult<String> {
    use std::io::File;

    let source_path = Path::new(cx.codemap().span_to_filename(sp));
    let path = source_path.dir_path().join(path);
    let mut file = try!(File::open(&path));
    file.read_to_string()
}

fn mk_doc_attr(doc_string: String) -> Attribute {
    use syntax::attr;
    let meta_item = attr::mk_name_value_item_str(
        token::intern_and_get_ident("doc"),
        token::intern_and_get_ident(doc_string.as_slice())
    );
    attr::mk_attr_inner(attr::mk_attr_id(), meta_item)
}

fn expand_attr_(cx: &mut ExtCtxt, sp: Span, meta: &MetaItem,
                attrs: &mut Vec<Attribute>) -> Result<(), AttrError> {

    let path = extract_doc_path(meta);
    let path = match path {
        Some(res) => try!(res),
        None => return Ok(())
    };
    let doc_string = try!(slurp_doc_file(cx, path, sp));

    let attr = mk_doc_attr(doc_string);
    attrs.push(attr);

    Ok(())
}

fn expand_attr(cx: &mut ExtCtxt, sp: Span, meta: &MetaItem,
               item: P<Item>) -> P<Item> {
    item.map(|mut item| {
        if let Err(e) = expand_attr_(cx, sp, meta, &mut item.attrs) {
            match e {
                AttrError::Path(sp) => {
                    cx.span_err(sp, "invalid NUL character in path");
                }
                // This error is actually unreachable, because non-string literals
                // aren't allowed in meta-items
                AttrError::Syntax(sp) => {
                    cx.span_bug(sp, "used non-string literal in meta-item");
                }
                AttrError::IoError(e) => {
                    cx.span_err(sp, format!("couldn't read doc file: {}", e).as_slice());
                }
            };
        };
        item
    })
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let modifier = box expand_attr as Box<ItemModifier>;
    reg.register_syntax_extension(token::intern("doc"),
                                  SyntaxExtension::Modifier(modifier));
}
