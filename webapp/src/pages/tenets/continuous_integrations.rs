use crate::prelude::*;

/// Page for tenet 11: Continuous Integrations
pub(crate) fn page_tenet_continuous_integrations(_contexts: &Contexts) -> Html {
    set_title("Continuous Integrations: Tenet 11 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-integrations.md" />
            <NextPageButton url="/tenets/continuous_delivery" />
        </>
    }
}
