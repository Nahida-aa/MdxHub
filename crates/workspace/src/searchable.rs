use crate::{WeakItemHandle, item::ItemHandle};
use any_vec::AnyVec;
use gpui::{AnyWeakEntity, App, Subscription, Window};
use project::search::SearchQuery;
use settings::SeedQuerySetting;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchToken(u64);

#[derive(Debug, Clone)]
pub enum SearchEvent {
    MatchesInvalidated,
    ActiveMatchChanged,
}

pub trait SearchableItemHandle: ItemHandle {
    fn downgrade(&self) -> Box<dyn WeakSearchableItemHandle>;
    fn boxed_clone(&self) -> Box<dyn SearchableItemHandle>;
    fn supported_options(&self, cx: &App) -> SearchOptions;
    fn subscribe_to_search_events(
        &self,
        window: &mut Window,
        cx: &mut App,
        handler: Box<dyn Fn(&SearchEvent, &mut Window, &mut App) + Send>,
    ) -> Subscription;
    fn clear_matches(&self, window: &mut Window, cx: &mut App);
    fn update_matches(
        &self,
        matches: &AnyVec<dyn Send>,
        active_match_index: Option<usize>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut App,
    );
    fn query_suggestion(
        &self,
        seed_query_override: Option<SeedQuerySetting>,
        window: &mut Window,
        cx: &mut App,
    ) -> String;
    fn activate_match(
        &self,
        index: usize,
        matches: &AnyVec<dyn Send>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut App,
    );
    fn select_matches(
        &self,
        matches: &AnyVec<dyn Send>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut App,
    );
    fn replace(
        &self,
        _: any_vec::element::ElementRef<'_, dyn Send>,
        _: &SearchQuery,
        token: SearchToken,
        _window: &mut Window,
        _: &mut App,
    );
    fn replace_all(
        &self,
        matches: &mut dyn Iterator<Item = any_vec::element::ElementRef<'_, dyn Send>>,
        query: &SearchQuery,
        token: SearchToken,
        window: &mut Window,
        cx: &mut App,
    );
    fn match_index_for_direction(
        &self,
        matches: &AnyVec<dyn Send>,
        current_index: usize,
        direction: Direction,
        count: usize,
        token: SearchToken,
        window: &mut Window,
        cx: &mut App,
    ) -> usize;
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<AnyVec<dyn Send>>;
    fn find_matches_with_token(
        &self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<(AnyVec<dyn Send>, SearchToken)>;
    fn active_match_index(
        &self,
        direction: Direction,
        matches: &AnyVec<dyn Send>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize>;
    fn search_bar_visibility_changed(&self, visible: bool, window: &mut Window, cx: &mut App);

    fn toggle_filtered_search_ranges(
        &mut self,
        enabled: Option<FilteredSearchRange>,
        window: &mut Window,
        cx: &mut App,
    );

    fn set_search_is_case_sensitive(&self, is_case_sensitive: Option<bool>, cx: &mut App);
}

pub trait WeakSearchableItemHandle: WeakItemHandle {
    fn upgrade(&self, cx: &App) -> Option<Box<dyn SearchableItemHandle>>;

    fn into_any(self) -> AnyWeakEntity;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchOptions {
    pub case: bool,
    pub word: bool,
    pub regex: bool,
    /// Specifies whether the  supports search & replace.
    pub replacement: bool,
    pub selection: bool,
    pub select_all: bool,
    pub find_in_results: bool,
}
// Whether to always select the current selection (even if empty)
// or to use the default (restoring the previous search ranges if some,
// otherwise using the whole file).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FilteredSearchRange {
    Selection,
    #[default]
    Default,
}
