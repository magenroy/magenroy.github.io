use crate::components::Header;
use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "main_content",
            Header {}
            div {
            class: "content",
                About {}
                Seminars {}
                Resources {}
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            id: "about",
            h3 {"About"}
            p { r" I'm a PhD student at Columbia working with Andrew Blumberg and Aise Johan de Jong. My current research focuses on equivariant and motivic homotopy theory, and in particular on six-functor formalisms (and variants) in the context of "stacky" geometry. I am also generally interested in various topics in homotopy theory and algebraic geometry, including K-theory, derived algebraic geometry, and TQFTs and factorization homology. My master's thesis on a variation of locally constant prefactorization algebras was supervised by Damien Calaque in 2019." }
        }
    }
}

#[component]
pub fn Seminars() -> Element {
    rsx! {
            div {
                id: "seminars",
                h3 {"Seminars"}
            ul {
                li {
                a { href: "https://math.columbia.edu/~magenroy/DAG-seminar.html", "Derived algebraic geometry seminar at Columbia University" }
            }
                li {
                a { href: "https://math.columbia.edu/~magenroy/motivicseminar.html", "Motivic homotopy theory seminar at Columbia University" }
            }
                li {
                a { href: "https://math.columbia.edu/~magenroy/MilnorWittMotivesSeminar.html", "Milnor-Witt motives seminar at Columbia University" }
            }
        }
    }
}
}

#[component]
pub fn Resources() -> Element {
    rsx! {
            div {
                id: "resources",
                h3 {"Resources"}
            ul {
                li {
                a { href: "https://math.columbia.edu/~magenroy/resources.html", "Unsorted resources" }
            }
                li {
                a { href: "https://math.columbia.edu/~magenroy/DAG-resources.html", "DAG/SAG resources" }
            }
                li {
                a { href: "https://math.columbia.edu/~magenroy/Motives-resources.html", "Motives resources" }
            }
        }
    }
}
}
