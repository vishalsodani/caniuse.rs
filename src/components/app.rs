use stdweb::web::{document, HtmlElement, IHtmlElement};
use yew::{
    events::{IKeyboardEvent, KeyPressEvent},
    html,
    services::keyboard::{KeyListenerHandle, KeyboardService},
    Bridge, Bridged, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};

use crate::{
    components::{About, FeaturePage, Header, Index, VersionPage},
    AppRoute, FEATURES, VERSIONS,
};

pub struct App {
    link: ComponentLink<Self>,
    input_ref: NodeRef,
    router: Box<dyn Bridge<RouteAgent>>,
    search_query: String,

    _key_listener_handle: KeyListenerHandle,
}

pub enum Msg {
    Update,
    FocusInput,
    Search(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Update));

        let link2 = link.clone();
        let _key_listener_handle = KeyboardService::register_key_press(
            &document(),
            (move |e: KeyPressEvent| {
                if e.key().as_str() == "s" {
                    link2.callback(|_| Msg::FocusInput).emit(());
                }
            })
            .into(),
        );

        Self {
            link,
            input_ref: NodeRef::default(),
            router,
            search_query: String::new(),
            _key_listener_handle,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Update => true,
            Msg::FocusInput => {
                self.input_ref.cast::<HtmlElement>().unwrap().focus();
                false
            }
            Msg::Search(query) => {
                self.search_query = query;
                self.router.send(RouteRequest::ChangeRoute(Route::new_no_state("/")));

                // Re-render after routing, through Msg::Update
                false
            }
        }
    }

    fn view(&self) -> Html {
        type Router = yew_router::router::Router<AppRoute>;
        let search_query = self.search_query.clone();
        let render_route = Router::render(move |route| match route {
            AppRoute::Index => html! { <Index search_query=search_query.clone() /> },
            AppRoute::About => html! { <About /> },
            AppRoute::Feature(slug) => match FEATURES.iter().find(|f| f.slug == slug) {
                Some(&data) => html! { <FeaturePage data=data /> },
                None => html! { "error: feature not found!" },
            },
            AppRoute::Version(number) => match VERSIONS.iter().find(|v| v.number == number) {
                Some(&data) => html! { <VersionPage data=data /> },
                None => html! { "error: version not found!" },
            },
        });

        html! {
            <>
                <Header input_ref=self.input_ref.clone()
                    oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />
                <div class="page">
                    <Router render=render_route />
                </div>
            </>
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.link.send_message(Msg::FocusInput);
        false
    }
}
