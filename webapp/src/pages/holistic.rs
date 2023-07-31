use crate::prelude::*;

/// App home page
pub(crate) fn page_holistic(_contexts: Contexts) -> Html {
    set_title("Holistic practices for Agile Development");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/holistic.md" />
            <NextPageButton url="/agile" />
        </>
    }
}
