use crate::prelude::*;

/// App home page
pub(crate) fn page_interviews(_contexts: Contexts) -> Html {
    set_title("Better interviewing for better Software Engineers");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/interview.md" />
            <NextPageButton url="/" />
        </>
    }
}
