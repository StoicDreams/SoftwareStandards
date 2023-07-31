use crate::prelude::*;

/// Page for tenet 1: Keep it Simple
pub(crate) fn page_tenet_keep_it_simple(_contexts: Contexts) -> Html {
    set_title("Keep it Simple: Tenet 1 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/keep-it-simple.md" />
            <NextPageButton url="/tenets/code_ownership" />
        </>
    }
}
