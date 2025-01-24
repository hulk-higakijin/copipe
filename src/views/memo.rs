use loco_rs::prelude::*;

use crate::models::_entities::memos;

/// Render a list view of `memos`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<memos::Model>) -> Result<Response> {
    format::render().view(v, "memo/list.html", data!({"items": items}))
}

/// Render a single `memo` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &memos::Model) -> Result<Response> {
    format::render().view(v, "memo/show.html", data!({"item": item}))
}

/// Render a `memo` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "memo/create.html", data!({}))
}

/// Render a `memo` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &memos::Model) -> Result<Response> {
    format::render().view(v, "memo/edit.html", data!({"item": item}))
}
