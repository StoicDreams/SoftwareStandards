use crate::prelude::*;

pub(crate) fn page_index(_contexts: Contexts) -> Html {
    push_state("/continuous-agile-software-engineering");
    page_home(_contexts)
}

/// App home page
pub(crate) fn page_home(_contexts: Contexts) -> Html {
    set_title(
        "Software Development Standards for Agile Development with Continuous Delivery Workflows",
    );
    html! {
        <>
            <MarkdownContent href="/d/en-US/case.md" />
            <NextPageButton url="/tenets/keep_it_simple" />
        </>
    }
}
