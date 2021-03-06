//! The signatures of the c functions we'll call

use libc::{c_char, c_void, c_int, c_uint, size_t};

#[link(name = "xml2")]
extern "C" {
    //tree
    pub fn xmlSaveFile(filename: *const c_char, cur: *mut c_void) -> c_int;
    pub fn xmlNewDoc(version: *const c_char) -> *mut c_void;
    pub fn xmlFreeDoc(cur: *mut c_void);
    // pub fn xmlFree(name : *const c_char);
    // pub fn xmlNewNode(ns : *mut c_void, name: *const c_char) -> *mut c_void;
    pub fn xmlNewDocNode(doc: *mut c_void, ns : *mut c_void, name: *const c_char, content: *const c_char) -> *mut c_void;
    pub fn xmlNewDocText(doc: *const c_void, content: *const c_char) -> *mut c_void;
    pub fn xmlFreeNode(cur: *mut c_void);
    pub fn xmlUnlinkNode(cur: *mut c_void);
    pub fn xmlNewNs(node : *mut c_void, href: *const c_char, prefix: *const c_char) -> *mut c_void;
    pub fn xmlNewChild(parent : *mut c_void, ns: *mut c_void, name: *const c_char, content: *const c_char) -> *mut c_void;
    pub fn xmlAddChild(parent : *mut c_void, cur : *mut c_void) -> *mut c_void;
    pub fn xmlNewTextChild(parent : *mut c_void, ns: *mut c_void, name: *const c_char, content: *const c_char) -> *mut c_void;
    pub fn xmlNewText(parent : *mut c_void, content: *const c_char) -> *mut c_void;
    pub fn xmlNewDocPI(doc: *mut c_void, name: *const c_char, content: *const  c_char) -> *mut c_void;
    // pub fn xmlFreeNs(cur: *mut c_void);
    // pub fn xmlNewDocFragment(doc: *mut c_void) -> *mut c_void;
    pub fn xmlDocGetRootElement(doc: *const c_void) -> *mut c_void;
    pub fn xmlDocSetRootElement(doc: *const c_void, root: *const c_void) -> *mut c_void;
    pub fn xmlGetProp(node: *const c_void, name: *const c_char) -> *const c_char;
    pub fn xmlNewProp(node: *const c_void, name: *const c_char, value: *const c_char) -> *mut c_void;

    //helper for tree
    pub fn xmlNextSibling(cur: *const c_void) -> *mut c_void;
    pub fn xmlPrevSibling(cur: *const c_void) -> *mut c_void;
    pub fn xmlAddPrevSibling(cur: *const c_void, new: *const c_void) -> *mut c_void;
    pub fn xmlRemovePropertyWithName(cur: *const c_void, name: *const c_char);
    pub fn xmlGetFirstChild(cur: *const c_void) -> *mut c_void;
    pub fn xmlGetParent(cur: *const c_void) -> *mut c_void;
    pub fn xmlNodeGetName(cur: *const c_void) -> *const c_char;
    pub fn xmlNodeGetContentPointer(cur: *const c_void) -> *const c_char;
    pub fn xmlNodeGetContent(cur: *const c_void) -> *const c_char;
    pub fn xmlNodeSetContent(node : *mut c_void, cur: *const c_char);
    pub fn xmlGetNodeType(cur: *const c_void) -> c_int;
    pub fn xmlBufferCreate() -> *mut c_void;
    pub fn xmlBufferFree(buf : *mut c_void);
    pub fn xmlBufferContent(buf : *mut c_void) -> *const c_char;
    pub fn xmlNodeDump(buf: *mut c_void, doc : *mut c_void, node: *mut c_void, indent: c_int, disable_format: c_int );
    //pub fn xmlDocDumpMemory(doc: *mut c_void, receiver: *mut *mut c_char, size: *const c_int, format: c_int );
    pub fn xmlDocDumpMemoryEnc(doc: *mut c_void, receiver: *mut *mut c_char, size: *const c_int,  encoding: *const c_char, format: c_int);

    //parser
    pub fn xmlReadFile(filename: *const c_char, encoding: *const c_char, options: c_uint) -> *mut c_void;
    // pub fn htmlParseFile(filename: *const c_char, encoding: *const c_char) -> *mut c_void;
    pub fn htmlReadFile(filename: *const c_char, encoding: *const c_char, options: c_uint) -> *mut c_void;
    // pub fn htmlReadDoc(html_string: *const c_char, url: *const c_char, encoding: *const c_char, options: c_uint) -> *mut c_void;
    pub fn xmlParseDoc(xml_string: *const c_char) -> *mut c_void;
    pub fn htmlParseDoc(xml_string: *const c_char, encoding: *const c_char) -> *mut c_void;
    pub fn htmlNewParserCtxt() -> *mut c_void;
    pub fn htmlCtxtReadDoc(ctxt: *mut c_void, html_string: *const c_char, url: *mut c_void, encoding: *const c_char, options: c_uint) -> *mut c_void;
    // pub fn htmlSAXParseDoc(xml_string: *const c_char, encoding: *const c_char, sax: *mut c_void, user_data: *mut c_void) -> *mut c_void;
    pub fn xmlInitParser();
    pub fn xmlCleanupParser();
    // pub fn xmlMemoryDump();
    pub fn xmlInitGlobals();
    pub fn xmlCleanupGlobals();
    pub fn xmlFree(some: *const c_char);
    pub fn xmlKeepBlanksDefault(flag : c_uint) -> c_uint;
    // pub fn xmlFreeParserCtxt(ctxt: *mut c_void);
    pub fn htmlFreeParserCtxt(ctxt: *mut c_void);
    // helper for parser
    pub fn htmlWellFormed(ctxt : *mut c_void) -> c_int;

    //xpath
    pub fn xmlXPathFreeContext(ctxt: *mut c_void);
    pub fn xmlXPathNewContext(doc: *mut c_void) -> *mut c_void;
    pub fn xmlXPathEvalExpression(str: *const c_char, ctxt: *mut c_void) -> *mut c_void;

    //helper for xpath
    pub fn xmlXPathObjectNumberOfNodes(val: *const c_void) -> c_int;
    pub fn xmlXPathObjectGetNode(val: *const c_void, index: size_t) -> *mut c_void;
    pub fn xmlFreeXPathObject(val: *const c_void);

    // error handling functions
    // pub fn xmlSetGenericErrorFunc(ctx: *mut c_void, handler: *mut c_void);
    // pub fn xmlThrDefSetGenericErrorFunc(ctx: *mut c_void, handler: *mut c_void);
    pub fn setWellFormednessHandler(ctxt: *mut c_void);
}
