use crate::prelude::*;

pub(crate) fn render_case_books(_contexts: Contexts) -> Html {
    html! {
        <CaseBooks />
    }
}

#[function_component(CaseBooks)]
pub(crate) fn case_books() -> Html {
    html! {
        <Paper class={format!("{} gap-1", CLASSES_SIDE_BY_SIDE)}>
            <Paper class="d-flex flex-column justify-center">
                <Quote color={Theme::Success} elevation={ELEVATION_STANDARD}>
                    {r#"
                    Get your copy of my book "CASE: Continous Agile Software Engineering" for more information about Continuous Agile Software Engineering and how to integrate it into your teams and workflows.

                    Note: These links use an affiliate tracker that allow us to earn an extra commission when you make a purchase."#}
                </Quote>
            </Paper>
            <Paper class="d-flex flex-row pa-2 gap-2 justify-center flex-wrap">
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Link target="_blank" href="https://amzn.to/48S30VU">
                        <Paper class="theme-secondary text-center pa-3">{"Kindle"}</Paper>
                    </Link>
                </Paper>
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Link target="_blank" href="https://amzn.to/3Sgpyu3">
                        <Paper class="theme-tertiary text-center pa-3">{"Paperback"}</Paper>
                    </Link>
                </Paper>
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Link target="_blank" href="https://amzn.to/3vue8db">
                        <Paper class="theme-primary text-center pa-3">{"Hardcover"}</Paper>
                    </Link>
                </Paper>
            </Paper>
        </Paper>
    }
}
