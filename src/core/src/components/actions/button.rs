use icondata::Icon as IconId;
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon; // Adjust based on your icon set (e.g., icondata::BsIcon)
use tailwind_fuse::tw_merge;

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

/// A reusable button component that supports icons, disabled state, and custom styles.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::actions::button::BasicButton;
/// use icondata::AiCheckCircleOutlined;
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <BasicButton
///             button_text="Confirm"
///             icon=Some(AiCheckCircleOutlined)
///             icon_before=true
///             onclick=Callback::new(|_| leptos::logging::log!("clicked"))
///         />
///     }
/// }
/// ```
#[component]
pub fn BasicButton(
    /// Label displayed inside the button.
    #[prop(into, optional)]
    button_text: MaybeProp<String>,

    /// **Deprecated**: use `class` instead. Still supported and merged
    /// in alongside `class` for backward compatibility. Will be removed
    /// in a future release - see CHANGELOG.
    #[prop(into, optional)]
    style_ext: MaybeProp<String>,

    /// Extra Tailwind classes for the button element. Merged with the
    /// component's default classes using conflict-aware merging — a
    /// utility you pass here will override the equivalent default
    /// (e.g. passing `bg-red-500` replaces the default `bg-*`), not
    /// just append.
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// **Deprecated**: use `content_class` instead.
    #[prop(into, optional)]
    children_style_ext: MaybeProp<String>,

    /// Additional Tailwind classes appended to the inner content span.
    #[prop(into, optional)]
    content_class: MaybeProp<String>,

    /// Click event callback. Defaults to a no-op.
    #[prop(default = Callback::new(|_| {}))]
    onclick: Callback<ev::MouseEvent>,

    /// Optional icon rendered alongside the text.
    #[prop(default = None)]
    icon: Option<IconId>,

    /// Accepts a `bool`, `Signal<bool>`, or `RwSignal<bool>`. Defaults to `false`.
    #[prop(into, default = MaybeProp::derive(move || Some(false)))]
    disabled: MaybeProp<bool>,

    /// One of `ButtonType::Button`, `ButtonType::Submit`, or `ButtonType::Reset`. Defaults to `ButtonType::Button`.
    #[prop(into, default = ButtonType::Button)]
    button_type: ButtonType,

    /// If `true`, the icon is placed before the text. Defaults to `false`.
    #[prop(default = false)]
    icon_before: bool,

    /// Optional child nodes. If provided, `button_text` and `icon` are ignored.
    #[prop(optional)]
    children: Option<ChildrenFn>,
    /// `NodeRef<Button>` for direct DOM access.
    #[prop(optional)]
    button_node_ref: NodeRef<Button>,
) -> impl IntoView {
    let button_text_styles = button_text.clone();
    let button_content_styles = move || {
        if button_text_styles.get().is_none() {
            ""
        } else if icon_before {
            "gap-2"
        } else {
            "gap-2 flex-row-reverse"
        }
    };

    // Merge order: base defaults -> deprecated prop -> new prop.
    // `class` is listed last so it wins conflicts against `style_ext`
    // when both are supplied during a migration.
    #[allow(deprecated)]
    let button_class = move || {
        tw_merge!(
            "font-bold py-2 px-4 cursor-pointer rounded-[5px] disabled:opacity-50 disabled:cursor-not-allowed",
            style_ext.get().unwrap_or_default(),
            class.get().unwrap_or_default(),
        )
    };

    #[allow(deprecated)]
    let content_class_merged = move || {
        tw_merge!(
            "flex flex-row items-center justify-center",
            button_content_styles(),
            children_style_ext.get().unwrap_or_default(),
            content_class.get().unwrap_or_default(),
        )
    };

    view! {
        <button
            type={
                match button_type {
                    ButtonType::Button => "button",
                    ButtonType::Submit => "submit",
                    ButtonType::Reset => "reset"
                }
            }
            class=button_class
            on:click=move |ev| onclick.run(ev)
            disabled={disabled}
            node_ref=button_node_ref
        >
            {
                if let Some(children) = &children {
                    Some(children())
                } else {
                    None
                }
            }
            {
                if children.is_none() {
                    Some(view! {
                        <span class=content_class_merged>
                            {move || match icon {
                                Some(button_icon) => view! {
                                    <Icon width="1em" height="1em" icon=button_icon />
                                }.into(),
                                None => None,
                            }}
                            <span>{button_text}</span>
                        </span>
                    })
                } else {
                    None
                }
            }
        </button>
    }
    .into_any()
}

/// A group of buttons rendered inline with shared styles and automatic border-radius on the ends.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use detaxine_ui::components::actions::button::{ButtonGroup, BasicButton};
/// use icondata::{AiCheckCircleOutlined, BsXCircle};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <ButtonGroup class="bg-primary text-white hover:bg-secondary">
///             <BasicButton button_text="First" icon=Some(AiCheckCircleOutlined) icon_before=true />
///             <BasicButton button_text="Second" icon=Some(BsXCircle) icon_before=false />
///             <BasicButton button_text="Third" disabled=true />
///         </ButtonGroup>
///     }
/// }
/// ```
#[component]
pub fn ButtonGroup(
    /// **Deprecated**: use `class` instead. Still supported and merged
    /// in alongside `class` for backward compatibility. `N/B:` All
    /// buttons share the same styles (these affect all buttons).
    #[prop(into, optional)]
    style_ext: MaybeProp<String>,

    /// Extra Tailwind classes applied to every button in the group.
    /// Merged with conflict-aware precedence - an override here wins
    /// over the default for the same property (e.g. passing a `border-*`
    /// color replaces the default border color rather than stacking).
    #[prop(into, optional)]
    class: MaybeProp<String>,

    /// Two or more `BasicButton` components.
    mut children: ChildrenFragmentMut,
) -> impl IntoView {
    let style_ext = style_ext.get().unwrap_or_default();
    let class = class.get().unwrap_or_default();

    view! {
        <div class="flex" role="group">
            {
                let nodes = children().nodes.into_iter().collect::<Vec<_>>();
                let children_len = nodes.len();

                nodes
                .into_iter()
                .enumerate()
                .map(|(index, child)| {
                    let style_ext = style_ext.clone();
                    let class = class.clone();

                    let class_name = move || {
                        let mut merged = tw_merge!(
                            "font-bold py-2 px-4 border border-light-gray border-l-0 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed",
                            style_ext.clone(),
                            class.clone()
                        );
                        if index == 0 {
                            merged.push_str(" rounded-l-[5px]");
                        }
                        if index == children_len - 1 {
                            merged.push_str(" rounded-r-[5px]");
                        }
                        merged
                    };
                    view! {
                        {child.attr("class", class_name())}
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ButtonType

    #[test]
    fn button_type_default_is_button() {
        assert!(matches!(ButtonType::default(), ButtonType::Button));
    }

    #[test]
    fn button_type_clone() {
        assert!(matches!(ButtonType::Submit.clone(), ButtonType::Submit));
    }

    // Icon/text layout logic

    fn content_styles(has_text: bool, icon_before: bool) -> &'static str {
        if !has_text {
            ""
        } else if icon_before {
            "gap-2"
        } else {
            "gap-2 flex-row-reverse"
        }
    }

    #[test]
    fn no_text_produces_no_layout_class() {
        assert_eq!(content_styles(false, true), "");
        assert_eq!(content_styles(false, false), "");
    }

    #[test]
    fn icon_before_true_places_icon_first() {
        assert!(!content_styles(true, true).contains("flex-row-reverse"));
    }

    #[test]
    fn icon_before_false_reverses_order() {
        assert!(content_styles(true, false).contains("flex-row-reverse"));
    }

    // ButtonGroup border-radius logic

    fn is_first(index: usize) -> bool {
        index == 0
    }
    fn is_last(index: usize, len: usize) -> bool {
        index == len - 1
    }

    #[test]
    fn single_button_is_both_first_and_last() {
        assert!(is_first(0));
        assert!(is_last(0, 1));
    }

    #[test]
    fn first_button_is_not_last_in_group() {
        assert!(is_first(0));
        assert!(!is_last(0, 3));
    }

    #[test]
    fn middle_button_is_neither_first_nor_last() {
        assert!(!is_first(1));
        assert!(!is_last(1, 3));
    }

    #[test]
    fn last_button_is_not_first_in_group() {
        assert!(!is_first(2));
        assert!(is_last(2, 3));
    }

    // Disabled tests
    #[test]
    fn disabled_defaults_to_false() {
        let disabled: MaybeProp<bool> = MaybeProp::derive(move || Some(false));
        assert_eq!(disabled.get(), Some(false));
    }

    #[test]
    fn disabled_can_be_set_to_true() {
        let disabled: MaybeProp<bool> = MaybeProp::from(true);
        assert_eq!(disabled.get(), Some(true));
    }

    #[test]
    fn disabled_accepts_signal() {
        let sig = RwSignal::new(false);
        let disabled: MaybeProp<bool> = MaybeProp::derive(move || Some(sig.get()));
        assert_eq!(disabled.get(), Some(false));
        sig.set(true);
        assert_eq!(disabled.get(), Some(true));
    }
}
