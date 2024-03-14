use crate::prelude::*;

/// App home page
pub(crate) fn page_agile(_contexts: &Contexts) -> Html {
    set_title("Agile Software Software Development");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/agile.md" />
            <NextPageButton url="/statement-of-work" />
        </>
    }
}
