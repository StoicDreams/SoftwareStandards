use crate::prelude::*;

/// Page for tenet 4: Continuous Feedback Loops
pub(crate) fn page_tenet_continuous_feedback_loops(_contexts: &Contexts) -> Html {
    set_title("Continuous Feedback Loops: Tenet 4 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-feedback-loops.md" />
            <NextPageButton url="/tenets/continuous_automation" />
        </>
    }
}
