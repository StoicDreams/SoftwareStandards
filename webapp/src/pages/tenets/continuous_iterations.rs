use crate::prelude::*;

/// Page for tenet 10: Continuous Iterations
pub(crate) fn page_tenet_continuous_iterations(_contexts: Contexts) -> Html {
    set_title("Continuous Iterations: Tenet 10 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-iterations.md" />
            <NextPageButton url="/tenets/continuous_integrations" />
        </>
    }
}
