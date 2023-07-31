use crate::prelude::*;

/// App home page
pub(crate) fn page_simple(_contexts: Contexts) -> Html {
    set_title("Simple Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/simple.md" />
            <NextPageButton url="/holistic" />
        </>
    }
}
