# rustrun-rs
The `cdylib`-style Rust DLL designed to be placed next to [RustRun](https://github.com/duckfromdiscord/RustRun/) in the PowerToys Run plugin folder. Heavily supported by research done in [csbindgen](https://github.com/Cysharp/csbindgen).

### Example

All plugins using this library MUST be a `cdylib`, and the simplest fits in a single file:

```
use rustrun::*;

const PLUGIN_ID: &str = "Plugin ID";
const PLUGIN_NAME: &str = "Name";
const PLUGIN_DESC: &str = "Desc";

rustrun::rustrun_info!(PLUGIN_ID, PLUGIN_NAME, PLUGIN_DESC);

/// Returns a list of search results for a given query.
pub fn search(query: String) -> Vec<SearchResult> {
    vec![]
}

/// Returns a list of context menu buttons for a given search result.
pub fn context_menu(query: SearchResult) -> Vec<ContextMenuResult> {
    vec![]
}

rustrun::impl_rustrun!(search, context_menu);

```