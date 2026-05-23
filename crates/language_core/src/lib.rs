pub mod language_name;
pub use language_name::{LanguageId, LanguageName};
pub mod language_config;
pub use language_config::{
    BlockCommentConfig, BracketPair, BracketPairConfig, BracketPairContent, DecreaseIndentConfig,
    JsxTagAutoCloseConfig, LanguageConfig, LanguageConfigOverride, LanguageMatcher,
    OrderedListConfig, Override, SoftWrap, TaskListConfig, WrapCharactersConfig,
    auto_indent_using_last_non_empty_line_default, deserialize_regex, deserialize_regex_vec,
    regex_json_schema, regex_vec_json_schema, serialize_regex,
};
pub mod grammar;
pub use grammar::{
    BracketsConfig, BracketsPatternConfig, DebugVariablesConfig, DebuggerTextObject, Grammar,
    GrammarId, HighlightsConfig, IndentConfig, InjectionConfig, InjectionPatternConfig,
    NEXT_GRAMMAR_ID, OutlineConfig, OverrideConfig, OverrideEntry, RedactionConfig,
    RunnableCapture, RunnableConfig, TextObject, TextObjectConfig,
};
pub mod highlight_map;
pub use highlight_map::{HighlightId, HighlightMap};
pub mod queries;
pub use queries::{LanguageQueries, QUERY_FILENAME_PREFIXES};
