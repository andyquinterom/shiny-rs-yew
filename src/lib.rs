use yew::prelude::*;

#[macro_export]
macro_rules! NS {
    ($x:expr) => {
        |y: &str| {
            format!("{}-{}", $x, y)
        }
    };
}

#[derive(Clone, Properties, PartialEq)]
pub struct ActionButtonProps {
    pub id: String,
    pub label: String,
}

#[function_component(ActionButton)]
pub fn action_button(ActionButtonProps { id, label }: &ActionButtonProps) -> Html {
    html! {
        <button id={id.clone()} type="button" class="btn btn-default action-button">{label.clone()}</button>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct NumericInputProps {
    pub id: String,
    pub label: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[prop_or_default]
    pub style: String
}

#[function_component(NumericInput)]
pub fn numeric_input(NumericInputProps { id, label, value, min, max, step, style }: &NumericInputProps) -> Html {
    html! {
        <div class="form-group shiny-input-container" style={style.clone()}>
            <label class="control-label" id={format!("{}-label", id)}  for={id.clone()}>{label.clone()}</label>
            <input id={id.clone()} type="number" class="form-control" step={step.to_string()} value={value.to_string()} min={min.to_string()} max={max.to_string()} />
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct TextInputProps {
    pub id: String,
    pub label: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub style: String
}

#[function_component(TextInput)]
pub fn text_input(TextInputProps { id, label, value, style }: &TextInputProps) -> Html {
    html! {
        <div class="form-group shiny-input-container" style={style.clone()}>
            <label class="control-label" id={format!("{}-label", id)}  for={id.clone()}>{label.clone()}</label>
            <input id={id.clone()} type="text" class="form-control" value={value.clone()}/>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct OutputProps {
    pub id: String,
    #[prop_or_default]
    pub style: String,
}

#[function_component(HtmlOutput)]
pub fn html_output(OutputProps { id, style }: &OutputProps) -> Html {
    html! {
        <div id={id.clone()} class="shiny-html-output" style={style.clone()}></div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct NavProps {
    pub title: String,
    pub id: String,
    #[prop_or(false)]
    pub active: bool,
    #[prop_or_default]
    pub children: Children
}

pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = NavProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = match ctx.props().active {
            true => "tab-pane  active show",
            false => "tab-pane"
        };
        html! {
            <>
                <div class={class} data-value="Hola" id={ctx.props().id.clone()}>
                    { ctx.props().children.clone() }
                </div>
            </>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct PageNavbarProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<Nav>
}

pub struct PageNavbar;

impl Component for PageNavbar {
    type Message = ();
    type Properties = PageNavbarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navbar_titles = ctx.props().children.iter().map(|child| {
            let id = format!("#{}", child.props.id.clone());
            let title = child.props.title.clone();
            let class = if child.props.active { "nav-link active show" } else { "nav-link" };
            html! {
                <li class="nav-item">
                    <a class={class} href={id} data-bs-toggle="tab">{title.clone()}</a>
                </li>
            }
        }).collect::<Vec<Html>>();
        html! {
            <>
            <nav class="navbar navbar-expand-lg bg-light" role="navigation">
                <div class="container-fluid">
                    <div class="navbar-header">
                        <span class="navbar-brand">{ctx.props().title.clone()}</span>
                        <button type="button" class="navbar-toggler" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                    </div>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="nav navbar-nav me-auto mb-2 mb-lg-0" data-tabsetid="1234">{navbar_titles}</ul>
                    </div>
                </div>
            </nav>
            <div class="container-fluid">
                <div class="tab-content" id="1234">
                    { ctx.props().children.clone() }
                </div>
            </div>
            </>
        }
    }
}
