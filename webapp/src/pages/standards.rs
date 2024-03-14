use crate::prelude::*;

/// App home page
pub(crate) fn page_standards(_contexts: &Contexts) -> Html {
    set_title("Team and Developer Standards");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/standards.md" />
            <NextPageButton url="/tests" />
        </>
    }
}
