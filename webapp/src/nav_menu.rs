use crate::prelude::*;

pub fn nav_content(contexts: &Contexts) -> Html {
    html! {
        <>
            <webui-flex justify="center" slot="header">
                <webui-stoic-dreams-logo title="Software Development Standards Logo" text="Web" text2="SSDev"></webui-stoic-dreams-logo>
            </webui-flex>
            <NavDisplay routes={get_nav_routing(contexts)} class="d-flex flex-column pa-1" />
        </>
    }
}

pub(crate) fn get_nav_routing(_contexts: &Contexts) -> Vec<NavRoute> {
    let nav_routes = vec![
        NavLinkInfo::link(
            "Home",
            "/continuous-agile-software-engineering",
            &FaIcon::duotone("house"),
            roles::PUBLIC,
            page_home,
        ),
        NavGroupInfo::link(
            "CASE Tenets",
            &FaIcon::duotone("books"),
            roles::PUBLIC,
            vec![
                NavLinkInfo::link(
                    "Keep it Simple",
                    "/tenets/keep_it_simple",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_keep_it_simple,
                ),
                NavLinkInfo::link(
                    "Code Ownership",
                    "/tenets/code_ownership",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_code_ownership,
                ),
                NavLinkInfo::link(
                    "Continuous Agility",
                    "/tenets/continuous_agility",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_agility,
                ),
                NavLinkInfo::link(
                    "Continuous Feedback Loops",
                    "/tenets/continuous_feedback_loops",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_feedback_loops,
                ),
                NavLinkInfo::link(
                    "Continuous Automation",
                    "/tenets/continuous_automation",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_automation,
                ),
                NavLinkInfo::link(
                    "Continuous Planning",
                    "/tenets/continuous_planning",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_planning,
                ),
                NavLinkInfo::link(
                    "Continuous Collaboration",
                    "/tenets/continuous_collaboration",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_collaboration,
                ),
                NavLinkInfo::link(
                    "Continuous Learning",
                    "/tenets/continuous_learning",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_learning,
                ),
                NavLinkInfo::link(
                    "Continuous Testing",
                    "/tenets/continuous_testing",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_testing,
                ),
                NavLinkInfo::link(
                    "Continuous Iterations",
                    "/tenets/continuous_iterations",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_iterations,
                ),
                NavLinkInfo::link(
                    "Continuous Integrations",
                    "/tenets/continuous_integrations",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_integrations,
                ),
                NavLinkInfo::link(
                    "Continuous Delivery",
                    "/tenets/continuous_delivery",
                    &FaIcon::duotone("circle"),
                    roles::PUBLIC,
                    page_tenet_continuous_delivery,
                ),
            ],
        ),
        NavLinkInfo::link(
            "Holistic",
            "/holistic",
            &FaIcon::duotone("infinity"),
            roles::PUBLIC,
            page_holistic,
        ),
        NavLinkInfo::link(
            "Agile",
            "/agile",
            &FaIcon::duotone("arrows-split-up-and-left"),
            roles::PUBLIC,
            page_agile,
        ),
        NavLinkInfo::link(
            "SOW",
            "/statement-of-work",
            &FaIcon::duotone("scroll"),
            roles::PUBLIC,
            page_statement_of_work,
        ),
        NavLinkInfo::link(
            "Standards",
            "/standards",
            &FaIcon::duotone("train-track"),
            roles::PUBLIC,
            page_standards,
        ),
        NavLinkInfo::link(
            "Tests",
            "/tests",
            &FaIcon::duotone("flask-vial"),
            roles::PUBLIC,
            page_tests,
        ),
        NavLinkInfo::link(
            "Interviews",
            "/interviews",
            &FaIcon::duotone("house"),
            roles::PUBLIC,
            page_interviews,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            &FaIcon::duotone("handshake"),
            roles::PUBLIC,
            starter_page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            &FaIcon::duotone("shield-exclamation"),
            roles::PUBLIC,
            starter_page_privacy,
        ),
        NavGroupInfo::link(
            "Hidden Nav",
            &FaIcon::regular("acorn"),
            roles::INVALID,
            vec![
                NavLinkInfo::link(
                    "Home",
                    "/",
                    &FaIcon::duotone("house"),
                    roles::PUBLIC,
                    page_index,
                ),
                NavLinkInfo::link(
                    "CASD",
                    "/continuous-agile-software-development",
                    &FaIcon::duotone("house"),
                    roles::PUBLIC,
                    page_index,
                ),
                NavLinkInfo::link(
                    "Simple",
                    "/simple",
                    &FaIcon::duotone("atom-simple"),
                    roles::PUBLIC,
                    page_simple,
                ),
            ],
        ),
    ];
    nav_routes.to_owned()
}
