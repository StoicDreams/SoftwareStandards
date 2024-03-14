use crate::prelude::*;

/// Page for tenet 5: Continuous Automation
pub(crate) fn page_tenet_continuous_automation(_contexts: &Contexts) -> Html {
    set_title("Continuous Automation: Tenet 5 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-automation.md" />
            <NextPageButton url="/tenets/continuous_planning" />
        </>
    }
}
