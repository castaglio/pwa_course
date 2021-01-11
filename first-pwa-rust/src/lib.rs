// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let elements = [
        "title".to_string(),
        "installable".to_string(),
        "offline".to_string(),
        "responsive".to_string(),
        "push".to_string(),
        "notifications".to_string(),
        "native-features".to_string(),
        "more".to_string(),
        "button".to_string(),
    ];
    let model = Model {
        visible: elements
            .iter()
            .map(|e| ((*e).clone(), true))
            .collect::<IndexMap<_, _>>(),
        toh: elements
            .iter()
            .map(|e| ((*e).clone(), None))
            .collect::<IndexMap<_, _>>(),
    };

    orders.perform_cmd(async { Msg::StartOver });

    model
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    visible: IndexMap<String, bool>,
    toh: IndexMap<String, Option<CmdHandle>>, //timeout handle hashmap
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    StartOver,
    OnTimer(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartOver => {
            let mut ms = 1000;
            model.visible.iter_mut().for_each(|(_k, v)| *v = false);
            for key in model.visible.keys() {
                let element = key.clone().to_string();
                model.toh[key] = Some(
                    orders.perform_cmd_with_handle(cmds::timeout(ms, || Msg::OnTimer(element))),
                );
                ms += 1000
            }
        }
        Msg::OnTimer(element) => {
            model.visible[&element] = true;
            model.toh[&element] = None;
        }
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![
            C!["title", IF!(model.visible["title"] => "animate-in")],
            h1!["Progressive Web Apps"],
        ],
        div![
            C!["content"],
            div![
                C![
                    "course-feature",
                    IF!(model.visible["installable"] => "animate-in")
                ],
                p!["They're installable (without and App Store)!"],
                attrs! { At::Id => "installable"},
            ],
            div![
                C![
                    "course-feature",
                    IF!(model.visible["offline"] => "animate-in")
                ],
                p!["They can work offline!"],
                attrs! { At::Id => "offline"},
            ],
            div![
                C![
                    "course-feature",
                    IF!(model.visible["responsive"] => "animate-in")
                ],
                p!["They look good on any device"],
                attrs! { At::Id => "responsive"},
            ],
            div![
                C!["course-feature", IF!(model.visible["push"] => "animate-in")],
                p!["You can receive Push Messages..."],
                attrs! { At::Id => "push"},
            ],
            div![
                C![
                    "course-feature",
                    IF!(model.visible["notifications"] => "animate-in")
                ],
                p!["...and show Notifications"],
                attrs! { At::Id => "notifications"},
            ],
            div![
                C![
                    "course-feature",
                    IF!(model.visible["native-features"] => "animate-in")
                ],
                p!["PWAs can access native device features like the Camera"],
                attrs! { At::Id => "native-features"},
            ],
            div![
                C!["course-feature", IF!(model.visible["more"] => "animate-in")],
                p!["And so much more!"],
                attrs! { At::Id => "more"},
            ],
        ],
        div![
            C!["start-over", IF!(model.visible[4] => "animate-in")],
            button![
                C!["button", IF!(model.visible["button"] => "animate-in")],
                "Start Again!"
            ],
            ev(Ev::Click, |_| Msg::StartOver),
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
