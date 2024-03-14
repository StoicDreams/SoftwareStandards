use crate::prelude::*;

/// Page for tenet 2: Code Ownership
pub(crate) fn page_tenet_code_ownership(_contexts: &Contexts) -> Html {
    set_title("Code Ownership: Tenet 2 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/code-ownership.md" />
            <NextPageButton url="/tenets/continuous_agility" />
        </>
    }
}
