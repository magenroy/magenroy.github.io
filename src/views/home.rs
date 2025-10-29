use crate::components::Header;
use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}
        main { class: "content",
            About {}
            Seminars {}
            Resources {}
        }
        footer {}
    }
}

#[component]
pub fn About() -> Element {
    let TOA = rsx! { a { href: "https://icms.bg/research-groups/theory-of-atoms/", "Theory of Atoms" }};
    rsx! {
        section {
            id: "about",
            h2 {"About"}
            p {r"As of June 2025, I am working in the ", {TOA}, " research group of Professor Ludmil Katzarkov at the Institute of Mathematics and Informatics at the Bulgarian Academy of Sciences, Sofia, Bulgaria. I received my PhD in May 2025 from Columbia University, where I worked on motivic homotopy theories of stacks and functor formalisms under the supervision of Andrew Blumberg and Aise Johan de Jong. I am generally interested in various topics from homotopy theory and algebraic geometry, including K-theory, derived algebraic geometry, equivariant and global homotopy theory, and TQFTs and factorization homology."}
            // p { {TOA}, r"I'm a PhD student at Columbia working with Andrew Blumberg and Aise Johan de Jong. My current research focuses on equivariant and motivic homotopy theory, and in particular on six-functor formalisms (and variants) in the context of "stacky" geometry. I am also generally interested in various topics in homotopy theory and algebraic geometry, including K-theory, derived algebraic geometry, and TQFTs and factorization homology. My master's thesis on a variation of locally constant prefactorization algebras was supervised by Damien Calaque in 2019." }
        }
    }
}

#[component]
pub fn Seminars() -> Element {
    rsx! {
        section {
            id: "seminars",
            h2 {"Seminars"}
            p { "Seminars I have organized:" }
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
        section {
            id: "resources",
            h2 {"Resources"}
            p { "These are unorganized lists of resources on various topics that I decided to collect at some point in the beginning of my PhD" }
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
