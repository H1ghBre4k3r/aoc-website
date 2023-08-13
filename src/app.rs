use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/aoc_website.css"/>

        <Title text="Advent of Code"/>

        <Meta name="color-scheme" content="light" />
        <Meta name="viewport" content="width=device-width; initial-scale=1.0;" />

        <Script src="https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js"/>
        <Script src="https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"/>

        <Router>
            <Navigation />
            <main>
                <Routes>
                    <Route path="" view=MainView/>
                    <Route path="/code" view=CodeView/>
                    <Route path="/code/:user" view=CodeView/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn MainView(cx: Scope) -> impl IntoView {
    view! { cx,
        <section>
            <h1>Advent of Code</h1>
        </section>
    }
}

#[component]
fn CodeView(cx: Scope) -> impl IntoView {
    let query = use_params_map(cx);
    let user = move || query.with(|params| params.get("user").cloned().unwrap_or_default());

    view! { cx,
        <Sidebar />
        <Show when=move || user().trim() != "" fallback=move |cx| view! { cx, <section>Select a user...</section>}>
            {highlight_all()}
            <section class="code-overview">
                <ul>
                    <li>
                        <div class="code-snippet">
                            <details open>
                                <summary>
                                    {user} Part 1
                                </summary>
                                <pre>
                                    <code class="language-rust">{CODE.trim()}</code>
                                </pre>
                            </details>
                        </div>
                    </li>
                    <li>
                        <div class="code-snippet">
                            <details>
                                <summary>
                                    {user} Part 2
                                </summary>
                                <pre>
                                    <code class="language-rust">{CODE.trim()}</code>
                                </pre>
                            </details>
                        </div>
                    </li>
                </ul>
            </section>
        </Show>
    }
}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "hydrate")]
#[wasm_bindgen(module = "/js/prism.js")]
extern "C" {
    pub fn highlight_all();
}

#[cfg(not(feature = "hydrate"))]
#[allow(dead_code)]
pub fn highlight_all() {}

#[component]
fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <div class="logo">
                <a href="/">
                    AoC
                </a>
            </div>
            <ul>
                <li>
                    <a href="/">
                        <span class="icon"><Svg id="home"/></span>
                        <span class="nav-label">Home</span>
                    </a>
                </li>
                <li>
                    <a href="/code">
                        <span class="icon"><Svg id="code-brackets"/></span>
                        <span class="nav-label">Code</span>
                    </a>
                </li>
                <li>
                    <a href="/last-years">
                        <span class="nav-label">Last Years</span>
                    </a>
                </li>
            </ul>
            <div class="profile">
                <details>
                    <summary>
                        <span class="nav-label">H1ghBre4k3r</span>
                        <span class="profile-picture">
                            <Svg id="user-circle" />
                        </span>
                    </summary>
                    <aside>
                        <ul>
                            <li>
                                <a href="/profile">
                                    <span class="icon"><Svg id="tools" /></span>Profile
                                </a>
                            </li>
                            <li>
                                <a href="/settings">
                                    <span class="icon"><Svg id="settings" /></span>Settings
                                </a>
                            </li>
                            <li>
                                <a href="/logout">
                                    <span class="icon"><Svg id="logout" /></span>Logout
                                </a>
                            </li>
                        </ul>
                    </aside>
                </details>
            </div>
        </nav>
    }
}

#[component]
fn Svg<'a>(cx: Scope, id: &'a str) -> impl IntoView {
    let url = format!("/assets/icons.svg#{id}");

    view! { cx,
        <svg viewBox="0 0 24 24">
            <use_ href=url/>
        </svg>
    }
}

#[component]
fn Sidebar(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let user = move || params.with(|params| params.get("user").cloned().unwrap_or_default());

    let names = vec![
        "h1ghbre4k3r",
        "dobiko",
        "dormanil",
        "maclement",
        "melf",
        "zihark",
        "sebfisch",
        "xtay2",
        "estugon",
        "fwcd",
        "b3z",
        "felioh",
        "h1tchhiker",
        "hendrick404",
        "tuhhy",
        "yorick",
        "skgland",
    ];

    let (users, _) = create_signal(cx, names);

    view! { cx,
        <section class="sidebar">
            <div>
                <header><h3>Users</h3></header>
                <div class="day">
                    <label for="day-select">Day</label>
                    <select name="day" id="day-select">
                        <option value="1">1</option>
                        <option value="2">2</option>
                        <option value="3">3</option>
                        <option value="4">4</option>
                        <option value="5">5</option>
                        <option value="6">6</option>
                        <option value="7">7</option>
                        <option value="8">8</option>
                    </select>
                </div>
                <ul>
                    <For each=users key=|name| name.to_owned() view=move|cx, name| {
                        let is_active = move || name == user();
                        let link = move || format!("/code/{name}");

                        view! {cx,
                            <li>
                                <a href=link class:active=is_active>{name}</a>
                            </li>
                        }
                    }/>
                </ul>
            </div>
        </section>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <section>
            <h1>"Not Found"</h1>
        </section>
    }
}

const CODE: &str = r#"
use std::{error::Error, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
struct ParseError;

/// Struct representing an item within a stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().chars().nth(1) {
            Some(val) => Ok(Self(val)),
            None => Err(ParseError),
        }
    }
}

/// Struct for representing a stack of crates.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Stack {
    items: Vec<Item>,
}

impl Stack {
    /// Push an element onto this stack.
    pub fn push(&mut self, item: Item) {
        self.items.push(item);
    }

    /// Pop an element from this stack.
    pub fn pop(&mut self) -> Option<Item> {
        self.items.pop()
    }

    /// Pop n elements of this stack.
    pub fn pop_n(&mut self, n: usize) -> Vec<Option<Item>> {
        let mut items = vec![];

        for _ in 0..n {
            items.insert(0, self.items.pop());
        }

        items
    }

    /// Push all provided elements into this stack.
    pub fn push_all(&mut self, items: Vec<Item>) {
        self.items.append(&mut items.clone());
    }
}

/// Struct for representing an instruction to move a specified amount of crates from one stack to
/// the other.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Instruction(usize, usize, usize);

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut whitespaces = s.split_whitespace();

        // the instructions usually have the form of:
        // 'move X from Y to Z'
        //
        // so we need the 1., 3. and 5. index, therefore always taking the 1. index, since
        // `nth(1)` advances the iterator by 2 elements.
        let amount = whitespaces.nth(1);
        let source = whitespaces.nth(1);
        let destination = whitespaces.nth(1);

        let (Some(amount), Some(source), Some(destination)) = (amount, source, destination) else {
            panic!("AoC betrayed us! ({:?} {:?} {:?})", amount, source, destination);
        };

        let amount = amount.parse::<usize>()?;
        let source = source.parse::<usize>()?;
        let destination = destination.parse::<usize>()?;
        Ok(Self(amount, source, destination))
    }
}

/// Split a row of the stack inputs into chunks.
fn split_row_input_into_chunks(inp: &str) -> Vec<Option<Item>> {
    inp.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>().trim().parse::<Item>().ok())
        .collect::<Vec<_>>()
}

/// Parse input into stacks and instructions.
#[aoc_generator(day5)]
fn generator_day5(inp: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let Some((crates, instructions)) = inp.split_once("\n\n") else {
        panic!("AoC betrayed us!")
    };

    // determine the amount of stacks we need to fill
    let mut stack_lines = crates.lines().rev();
    let num_stacks =
        stack_lines
            .next()
            .unwrap()
            .chars()
            .fold(0, |memo, c| if c.is_numeric() { memo + 1 } else { memo });
    let mut stacks = vec![Stack::default(); num_stacks];

    // ...and then fill them :D
    for line in stack_lines {
        for (i, ele) in split_row_input_into_chunks(line).iter().enumerate() {
            if let Some(item) = *ele {
                stacks[i].push(item);
            }
        }
    }

    (
        stacks,
        // some fancy string parsing etc.
        instructions
            .lines()
            .map(|line| {
                line.parse::<Instruction>()
                    .expect("Parsing not successful :(")
            })
            .collect::<Vec<_>>(),
    )
}

#[aoc(day5, part1)]
fn day05_part1(input: &(Vec<Stack>, Vec<Instruction>)) -> String {
    let (mut stacks, instructions) = input.clone();

    // move crates around
    for Instruction(amount, source, target) in instructions {
        for _ in 0..amount {
            let item = stacks[source - 1].pop().expect("we are empty....");
            stacks[target - 1].push(item);
        }
    }

    // combine top elements
    stacks
        .iter_mut()
        .map(|stack| stack.pop())
        .fold("".to_owned(), |mut memo, current| {
            if let Some(c) = current {
                memo.push(c.0);
            }
            memo
        })
}

#[aoc(day5, part2)]
fn day05_part2(input: &(Vec<Stack>, Vec<Instruction>)) -> String {
    let (mut stacks, instructions) = input.clone();

    // move crates around
    for Instruction(amount, source, target) in instructions {
        let items = stacks[source - 1].pop_n(amount);
        stacks[target - 1].push_all(items.into_iter().flatten().collect());
    }

    // combine top elements
    stacks
        .iter_mut()
        .map(|stack| stack.pop())
        .fold("".to_owned(), |mut memo, current| {
            if let Some(c) = current {
                memo.push(c.0);
            }
            memo
        })
}

#[cfg(test)]
mod tests {

    use crate::day_05::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_item() {
        assert_eq!("[D]".parse::<Item>().unwrap(), Item('D'));
    }

    #[test]
    fn test_split_input_into_chunks() {
        assert_eq!(split_row_input_into_chunks("[A]"), vec![Some(Item('A'))]);
        assert_eq!(
            split_row_input_into_chunks("[A]    "),
            vec![Some(Item('A')), None]
        );
        assert_eq!(
            split_row_input_into_chunks("[A]     [B]"),
            vec![Some(Item('A')), None, Some(Item('B'))]
        );
        assert_eq!(
            split_row_input_into_chunks("    [A]"),
            vec![None, Some(Item('A'))]
        );
        assert_eq!(
            split_row_input_into_chunks("    [A]    "),
            vec![None, Some(Item('A')), None]
        )
    }

    #[test]
    fn test_pop_n() {
        let mut stack = Stack {
            items: vec![Item('A'), Item('B')],
        };
        let items = stack.pop_n(2);
        assert_eq!(items, vec![Some(Item('A')), Some(Item('B'))]);
        assert!(stack.items.is_empty());
    }

    #[test]
    fn test_push_n() {
        let mut stack = Stack::default();
        stack.push_all(vec![Item('A'), Item('B')]);
        assert_eq!(stack.items, vec![Item('A'), Item('B')]);
    }

    #[test]
    fn test_generator_day5() {
        let stacks = generator_day5(INPUT);
        assert_eq!(
            stacks,
            (
                vec![
                    Stack {
                        items: vec![Item('Z'), Item('N')]
                    },
                    Stack {
                        items: vec![Item('M'), Item('C'), Item('D')]
                    },
                    Stack {
                        items: vec![Item('P')]
                    }
                ],
                vec![
                    Instruction(1, 2, 1),
                    Instruction(3, 1, 3),
                    Instruction(2, 2, 1),
                    Instruction(1, 1, 2)
                ]
            )
        )
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            "move 1 from 2 to 3".parse::<Instruction>().unwrap(),
            Instruction(1, 2, 3)
        );
    }

    #[test]
    fn test_day05_part1() {
        let generated = generator_day5(INPUT);
        assert_eq!(day05_part1(&generated), "CMZ".to_string());
    }

    #[test]
    fn test_day06_part2() {
        let generated = generator_day5(INPUT);
        assert_eq!(day05_part2(&generated), "MCD".to_string());
    }
}
"#;
