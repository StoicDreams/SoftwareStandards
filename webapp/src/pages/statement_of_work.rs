use crate::prelude::*;

/// App home page
pub(crate) fn page_statement_of_work(_contexts: &Contexts) -> Html {
    set_title("Statements of Work - Identify the Problem");
    html! {
        <>
            <MarkdownContent href="/d/en-US/info/statement_of_work.md" />
            <NextPageButton url="/standards" />
        </>
    }
}
