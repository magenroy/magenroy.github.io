use dioxus::prelude::*;

#[component]
pub fn Seminars() -> Element {
    // let path = SEMINAR_DIR.resolve();
    // let Content: Element = match std::fs::read_dir(path) {
    //     Ok(x) => rsx! {
    //         "a"
    //     },
    //     Err(_) => rsx! {
    //         "b"
    //     },
    // }; 

    rsx! {
            h2 {"Seminars"}
            p { "Seminars I have organized:" }
            // {Content}
            ul {
                li {
                    // a { href: "https://math.columbia.edu/~magenroy/DAG-seminar.html", "Derived algebraic geometry seminar at Columbia University" }
                    a { href: "Columbia/DAG-seminar.html", "Derived algebraic geometry seminar at Columbia University" }
                }
                li {
                    // a { href: "https://math.columbia.edu/~magenroy/motivicseminar.html", "Motivic homotopy theory seminar at Columbia University" }
                    a { href: "Columbia/motivicseminar.html", "Motivic homotopy theory seminar at Columbia University" }
                }
                li {
                    // a { href: "https://math.columbia.edu/~magenroy/MilnorWittMotivesSeminar.html", "Milnor-Witt motives seminar at Columbia University" }
                    a { href: "Columbia/MilnorWittMotivesSeminar.html", "Milnor-Witt motives seminar at Columbia University" }
                }
            }
        }
}
