use sapling::ast_spec::json::{JSONFormat, JSON};
use sapling::ast_spec::test_json::TestJSON;
use sapling::editable_tree::spec::Spec;
use sapling::editor::Editor;
use sapling::node_map::vec::{Index, VecNodeMap};

fn main() {
    // For the time being, start the editor with some pre-made JSON
    let start_node_map: VecNodeMap<JSON<Index>> = TestJSON::Array(vec![
        TestJSON::True,
        TestJSON::False,
        TestJSON::Object(vec![("value".to_string(), TestJSON::True)]),
    ])
    .build_node_map();
    let tree: Spec<JSON<Index>> = Spec::from_tree(start_node_map);
    let editor = Editor::new(tree, JSONFormat::Pretty);
    editor.run();
}
