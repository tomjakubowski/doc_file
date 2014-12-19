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
    Path,
    Syntax
}

impl FromError<io::IoError> for AttrError {
    fn from_error(e: io::IoError) -> AttrError {
        AttrError::IoError(e)
    }
}

fn extract_doc_path(meta: &MetaItem) -> Result<Path, AttrError> {
    use syntax::ast::MetaItem_::MetaNameValue as NameValue;
    use syntax::ast::Lit_::LitStr;

    match meta.node {
        NameValue(_, ref lit) => {
            match lit.node {
                LitStr(ref path, _) => {
                    Path::new_opt(path).ok_or(AttrError::Path)
                },
                _ => Err(AttrError::Syntax)
            }
        },
        _ => Err(AttrError::Syntax)
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

    let path = try!(extract_doc_path(meta));
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
                AttrError::Path => {
                    cx.span_err(sp, "Invalid path in doc_file attribute");
                }
                AttrError::Syntax => {
                    cx.span_err(sp, "Invalid use of doc_file attribute, use like: \
                                `#[doc_file = \"foo.markdown\"]`");
                }
                AttrError::IoError(e) => {
                    let msg = format!("IO error reading doc_file attribute: {}", e);
                    cx.span_err(sp, msg.as_slice());
                }
            };
        };
        item
    })
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let modifier = box expand_attr as Box<ItemModifier>;
    reg.register_syntax_extension(token::intern("doc_file"),
                                  SyntaxExtension::Modifier(modifier));
}
