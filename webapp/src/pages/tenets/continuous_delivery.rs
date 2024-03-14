use crate::prelude::*;

/// Page for tenet 12: Continuous Delivery
pub(crate) fn page_tenet_continuous_delivery(_contexts: &Contexts) -> Html {
    set_title("Continuous Delivery: Tenet 12 of Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/tenets/continuous-delivery.md" />
            <NextPageButton url="/tenets/keep_it_simple" />
        </>
    }
}
