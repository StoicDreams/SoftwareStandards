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
                    {r#"Get your copy of my book "CASE: Continous Agile Software Engineering" for more information about Continuous Agile Software Engineering and how to integrate it into your teams and workflows."#}
                </Quote>
            </Paper>
            <Paper class="d-flex flex-row pa-2 gap-2 justify-center flex-wrap">
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Paper class="theme-secondary text-center pa-3 mb-2">{"Kindle"}</Paper>
                    <HtmlContent html={r#"
                    <iframe sandbox="allow-popups allow-scripts allow-modals allow-forms allow-same-origin" style="width:120px;height:240px;" marginwidth="0" marginheight="0" scrolling="no" frameborder="0" src="//ws-na.amazon-adsystem.com/widgets/q?ServiceVersion=20070822&OneJS=1&Operation=GetAdHtml&MarketPlace=US&source=ss&ref=as_ss_li_til&ad_type=product_link&tracking_id=erikgassler-20&language=en_US&marketplace=amazon&region=US&placement=B0C4FX1F1S&asins=B0C4FX1F1S&linkId=f90be5ed0aa2e0038a6cddb219a5487c&show_border=true&link_opens_in_new_window=true"></iframe>
                    "#} />
                </Paper>
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Paper class="theme-tertiary text-center pa-3 mb-2">{"Paperback"}</Paper>
                    <HtmlContent html={r#"
                    <iframe sandbox="allow-popups allow-scripts allow-modals allow-forms allow-same-origin" style="width:120px;height:240px;" marginwidth="0" marginheight="0" scrolling="no" frameborder="0" src="//ws-na.amazon-adsystem.com/widgets/q?ServiceVersion=20070822&OneJS=1&Operation=GetAdHtml&MarketPlace=US&source=ss&ref=as_ss_li_til&ad_type=product_link&tracking_id=erikgassler-20&language=en_US&marketplace=amazon&region=US&placement=B0C47LV1Y8&asins=B0C47LV1Y8&linkId=73a4b5d19008b9eac1c02ba85efcb29c&show_border=true&link_opens_in_new_window=true"></iframe>
                    "#} />
                </Paper>
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Paper class="theme-primary text-center pa-3 mb-2">{"Hardcover"}</Paper>
                    <HtmlContent html={r#"
                    <iframe sandbox="allow-popups allow-scripts allow-modals allow-forms allow-same-origin" style="width:120px;height:240px;" marginwidth="0" marginheight="0" scrolling="no" frameborder="0" src="//ws-na.amazon-adsystem.com/widgets/q?ServiceVersion=20070822&OneJS=1&Operation=GetAdHtml&MarketPlace=US&source=ss&ref=as_ss_li_til&ad_type=product_link&tracking_id=erikgassler-20&language=en_US&marketplace=amazon&region=US&placement=B0C47YGGYH&asins=B0C47YGGYH&linkId=ad750159f22ea299d1ed66f4b21538cc&show_border=true&link_opens_in_new_window=true"></iframe>
                    "#} />
                </Paper>
            </Paper>
        </Paper>
    }
}
