use crate::prelude::*;

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo::builder(
        "Navigation Menu",
        || html! {<i class="fa-solid fa-bars"></i>},
        DynHtml::new(nav_menu_render),
    )
    .set_button_class("btn toggle theme-inherit")
    .hide_header()
    .hide_footer()
    .set_drawer(Direction::Left)
    .build()
}

pub(crate) fn get_nav_routing() -> Vec<NavRoute> {
    let nav_routes = vec![
        NavLinkInfo::link(
            "Home",
            "/continuous-agile-software-engineering",
            "fa-duotone fa-fw fa-house",
            roles::PUBLIC,
            page_home,
        ),
        NavGroupInfo::link(
            "CASE Tenets",
            "fa-duotone fa-fw fa-books",
            roles::PUBLIC,
            vec![
                NavLinkInfo::link(
                    "Keep it Simple",
                    "/tenets/keep_it_simple",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_keep_it_simple,
                ),
                NavLinkInfo::link(
                    "Code Ownership",
                    "/tenets/code_ownership",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_code_ownership,
                ),
                NavLinkInfo::link(
                    "Continuous Agility",
                    "/tenets/continuous_agility",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_agility,
                ),
                NavLinkInfo::link(
                    "Continuous Feedback Loops",
                    "/tenets/continuous_feedback_loops",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_feedback_loops,
                ),
                NavLinkInfo::link(
                    "Continuous Automation",
                    "/tenets/continuous_automation",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_automation,
                ),
                NavLinkInfo::link(
                    "Continuous Planning",
                    "/tenets/continuous_planning",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_planning,
                ),
                NavLinkInfo::link(
                    "Continuous Collaboration",
                    "/tenets/continuous_collaboration",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_collaboration,
                ),
                NavLinkInfo::link(
                    "Continuous Learning",
                    "/tenets/continuous_learning",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_learning,
                ),
                NavLinkInfo::link(
                    "Continuous Testing",
                    "/tenets/continuous_testing",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_testing,
                ),
                NavLinkInfo::link(
                    "Continuous Iterations",
                    "/tenets/continuous_iterations",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_iterations,
                ),
                NavLinkInfo::link(
                    "Continuous Integrations",
                    "/tenets/continuous_integrations",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_integrations,
                ),
                NavLinkInfo::link(
                    "Continuous Delivery",
                    "/tenets/continuous_delivery",
                    "fa-duotone fa-fw fa-circle",
                    roles::PUBLIC,
                    page_tenet_continuous_delivery,
                ),
            ],
        ),
        NavLinkInfo::link(
            "Holistic",
            "/holistic",
            "fa-duotone fa-fw fa-infinity",
            roles::PUBLIC,
            page_holistic,
        ),
        NavLinkInfo::link(
            "Agile",
            "/agile",
            "fa-duotone fa-fw fa-arrows-split-up-and-left",
            roles::PUBLIC,
            page_agile,
        ),
        NavLinkInfo::link(
            "SOW",
            "/statement-of-work",
            "fa-duotone fa-fw fa-scroll",
            roles::PUBLIC,
            page_statement_of_work,
        ),
        NavLinkInfo::link(
            "Standards",
            "/standards",
            "fa-duotone fa-fw fa-train-track",
            roles::PUBLIC,
            page_standards,
        ),
        NavLinkInfo::link(
            "Tests",
            "/tests",
            "fa-duotone fa-fw fa-flask-vial",
            roles::PUBLIC,
            page_tests,
        ),
        NavLinkInfo::link(
            "Interviews",
            "/interviews",
            "fa-duotone fa-fw fa-house",
            roles::PUBLIC,
            page_interviews,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            "fa-duotone fa-fw fa-handshake",
            roles::PUBLIC,
            starter_page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            "fa-duotone fa-fw fa-shield-exclamation",
            roles::PUBLIC,
            starter_page_privacy,
        ),
        NavGroupInfo::link(
            "Hidden Nav",
            "fa-acorn",
            roles::INVALID,
            vec![
                NavLinkInfo::link(
                    "Home",
                    "/",
                    "fa-duotone fa-fw fa-house",
                    roles::PUBLIC,
                    page_index,
                ),
                NavLinkInfo::link(
                    "CASD",
                    "/continuous-agile-software-development",
                    "fa-duotone fa-fw fa-house",
                    roles::PUBLIC,
                    page_index,
                ),
                NavLinkInfo::link(
                    "Simple",
                    "/simple",
                    "fa-duotone fa-fw fa-atom-simple",
                    roles::PUBLIC,
                    page_simple,
                ),
            ],
        ),
    ];
    nav_routes.to_owned()
}

fn nav_menu_render() -> Html {
    html! {
        <>
            <Paper class="d-flex pa-1 justify-center">
                <img src="Logo.svg" title="Web UI Logo" />
            </Paper>
            <NavDisplay routes={get_nav_routing()} class="d-flex flex-column pa-1" />
        </>
    }
}
