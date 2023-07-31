use crate::prelude::*;

/// Page for tenet 9: Continuous Testing
pub(crate) fn page_tenet_continuous_testing(_contexts: Contexts) -> Html {
    set_title("Continuous Testing: Tenet 9 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-testing.md" />
            <NextPageButton url="/tenets/continuous_iterations" />
        </>
    }
}
