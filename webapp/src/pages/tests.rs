use crate::prelude::*;

/// App home page
pub(crate) fn page_tests(_contexts: Contexts) -> Html {
    set_title("Test Strategies for Continuous Development");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/tests.md" />
            <NextPageButton url="/interviews" />
        </>
    }
}
