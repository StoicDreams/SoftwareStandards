use crate::prelude::*;

/// Page for tenet 3: Continuous Agility
pub(crate) fn page_tenet_continuous_agility(_contexts: Contexts) -> Html {
    set_title("Continuous Agility: Tenet 3 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-agility.md" />
            <NextPageButton url="/tenets/continuous_feedback_loops" />
        </>
    }
}
