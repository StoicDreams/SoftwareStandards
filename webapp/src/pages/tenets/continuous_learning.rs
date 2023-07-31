use crate::prelude::*;

/// Page for tenet 8: Continuous Learning
pub(crate) fn page_tenet_continuous_learning(_contexts: Contexts) -> Html {
    set_title("Continuous Learning: Tenet 8 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-learning.md" />
            <NextPageButton url="/tenets/continuous_testing" />
        </>
    }
}
