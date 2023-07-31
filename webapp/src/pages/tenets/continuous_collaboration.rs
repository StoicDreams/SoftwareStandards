use crate::prelude::*;

/// Page for tenet 7: Continuous Collaboration
pub(crate) fn page_tenet_continuous_collaboration(_contexts: Contexts) -> Html {
    set_title("Continuous Collaboration: Tenet 7 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-collaboration.md" />
            <NextPageButton url="/tenets/continuous_learning" />
        </>
    }
}
