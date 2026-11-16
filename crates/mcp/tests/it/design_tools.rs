//! The design tools: get_design_system, find_nodes and create_tree.

use crate::common::Client;
use serde_json::{Value, json};

fn card() -> Value {
    json!({
        "type": "frame", "name": "Card",
        "props": {"layout": "vertical", "padding": 16, "gap": 8, "fill": "#FFFFFF"},
        "children": [
            {"type": "text", "name": "Title", "text": "Trail report"},
            {"type": "frame", "name": "Row", "props": {"layout": "horizontal", "gap": 4}, "children": [
                {"type": "ellipse", "name": "Dot", "width": 8, "height": 8, "props": {"fill": "#3366FF"}},
                {"type": "text", "name": "Label", "text": "Open"}
            ]}
        ]
    })
}

#[test]
fn create_tree_builds_a_nested_design_in_one_undo_step() {
    let mut c = Client::new();
    let undo_before = c.doc()["history"]["undo"].as_array().map_or(0, Vec::len);
    let r = c.ok("create_tree", json!({"tree": card(), "label": "Build card"}));
    assert_eq!(r["layers"], 5);
    let root = r["root"].clone();
    assert_eq!(r["tree"]["id"], root);
    assert_eq!(r["tree"]["children"][1]["children"][0]["name"], "Dot");

    let node = c.node(&root);
    assert_eq!(node["name"], "Card");
    let kids: Vec<&str> = node["children"].as_array().unwrap().iter().map(|k| k["name"].as_str().unwrap()).collect();
    assert_eq!(kids, ["Title", "Row"]);
    assert_eq!(node["children"][1]["children"][1]["characters"], "Open");

    let doc = c.doc();
    assert_eq!(doc["selection"], json!([root]), "the root becomes the selection");
    let undo = doc["history"]["undo"].as_array().unwrap();
    assert_eq!(undo.len(), undo_before + 1, "one undo step: {undo:?}");
    c.ok("undo", json!({}));
    assert!(c.page_children().is_empty(), "undo takes the whole tree back");
}

#[test]
fn create_tree_makes_instances_and_goes_inside_a_parent() {
    let mut c = Client::new();
    let button = c.create(json!({"type": "component", "name": "Button", "width": 80, "height": 32}));
    let page = c.create(json!({"type": "frame", "name": "Screen", "x": 400, "width": 300, "height": 300}));
    let r = c.ok(
        "create_tree",
        json!({"parent": page, "select": false, "tree": {"type": "frame", "name": "Actions", "props": {"layout": "horizontal"}, "children": [
            {"component": button, "name": "Primary"},
            {"component": button, "name": "Secondary", "width": 120}
        ]}}),
    );
    let actions = c.node(&r["root"]);
    let parent = c.node(&page);
    assert_eq!(parent["children"][0]["id"], actions["id"]);
    for (child, name) in actions["children"].as_array().unwrap().iter().zip(["Primary", "Secondary"]) {
        assert_eq!(child["type"], "INSTANCE");
        assert_eq!(child["name"], name);
    }
    assert_eq!(actions["children"][1]["width"].as_f64(), Some(120.0));
    assert_ne!(c.doc()["selection"], json!([r["root"]]), "select: false keeps the selection");
}

#[test]
fn create_tree_refuses_bad_trees_and_creates_nothing() {
    let mut c = Client::new();
    for (tree, says) in [
        (json!({"name": "No type"}), "`type` (or `component`"),
        (json!({"type": "frame", "colour": "#000"}), "unknown key `colour`"),
        (json!({"type": "frame", "component": 1}), "not both"),
        (json!({"type": "frame", "children": [{"type": "frame", "children": "x"}]}), "tree.children[0]: `children` must be an array"),
        (json!({"type": "frame", "children": [{"type": "nope"}]}), "nope"),
        (json!({"component": 99999, "text": "x"}), "not an instance"),
    ] {
        let e = c.err("create_tree", json!({"tree": tree}));
        assert!(e.contains(says), "{tree}: {e}");
    }
    assert!(c.page_children().is_empty(), "a failed tree leaves nothing behind");

    let mut deep = json!({"type": "frame"});
    for _ in 0..40 {
        deep = json!({"type": "frame", "children": [deep]});
    }
    assert!(c.err("create_tree", json!({"tree": deep})).contains("deeper than"));
    let wide = json!({"type": "frame", "children": vec![json!({"type": "rectangle"}); 600]});
    assert!(c.err("create_tree", json!({"tree": wide})).contains("more than 500 layers"));
    assert!(c.err("create_tree", json!({"tree": [1]})).contains("`tree` must be a layer object"));
}

#[test]
fn find_nodes_searches_by_name_type_and_text_without_selecting() {
    let mut c = Client::new();
    let r = c.ok("create_tree", json!({"tree": card()}));
    let root = r["root"].clone();
    c.ok("select", json!({"ids": [root]}));

    let texts = c.ok("find_nodes", json!({"type": "TEXT"}));
    assert_eq!(texts["total"], 2);
    let names: Vec<&str> = texts["matches"].as_array().unwrap().iter().map(|m| m["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["Title", "Label"], "tree order");
    assert_eq!(texts["matches"][1]["path"], "Card / Row");

    let open = c.ok("find_nodes", json!({"text": "OPEN"}));
    assert_eq!(open["total"], 1);
    assert_eq!(open["matches"][0]["characters"], "Open");

    let rows = c.ok("find_nodes", json!({"name": "ro", "type": "frame"}));
    assert_eq!(rows["matches"].as_array().unwrap().len(), 1);
    assert_eq!(rows["matches"][0]["name"], "Row");

    let limited = c.ok("find_nodes", json!({"limit": 2}));
    assert_eq!((limited["total"].as_u64(), limited["shown"].as_u64()), (Some(5), Some(2)));
    assert!(limited["hint"].is_string());

    assert_eq!(c.doc()["selection"], json!([root]), "searching doesn't select");
}

#[test]
fn find_nodes_reaches_other_pages_only_when_asked() {
    let mut c = Client::new();
    c.create(json!({"type": "rectangle", "name": "First page box"}));
    c.run("page.add", json!({"name": "Second"}));
    c.create(json!({"type": "rectangle", "name": "Second page box"}));
    let here = c.ok("find_nodes", json!({"name": "page box"}));
    assert_eq!(here["total"], 1);
    assert_eq!(here["matches"][0]["pageName"], "Second");
    let all = c.ok("find_nodes", json!({"name": "page box", "allPages": true}));
    assert_eq!(all["total"], 2);
    assert!(c.err("find_nodes", json!({"allPages": "yes"})).contains("boolean"));
}

#[test]
fn get_design_system_reads_every_part_in_one_call() {
    let mut c = Client::new();
    let swatch = c.create(json!({"type": "rectangle", "name": "Swatch", "props": {"fill": "#3366FF"}}));
    c.run("style.create", json!({"type": "paint", "name": "Brand/Primary", "from": swatch}));
    c.create(json!({"type": "component", "name": "Button"}));
    let ds = c.ok("get_design_system", json!({}));
    for part in ["pages", "styles", "variables", "components"] {
        assert!(!ds[part].is_null() && ds[part].get("error").is_none(), "{part}: {ds}");
    }
    assert!(ds["styles"].to_string().contains("Brand/Primary"));
    assert_eq!(ds["components"][0]["name"], "Button");
    assert_eq!(ds["pages"].as_array().unwrap().len(), 1);

    let only = c.ok("get_design_system", json!({"include": ["components"]}));
    assert_eq!(only.as_object().unwrap().keys().collect::<Vec<_>>(), ["components"]);
    assert!(c.err("get_design_system", json!({"include": ["fonts"]})).contains("`include` takes"));
}
