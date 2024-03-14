use crate::prelude::*;

/// Page for tenet 6: Continuous Planning
pub(crate) fn page_tenet_continuous_planning(_contexts: &Contexts) -> Html {
    set_title("Continuous Planning: Tenet 6 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-planning.md" />
            <NextPageButton url="/tenets/continuous_collaboration" />
        </>
    }
}
