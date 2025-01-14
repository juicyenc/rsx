use std::{
    collections::HashMap,
    fmt::Write,
    fmt::{Debug, Display},
};

use crate::{Children, InnerChildren, Tag};

#[macro_export]
macro_rules! prop {
    ($($($name:tt)-+ $(= $value:expr)?),+) => {{ let mut props: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        $(
            let name = vec![$(stringify!($name)),*];
            let key = name.join("-");
            if key.starts_with("on") {
                panic!("event handler {} should be registered with env! macro", key);
            } else if key == "style" {
                panic!("style property should be set with style! macro");
            }
            let mut no_value = true;
            $(
                no_value = false;
                props.insert(key.to_string(), $value.to_string());
            )?
            if no_value {
                props.insert(key.to_string(), String::new());
            }

        )*
        $crate::tags::TagProp(props)
    }};
    () => {{
        $crate::tags::TagProp::default()
    }}
}

/// and event listener
#[macro_export]
macro_rules! on {
    ($($name:ident = $cb:ident),+) => {
      {
          let mut handlers: std::collections::HashMap<String, String> = std::collections::HashMap::new();
          $(
              let key = stringify!($name);
              let val = stringify!($cb);
              handlers.insert(key.to_string(), val.to_string());

          )*
          $crate::tags::TagHandler(props)
      }
    };
    () => {{
        $crate::tags::TagHandler::default()
    }}
}

/// simple wrapper of tag props
#[derive(Debug, Clone, Default)]
pub struct TagProp(pub HashMap<String, String>);

/// simple wrapper of tag style
#[derive(Debug, Clone, Default)]
pub struct TagStyle(pub HashMap<String, String>);

/// simple wrapper of tag event handler
#[derive(Debug, Clone, Default)]
pub struct TagHandler(pub HashMap<String, String>);

pub struct UnitTag {
    pub tag: &'static str,
    pub props: TagProp,
    pub on: TagHandler,
    pub style: TagStyle,
    pub children: InnerChildren,
}

impl Tag for UnitTag {
    fn name(&self) -> &'static str {
        self.tag
    }

    fn format(&self, f: &mut TagFormatter, buf: &mut String) -> std::fmt::Result {
        let pad = f.pad_size();
        write!(buf, "{:pad$}<{}", "", self.tag)?;
        if f.newline_on_prop {
            buf.push_str(f.line_sep);
            let pad = pad + 1;
            write!(buf, "{:pad$}", "")?;
            for (name, val) in self.props.0.iter() {
                if val.is_empty() {
                    write!(buf, r#"{:pad$}{}"#, "", name)?;
                } else {
                    write!(buf, r#"{:pad$}{}="{}""#, "", name, val)?;
                }
                buf.push_str(f.line_sep);
            }
            if !self.style.0.is_empty() {
                write!(buf, "{:pad$}", "")?;
                write!(buf, "style=\"")?;
                for (name, val) in self.style.0.iter() {
                    write!(buf, "{}: {}; ", name, val)?;
                }
                write!(buf, "\"")?;
            }
            for (name, val) in self.on.0.iter() {
                write!(buf, "{:pad$}", "")?;
                write!(buf, r#"on{}="{}""#, name, val)?;
                buf.push_str(f.line_sep);
            }
            let pad = pad - 1;
            write!(buf, "{:pad$}>", "")?;
            buf.push_str(f.line_sep);
        } else {
            for (name, val) in self.props.0.iter() {
                if val.is_empty() {
                    write!(buf, r#" {}"#, name)?;
                } else {
                    write!(buf, r#" {}="{}""#, name, val)?;
                }
            }
            if !self.style.0.is_empty() {
                write!(buf, " style=\"")?;
                for (name, val) in self.style.0.iter() {
                    write!(buf, "{}: {}; ", name, val)?;
                }
                write!(buf, "\"")?;
            }
            for (name, val) in self.on.0.iter() {
                write!(buf, r#" on{}="{}""#, name, val)?;
            }
            buf.push('>');
        }
        buf.push_str(f.line_sep);
        f.indent += 1;
        for child in self.children.iter() {
            child.format(f, buf)?
        }
        f.indent -= 1;
        write!(buf, "{:pad$}</{}>", "", self.tag)?;
        buf.push_str(f.line_sep);
        Ok(())
    }
}

impl Display for UnitTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut content = String::new();
        let mut formatter = TagFormatter::default();
        self.format(&mut formatter, &mut content)?;
        write!(f, "{}", content)
    }
}

impl<C: Into<Children>> From<(&'static str, C)> for UnitTag {
    fn from(src: (&'static str, C)) -> Self {
        Self {
            tag: src.0,
            children: src.1.into().0,
            props: Default::default(),
            on: Default::default(),
            style: Default::default(),
        }
    }
}

impl<C: Into<Children>> From<(&'static str, TagProp, C)> for UnitTag {
    fn from(src: (&'static str, TagProp, C)) -> Self {
        Self {
            tag: src.0,
            children: src.2.into().0,
            props: src.1,
            on: Default::default(),
            style: Default::default(),
        }
    }
}

impl<C: Into<Children>> From<(&'static str, TagHandler, C)> for UnitTag {
    fn from(src: (&'static str, TagHandler, C)) -> Self {
        Self {
            tag: src.0,
            children: src.2.into().0,
            props: Default::default(),
            on: src.1,
            style: Default::default(),
        }
    }
}

impl<C: Into<Children>> From<(&'static str, TagStyle, C)> for UnitTag {
    fn from(src: (&'static str, TagStyle, C)) -> Self {
        Self {
            tag: src.0,
            children: src.2.into().0,
            props: Default::default(),
            on: Default::default(),
            style: src.1,
        }
    }
}

impl<C: Into<Children>> From<(&'static str, TagProp, TagHandler, C)> for UnitTag {
    fn from(src: (&'static str, TagProp, TagHandler, C)) -> Self {
        Self {
            tag: src.0,
            children: src.3.into().0,
            style: Default::default(),
            props: src.1,
            on: src.2,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagProp, TagStyle, C)> for UnitTag {
    fn from(src: (&'static str, TagProp, TagStyle, C)) -> Self {
        Self {
            tag: src.0,
            children: src.3.into().0,
            on: Default::default(),
            props: src.1,
            style: src.2,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagHandler, TagProp, C)> for UnitTag {
    fn from(src: (&'static str, TagHandler, TagProp, C)) -> Self {
        Self {
            tag: src.0,
            children: src.3.into().0,
            style: Default::default(),
            on: src.1,
            props: src.2,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagHandler, TagStyle, C)> for UnitTag {
    fn from(src: (&'static str, TagHandler, TagStyle, C)) -> Self {
        Self {
            tag: src.0,
            children: src.3.into().0,
            props: Default::default(),
            on: src.1,
            style: src.2,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagStyle, TagProp, C)> for UnitTag {
    fn from(src: (&'static str, TagStyle, TagProp, C)) -> Self {
        Self {
            tag: src.0,
            children: src.3.into().0,
            on: Default::default(),
            style: src.1,
            props: src.2,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagStyle, TagHandler, C)> for UnitTag {
    fn from(src: (&'static str, TagStyle, TagHandler, C)) -> Self {
        Self {
            tag: src.0,
            children: src.3.into().0,
            props: Default::default(),
            style: src.1,
            on: src.2,
        }
    }
}

impl<C: Into<Children>> From<(&'static str, TagProp, TagHandler, TagStyle, C)> for UnitTag {
    fn from(src: (&'static str, TagProp, TagHandler, TagStyle, C)) -> Self {
        Self {
            tag: src.0,
            children: src.4.into().0,
            props: src.1,
            on: src.2,
            style: src.3,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagProp, TagStyle, TagHandler, C)> for UnitTag {
    fn from(src: (&'static str, TagProp, TagStyle, TagHandler, C)) -> Self {
        Self {
            tag: src.0,
            children: src.4.into().0,
            props: src.1,
            style: src.2,
            on: src.3,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagHandler, TagProp, TagStyle, C)> for UnitTag {
    fn from(src: (&'static str, TagHandler, TagProp, TagStyle, C)) -> Self {
        Self {
            tag: src.0,
            children: src.4.into().0,
            on: src.1,
            props: src.2,
            style: src.3,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagHandler, TagStyle, TagProp, C)> for UnitTag {
    fn from(src: (&'static str, TagHandler, TagStyle, TagProp, C)) -> Self {
        Self {
            tag: src.0,
            children: src.4.into().0,
            on: src.1,
            style: src.2,
            props: src.3,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagStyle, TagProp, TagHandler, C)> for UnitTag {
    fn from(src: (&'static str, TagStyle, TagProp, TagHandler, C)) -> Self {
        Self {
            tag: src.0,
            children: src.4.into().0,
            style: src.1,
            props: src.2,
            on: src.3,
        }
    }
}
impl<C: Into<Children>> From<(&'static str, TagStyle, TagHandler, TagProp, C)> for UnitTag {
    fn from(src: (&'static str, TagStyle, TagHandler, TagProp, C)) -> Self {
        Self {
            tag: src.0,
            children: src.4.into().0,
            style: src.1,
            on: src.2,
            props: src.3,
        }
    }
}

impl UnitTag {
    /// set tag properties
    pub fn props(mut self, props: TagProp) -> Self {
        self.props = props;
        self
    }

    /// set tag styles
    pub fn style(mut self, style: TagStyle) -> Self {
        self.style = style;
        self
    }

    /// set tag handlers
    pub fn on(mut self, handlers: TagHandler) -> Self {
        self.on = handlers;
        self
    }
}

pub fn x<T: Into<UnitTag>>(tag: T) -> UnitTag {
    tag.into()
}

#[derive(Debug, Clone)]
pub struct TagFormatter {
    pub tab_size: usize,
    pub indent: usize,
    pub max_width: usize,
    pub newline_on_prop: bool,
    pub line_sep: &'static str,
}

impl Default for TagFormatter {
    fn default() -> Self {
        Self {
            tab_size: 4,
            indent: 0,
            max_width: 120,
            newline_on_prop: false,
            line_sep: "\n",
        }
    }
}

impl TagFormatter {
    pub fn pad_size(&self) -> usize {
        self.indent * self.tab_size
    }
}

/// a helper macro to define custom html tag construct function, struct and arguments structs
///
/// ## example
///
/// ```no_run
/// tag!(app, App, AppArgs, "my custom tag");
///
/// let app = app(h1("great"));
/// ```
///
#[macro_export]
macro_rules! tag {
    ($func_name:ident, $struct:ident, $arg:ident, $($doc:literal),+) => {
        $(#[doc=$doc])+
        pub struct $struct (UnitTag);

        impl $crate::Tag for $struct {
            fn name(&self) -> &'static str {
                self.0.name()
            }

            fn format(&self, f: &mut $crate::TagFormatter, buf: &mut String) -> std::fmt::Result {
                self.0.format(f, buf)
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        // impl <C: Into<$crate::Children>> From<C> for $struct {
        //     fn from(src: C) -> Self {
        //         $struct ($crate::tags::UnitTag {
        //             tag: stringify!($func_name),
        //             props: Default::default(),
        //             on: Default::default(),
        //             style: Default::default(),
        //             children: src.into().0,
        //         })
        //     }
        // }

        // impl<TA: $crate::Tag + 'static> From<TA> for $struct {
        //     fn from(src: TA) -> Self {
        //         let children: $crate::InnerChildren = vec![Box::new(src)];
        //         Self($crate::tags::UnitTag {
        //             tag: stringify!($func_name),
        //             children: children.0,
        //             props: Default::default(),
        //             on: Default::default(),
        //             style: Default::default()
        //         })
        //     }
        // }

        // impl From<$struct> for $crate::Children {
        //     fn from(src: $struct) -> Self {
        //         $crate::Children(vec![Box ::new(src)])
        //     }
        // }

        impl<TA: $crate::Tag + 'static, TB: $crate::Tag + 'static> From<(TA, TB)> for $struct {
            fn from(src: (TA, TB)) -> Self {
                let children: $crate::Children = src.into();
                Self($crate::tags::UnitTag {
                    tag: stringify!($func_name),
                    children: children.0,
                    props: Default::default(),
                    on: Default::default(),
                    style: Default::default()
                })
            }
        }

        pub struct $arg {
            pub children: $crate::InnerChildren,
            pub props: $crate::tags::TagProp,
            pub on: $crate::tags::TagHandler,
            pub style: $crate::tags::TagStyle,
        }

        impl<C: Into<Children>> From<C> for $arg {
            fn from(src:  C) -> Self {
                Self {
                    children: src.into().0,
                    props: Default::default(),
                    on: Default::default(),
                    style: Default::default(),
                }
            }
        }

        impl<C: Into<Children>> From<(TagProp, C)> for $arg {
            fn from(src: ( TagProp, C)) -> Self {
                Self {
                    children: src.1.into().0,
                    props: src.0,
                    on: Default::default(),
                    style: Default::default(),
                }
            }
        }

        impl<C: Into<Children>> From<(TagHandler, C)> for $arg {
            fn from(src: ( TagHandler, C)) -> Self {
                Self {
                    children: src.1.into().0,
                    props: Default::default(),
                    on: src.0,
                    style: Default::default(),
                }
            }
        }

        impl<C: Into<Children>> From<(TagStyle, C)> for $arg {
            fn from(src: ( TagStyle, C)) -> Self {
                Self {
                    children: src.1.into().0,
                    props: Default::default(),
                    on: Default::default(),
                    style: src.0,
                }
            }
        }

        impl<C: Into<Children>> From<(TagProp, TagHandler, C)> for $arg {
            fn from(src: ( TagProp, TagHandler, C)) -> Self {
                Self {
                    children: src.2.into().0,
                    style: Default::default(),
                    props: src.0,
                    on: src.1,
                }
            }
        }
        impl<C: Into<Children>> From<(TagProp, TagStyle, C)> for $arg {
            fn from(src: ( TagProp, TagStyle, C)) -> Self {
                Self {
                    children: src.2.into().0,
                    on: Default::default(),
                    props: src.0,
                    style: src.1,
                }
            }
        }
        impl<C: Into<Children>> From<(TagHandler, TagProp, C)> for $arg {
            fn from(src: ( TagHandler, TagProp, C)) -> Self {
                Self {
                    children: src.2.into().0,
                    style: Default::default(),
                    on: src.0,
                    props: src.1,
                }
            }
        }
        impl<C: Into<Children>> From<(TagHandler, TagStyle, C)> for $arg {
            fn from(src: ( TagHandler, TagStyle, C)) -> Self {
                Self {
                    children: src.2.into().0,
                    props: Default::default(),
                    on: src.0,
                    style: src.1,
                }
            }
        }
        impl<C: Into<Children>> From<(TagStyle, TagProp, C)> for $arg {
            fn from(src: ( TagStyle, TagProp, C)) -> Self {
                Self {
                    children: src.2.into().0,
                    on: Default::default(),
                    style: src.0,
                    props: src.1,
                }
            }
        }
        impl<C: Into<Children>> From<(TagStyle, TagHandler, C)> for $arg {
            fn from(src: ( TagStyle, TagHandler, C)) -> Self {
                Self {
                    children: src.2.into().0,
                    props: Default::default(),
                    style: src.0,
                    on: src.1,
                }
            }
        }

        impl<C: Into<Children>> From<(TagProp, TagHandler, TagStyle, C)> for $arg {
            fn from(src: ( TagProp, TagHandler, TagStyle, C)) -> Self {
                Self {
                    children: src.3.into().0,
                    props: src.0,
                    on: src.1,
                    style: src.2,
                }
            }
        }
        impl<C: Into<Children>> From<(TagProp, TagStyle, TagHandler, C)> for $arg {
            fn from(src: ( TagProp, TagStyle, TagHandler, C)) -> Self {
                Self {
                    children: src.3.into().0,
                    props: src.0,
                    style: src.1,
                    on: src.2,
                }
            }
        }
        impl<C: Into<Children>> From<(TagHandler, TagProp, TagStyle, C)> for $arg {
            fn from(src: ( TagHandler, TagProp, TagStyle, C)) -> Self {
                Self {
                    children: src.3.into().0,
                    on: src.0,
                    props: src.1,
                    style: src.2,
                }
            }
        }
        impl<C: Into<Children>> From<(TagHandler, TagStyle, TagProp, C)> for $arg {
            fn from(src: ( TagHandler, TagStyle, TagProp, C)) -> Self {
                Self {
                    children: src.3.into().0,
                    on: src.0,
                    style: src.1,
                    props: src.2,
                }
            }
        }
        impl<C: Into<Children>> From<(TagStyle, TagProp, TagHandler, C)> for $arg {
            fn from(src: ( TagStyle, TagProp, TagHandler, C)) -> Self {
                Self {
                    children: src.3.into().0,
                    style: src.0,
                    props: src.1,
                    on: src.2,
                }
            }
        }
        impl<C: Into<Children>> From<(TagStyle, TagHandler, TagProp, C)> for $arg {
            fn from(src: ( TagStyle, TagHandler, TagProp, C)) -> Self {
                Self {
                    children: src.3.into().0,
                    style: src.0,
                    on: src.1,
                    props: src.2,
                }
            }
        }

        ////////////////// helper sep

    impl From<TagProp> for $arg {
        fn from(src: TagProp) -> Self {
            Self {
                children: vec![],
                props: src,
                on: Default::default(),
                style: Default::default(),
            }
        }
    }

    impl From<TagHandler> for $arg {
        fn from(src: TagHandler) -> Self {
            Self {
                children: vec![],
                props: Default::default(),
                on: src,
                style: Default::default(),
            }
        }
    }

    impl From<TagStyle> for $arg {
        fn from(src:  TagStyle) -> Self {
            Self {
                children: vec![],
                props: Default::default(),
                on: Default::default(),
                style: src,
            }
        }
    }

    impl From<(TagProp, TagHandler)> for $arg {
        fn from(src: ( TagProp, TagHandler)) -> Self {
            Self {
                children: vec![],
                style: Default::default(),
                props: src.0,
                on: src.1,
            }
        }
    }
    impl From<(TagProp, TagStyle)> for $arg {
        fn from(src: ( TagProp, TagStyle)) -> Self {
            Self {
                children: vec![],
                on: Default::default(),
                props: src.0,
                style: src.1,
            }
        }
    }
    impl From<(TagHandler, TagProp)> for $arg {
        fn from(src: ( TagHandler, TagProp)) -> Self {
            Self {
                children: vec![],
                style: Default::default(),
                on: src.0,
                props: src.1,
            }
        }
    }
    impl From<(TagHandler, TagStyle)> for $arg {
        fn from(src: ( TagHandler, TagStyle)) -> Self {
            Self {
                children: vec![],
                props: Default::default(),
                on: src.0,
                style: src.1,
            }
        }
    }
    impl From<(TagStyle, TagProp)> for $arg {
        fn from(src: ( TagStyle, TagProp)) -> Self {
            Self {
                children: vec![],
                on: Default::default(),
                style: src.0,
                props: src.1,
            }
        }
    }
    impl From<(TagStyle, TagHandler)> for $arg {
        fn from(src: ( TagStyle, TagHandler)) -> Self {
            Self {
                children: vec![],
                props: Default::default(),
                style: src.0,
                on: src.1,
            }
        }
    }

    impl From<(TagProp, TagHandler, TagStyle)> for $arg {
        fn from(src: ( TagProp, TagHandler, TagStyle)) -> Self {
            Self {
                children: vec![],
                props: src.0,
                on: src.1,
                style: src.2,
            }
        }
    }
    impl From<(TagProp, TagStyle, TagHandler)> for $arg {
        fn from(src: ( TagProp, TagStyle, TagHandler)) -> Self {
            Self {
                children: vec![],
                props: src.0,
                style: src.1,
                on: src.2,
            }
        }
    }
    impl From<(TagHandler, TagProp, TagStyle)> for $arg {
        fn from(src: ( TagHandler, TagProp, TagStyle)) -> Self {
            Self {
                children: vec![],
                on: src.0,
                props: src.1,
                style: src.2,
            }
        }
    }
    impl From<(TagHandler, TagStyle, TagProp)> for $arg {
        fn from(src: ( TagHandler, TagStyle, TagProp)) -> Self {
            Self {
                children: vec![],
                on: src.0,
                style: src.1,
                props: src.2,
            }
        }
    }
    impl From<(TagStyle, TagProp, TagHandler)> for $arg {
        fn from(src: ( TagStyle, TagProp, TagHandler)) -> Self {
            Self {
                children: vec![],
                style: src.0,
                props: src.1,
                on: src.2,
            }
        }
    }
    impl From<(TagStyle, TagHandler, TagProp)> for $arg {
        fn from(src: ( TagStyle, TagHandler, TagProp)) -> Self {
            Self {
                children: vec![],
                style: src.0,
                on: src.1,
                props: src.2,
            }
        }
    }


        impl $struct {
            /// set tag properties
            pub fn props(mut self, props: $crate::tags::TagProp) -> Self {
                self.0.props = props;
                self
            }

            /// set tag styles
            pub fn style(mut self, style: $crate::tags::TagStyle) -> Self {
                self.0.style = style;
                self
            }

            /// set tag handlers
            pub fn on(mut self, handlers: $crate::tags::TagHandler) -> Self {
                self.0.on = handlers;
                self
            }
        }

        $(#[doc=$doc])+
        pub fn $func_name<T: Into<$arg>>(tag: T) -> $struct {
            let args: $arg = tag.into();
            let $arg { children, props, on, style } = args;
            $struct($crate::tags::UnitTag {
                tag: stringify!($func_name),
                children,
                props,
                on,
                style
            })
        }


    };
}

tag!(
    a,
    A,
    AArgs,
    r#""#,
    r#"`<a>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/a)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"可见的内容（Transparent），包含流内容（不包括交互式内容）或文字内容（phrasing content）。"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"接受短语内容的任何元素或接受流内容的任何元素，但始终不接受 &lt;a&gt; 元素（根据对称的逻辑原理，如果 &lt;a&gt; 标记作为父元素，不能具有交互内容 ，则相同的 &lt;a&gt; 内容不能具有 &lt;a&gt; 标记作为其父元素）。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLAnchorElement)"#
);
tag!(
    abbr,
    Abbr,
    AbbrArgs,
    r#""#,
    r#"`<abbr>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/abbr)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    address,
    Address,
    AddressArgs,
    r#""#,
    r#"`<address>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/address)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>, 但是不能嵌套<code>&lt;address&gt;元素</code>, 不能是头部内容 (<a href="/zh-CN/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a>, <a href="/en-US/docs/Web/HTML/Element/Heading_Elements" title="Currently only available in English (US)" class="only-in-en-us"><code>&lt;h1&gt;</code> <small>(en-US)</small></a>, <a href="/en-US/docs/Web/HTML/Element/Heading_Elements" class="only-in-en-us" title="Currently only available in English (US)"><code>&lt;h2&gt;</code> <small>(en-US)</small></a>, <a href="/en-US/docs/Web/HTML/Element/Heading_Elements" class="only-in-en-us" title="Currently only available in English (US)"><code>&lt;h3&gt;</code> <small>(en-US)</small></a>, <a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/Heading_Elements"><code>&lt;h4&gt;</code> <small>(en-US)</small></a>, <a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/Heading_Elements"><code>&lt;h5&gt;</code> <small>(en-US)</small></a>, <a href="/en-US/docs/Web/HTML/Element/Heading_Elements" class="only-in-en-us" title="Currently only available in English (US)"><code>&lt;h6&gt;</code> <small>(en-US)</small></a>), 不能是区块内容 (<a href="/zh-CN/docs/Web/HTML/Element/article"><code>&lt;article&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/aside"><code>&lt;aside&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/section"><code>&lt;section&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/nav"><code>&lt;nav&gt;</code></a>), 不能是<a href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a> 或 <a href="/zh-CN/docs/Web/HTML/Element/footer"><code>&lt;footer&gt;</code></a>元素。"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>, but always excluding <code>&lt;address&gt;</code> elements (according to the logical principle of symmetry, if <code>&lt;address&gt;</code> tag, as a parent, can not have nested <code>&lt;address&gt;</code> element, then the same <code>&lt;address&gt;</code> content can not have <code>&lt;address&gt;</code> tag as its parent)."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    area,
    Area,
    AreaArgs,
    r#""#,
    r#"`<area>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/area)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn>它是一个空的元素不允许嵌套任何子元素或者文本。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn>只能允许有开始标签不允许有结束标签。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn> &lt;area&gt;元素必须拥有一个&lt;map&gt;元素祖先元素，但不一定是直接的父元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLAreaElement)"#
);
tag!(
    article,
    Article,
    ArticleArgs,
    r#""#,
    r#"`<article>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/article)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"所有接受 <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>的元素。注意<code>&lt;article&gt;</code>元素不能 成为<a href="/zh-CN/docs/Web/HTML/Element/address"><code>&lt;address&gt;</code></a>元素的子元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    aside,
    Aside,
    AsideArgs,
    r#""#,
    r#"`<aside>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/aside)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="HTML/Content categories#Flow content" href="/zh-CN/docs/Web/Guide/HTML/Content_categories#流式元素（flow_content）">流式元素</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"所有元素接受流式元素。注意&nbsp;<code>&lt;aside&gt;</code>&nbsp;不能是<a href="/zh-CN/docs/Web/HTML/Element/address"><code>&lt;address&gt;</code></a> 元素的后代"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    audio,
    Audio,
    AudioArgs,
    r#""#,
    r#"`<audio>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/audio)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    b,
    B,
    BArgs,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任意可容纳&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    base,
    Base,
    BaseArgs,
    r#"- 允许的内容"#,
    r#"无，它是一个<a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>"#,
    r#"- 忽略结束标签"#,
    r#"该标签不能有结束标签。"#,
    r#"- 允许的父类"#,
    r#"任何不带有任何其他 <a href="/zh-CN/docs/Web/HTML/Element/base" aria-current="page"><code>&lt;base&gt;</code></a> 元素的<a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a> 元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLBaseElement)"#
);
tag!(
    bdi,
    Bdi,
    BdiArgs,
    r#"- 允许的内容"#,
    r#"<a title="HTML/Content_categories#Flow_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a title="HTML/Content_categories#Flow_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    bdo,
    Bdo,
    BdoArgs,
    r#""#,
    r#"`<bdo>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/bdo)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="zh-CN/docs/Web/Guide/HTML/Content_categories">短语元素</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"接受<a href="zh-CN/docs/Web/Guide/HTML/Content_categories">短语元素</a>的任何元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    blockquote,
    Blockquote,
    BlockquoteArgs,
    r#""#,
    r#"`<blockquote>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/blockquote)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)"#
);
tag!(
    body,
    Body,
    BodyArgs,
    r#""#,
    r#"`<body>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/body)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"The start tag may be omitted if the first thing inside it is not a space character, comment, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a> element or <a href="/zh-CN/docs/Web/HTML/Element/style"><code>&lt;style&gt;</code></a> element. The end tag may be omitted if the <code>&lt;body&gt;</code> element has contents or has a start tag, and is not immediately followed by a comment."#,
    r#"- 允许的父类"#,
    r#"它必须是 <a href="/zh-CN/docs/Web/HTML/Element/html">html</a> 元素的直接子元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLBodyElement)"#
);
tag!(
    br,
    Br,
    BrArgs,
    r#""#,
    r#"`<br>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/br)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"无，这是一个<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>."#,
    r#"- 忽略结束标签"#,
    r#"必须有一个开始标签，并且一定不能有结束标签。在 XHTML 中将元素写为&nbsp;<code>&lt;br&nbsp;/&gt;</code>。"#,
    r#"- 允许的父类"#,
    r#"任意可容纳&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLBRElement)"#
);
tag!(
    button,
    Button,
    ButtonArgs,
    r#""#,
    r#"`<button>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/button)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。&nbsp;"#,
    r#"- 允许的父类"#,
    r#"任意可容纳&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLButtonElement)"#
);
tag!(
    canvas,
    Canvas,
    CanvasArgs,
    r#""#,
    r#"`<canvas>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/canvas)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    caption,
    Caption,
    CaptionArgs,
    r#""#,
    r#"`<caption>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/caption)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="HTML/Content categories#Flow content" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"A <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element, as its first descendant."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement)"#
);
tag!(
    cite,
    Cite,
    CiteArgs,
    r#""#,
    r#"`<cite>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/cite)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">叙述内容（Phrasing Content）</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">叙述内容（Phrasing Content）</a>的元素。"#,
    r#"- [dom API]()"#
);
tag!(
    code,
    Code,
    CodeArgs,
    r#""#,
    r#"`<code>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/code)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>Permitted content</dfn> <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" title="HTML/Content_categories#Phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>Tag omission</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>Permitted parent elements</dfn> Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" title="HTML/Content_categories#Phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    col,
    Col,
    ColArgs,
    r#""#,
    r#"`<col>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/col)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>Permitted content</dfn> None, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>Tag omission</dfn> The <span title="syntax-start-tag">start tag</span> is mandatory, but, as it is a void element, the <span title="syntax-end-tag">use of an end tag</span> is forbidden."#,
    r#"- 允许的父类"#,
    r#"<dfn>Permitted parent elements</dfn> <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> only, though it can be implicitly defined as its start tag is not mandatory. The <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> must not have a <a href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-span"><code>span</code></a> attribute."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)"#
);
tag!(
    colgroup,
    Colgroup,
    ColgroupArgs,
    r#""#,
    r#"`<colgroup>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/colgroup)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"If the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-span"><code>span</code></a> attribute is present: none, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>.<br>"#,
    r#"If the attribute is not present: zero or more <a href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> element"#,
    r#"- 忽略结束标签"#,
    r#"如果元素的第一个子元素存在且是一个 <a href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> 元素，而且在它之前没有省略了结束标签的 <a href="/zh-CN/docs/Web/HTML/Element/colgroup" aria-current="page"><code>&lt;colgroup&gt;</code></a> 元素，元素的开始标签可以被省略。<br>"#,
    r#"如果之后没有紧跟一个空格或注释，元素的结束标签可以被省略。"#,
    r#"- 允许的父类"#,
    r#"一个 <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 元素。The <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> must appear after any optional <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a> element but before any <a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a>, <a class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/tbody" title="Currently only available in English (US)"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>, <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)"#
);
tag!(
    content,
    Content,
    ContentArgs,
    r#""#,
    r#"`<content>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/content)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流式内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受流式内容的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLContentElement)"#
);
tag!(
    data,
    Data,
    DataArgs,
    r#""#,
    r#"`<data>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/data)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLDataElement)"#
);
tag!(
    datalist,
    Datalist,
    DatalistArgs,
    r#""#,
    r#"`<datalist>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/datalist)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许内容</dfn>要么&nbsp;<a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/HTML/Content_categories#Phrasing_content">段落内容</a>&nbsp;要么 0 个或多个 <a href="/zh-CN/docs/Web/HTML/Element/option"><code>&lt;option&gt;</code></a>元素。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>遗漏标签</dfn>不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许父级元素</dfn>任何接受<a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/HTML/Content_categories#Phrasing_content">段落内容</a>的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement)"#
);
tag!(
    dd,
    Dd,
    DdArgs,
    r#""#,
    r#"`<dd>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/dd)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="HTML/Content_categories#Flow_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"必须有开标签。如果该元素后面紧跟着另一个 &lt;dd&gt; 元素，或者父元素中没有更多内容，则可以省略闭标签。"#,
    r#"- 允许的父类"#,
    r#"该元素需要出现在 <a href="/zh-CN/docs/Web/HTML/Element/dt"><code>&lt;dt&gt;</code></a> 元素和 <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/dd"><code>&lt;dd&gt;</code></a> 元素之后，并且在一个 <a href="/zh-CN/docs/Web/HTML/Element/dl"><code>&lt;dl&gt;</code></a> 元素里。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    del,
    Del,
    DelArgs,
    r#""#,
    r#"`<del>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/del)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#透明内容模型（transparent_content_model）">透明内容模型</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任意<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#短语元素（phrasing_content）">短语元素</a>&nbsp;"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement)"#
);
tag!(
    details,
    Details,
    DetailsArgs,
    r#""#,
    r#"`<details>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/details)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"One <a href="/zh-CN/docs/Web/HTML/Element/summary"><code>&lt;summary&gt;</code></a> element followed by <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLDetailsElement)"#
);
tag!(
    dfn,
    Dfn,
    DfnArgs,
    r#""#,
    r#"`<dfn>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/dfn)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#phrasing_content" title="HTML/Content_categories#Phrasing_content">Phrasing content</a>, but no <a href="/zh-CN/docs/Web/HTML/Element/dfn" aria-current="page"><code>&lt;dfn&gt;</code></a> element must be a descendant."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#phrasing_content" title="HTML/Content_categories#Phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    dialog,
    Dialog,
    DialogArgs,
    r#""#,
    r#"`<dialog>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/dialog)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#流式元素（flow_content）">流式元素</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#流式元素（flow_content）">流式元素</a>的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLDialogElement)"#
);
tag!(
    div,
    Div,
    DivArgs,
    r#""#,
    r#"`<div>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/div)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>.<br>"#,
    r#"Or (in <a href="/zh-CN/docs/Glossary/WHATWG">WHATWG</a> HTML): If the parent is a <a href="/zh-CN/docs/Web/HTML/Element/dl"><code>&lt;dl&gt;</code></a> element: one or more <a href="/zh-CN/docs/Web/HTML/Element/dt"><code>&lt;dt&gt;</code></a> elements followed by one or more <a href="/zh-CN/docs/Web/HTML/Element/dd"><code>&lt;dd&gt;</code></a> elements, optionally intermixed with <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a> elements."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>.<br>"#,
    r#"Or (in <a href="/zh-CN/docs/Glossary/WHATWG">WHATWG</a> HTML): <a href="/zh-CN/docs/Web/HTML/Element/dl"><code>&lt;dl&gt;</code></a> element."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLDivElement)"#
);
tag!(
    dl,
    Dl,
    DlArgs,
    r#""#,
    r#"`<dl>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/dl)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>标签省略</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>允许的父元素</dfn>符合流内容的任何元素"#,
    r#"- 允许的父类"#,
    r#"<dfn>DOM 接口</dfn> <a href="/en-US/docs/Web/API/HTMLDListElement" title="Currently only available in English (US)" class="only-in-en-us"><code>HTMLDListElement</code> <small>(en-US)</small></a>"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement)"#
);
tag!(
    dt,
    Dt,
    DtArgs,
    r#""#,
    r#"`<dt>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/dt)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn><a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#flowing_content"> 流内容</a>，但是不能包含&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a> 元素、<a href="/zh-CN/docs/Web/HTML/Element/footer"><code>&lt;footer&gt;</code></a> 元素或者其他章节、标题内容。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn>必须有开标签。如果该元素后面紧跟着另一个 &lt;dd&gt; 元素，或者父元素中没有更多内容，则可以省略闭标签。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn>该元素需要出现在&nbsp;<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/dt"><code>&lt;dt&gt;</code></a> 元素或者 <a href="/zh-CN/docs/Web/HTML/Element/dd"><code>&lt;dd&gt;</code></a> 元素之前，并且在&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/dl"><code>&lt;dl&gt;</code></a> 元素中。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    em,
    Em,
    EmArgs,
    r#""#,
    r#"`<em>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/em)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>Permitted content</dfn> <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>Tag omission</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>Permitted parent elements</dfn> Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(embed, Embed, EmbedArgs, "");
tag!(
    fieldset,
    Fieldset,
    FieldsetArgs,
    r#""#,
    r#"`<fieldset>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/fieldset)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"可选的<a href="/zh-CN/docs/Web/HTML/Element/legend"><code>&lt;legend&gt;</code></a> 元素，后面是内容流（flow content）"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts&nbsp;<a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content (en-US)</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLFieldSetElement)"#
);
tag!(
    figcaption,
    Figcaption,
    FigcaptionArgs,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" title="HTML/Content categories#Flow content">流式内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/figure"><code>&lt;figure&gt;</code></a> 元素；<code>&lt;figcaption&gt;</code> 元素必须是它的第一个或者最 后一个子节点。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    figure,
    Figure,
    FigureArgs,
    r#""#,
    r#"`<figure>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/figure)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"A <a href="/zh-CN/docs/Web/HTML/Element/figcaption"><code>&lt;figcaption&gt;</code></a> element, followed by <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; or flow content followed by a <a href="/zh-CN/docs/Web/HTML/Element/figcaption"><code>&lt;figcaption&gt;</code></a> element; or flow content."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"所有接受&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>&nbsp;的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    footer,
    Footer,
    FooterArgs,
    r#""#,
    r#"`<footer>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/footer)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn> <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" title="Currently only available in English (US)" class="only-in-en-us">流内容 (en-US)</a>，但是不能包含&lt;footer&gt;或者<a href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a>。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn>任何接收<a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流内容 (en-US)</a>的元素。注意&lt;footer&gt;元素必须不能是&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/address"><code>&lt;address&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a> 或者另一个<code>&lt;footer&gt;</code> 元素的后代 元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    form,
    Form,
    FormArgs,
    r#""#,
    r#"`<form>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/form)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>, but not containing <code>&lt;form&gt;</code> elements"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"可以是 HTML 的<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">任何标签</a>"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLFormElement)"#
);
tag!(
    h1,
    H1,
    H1Args,
    r#""#,
    r#"`<h1>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/h1)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"None, both the starting and ending tag are mandatory."#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Any element that accepts"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; don't use a heading element as a child of the"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a> element — it is now deprecated."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"#
);
tag!(
    h2,
    H2,
    H2Args,
    r#""#,
    r#"`<h2>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/h2)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"None, both the starting and ending tag are mandatory."#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Any element that accepts"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; don't use a heading element as a child of the"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a> element — it is now deprecated."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"#
);
tag!(
    h3,
    H3,
    H3Args,
    r#""#,
    r#"`<h3>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/h3)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"None, both the starting and ending tag are mandatory."#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Any element that accepts"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; don't use a heading element as a child of the"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a> element — it is now deprecated."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"#
);
tag!(
    h4,
    H4,
    H4Args,
    r#""#,
    r#"`<h4>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/h4)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"None, both the starting and ending tag are mandatory."#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Any element that accepts"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; don't use a heading element as a child of the"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a> element — it is now deprecated."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"#
);
tag!(
    h5,
    H5,
    H5Args,
    r#""#,
    r#"`<h5>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/h5)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"None, both the starting and ending tag are mandatory."#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Any element that accepts"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; don't use a heading element as a child of the"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a> element — it is now deprecated."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"#
);
tag!(
    h6,
    H6,
    H6Args,
    r#""#,
    r#"`<h6>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/h6)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"None, both the starting and ending tag are mandatory."#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Any element that accepts"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>; don't use a heading element as a child of the"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/hgroup"><code>&lt;hgroup&gt;</code></a> element — it is now deprecated."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)"#
);
tag!(
    head,
    Head,
    HeadArgs,
    r#""#,
    r#"`<head>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/head)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许内容</dfn>至少包含一个<a href="/zh-CN/docs/Web/HTML/Element/title"><code>&lt;title&gt;</code></a> 元素来指定文档的标题信息，除非标题已经从更高等级协议中指定（<a href="/zh-CN/docs/Web/HTML/Element/iframe"><code>&lt;iframe&gt;</code></a> ）。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>允许父元素</dfn><a href="/zh-CN/docs/Web/HTML/Element/html"><code>&lt;html&gt;</code></a> 元素"#,
    r#"- 允许的父类"#,
    r#"<dfn>DOM 接口</dfn> <a href="/zh-CN/docs/Web/API/HTMLHeadElement"><code>HTMLHeadElement</code></a>"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLHeadElement)"#
);
tag!(
    header,
    Header,
    HeaderArgs,
    r#""#,
    r#"`<header>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/header)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content (en-US)</a>，但是不允许 <code>&lt;header&gt;</code> 或<a href="/zh-CN/docs/Web/HTML/Element/footer"><code>&lt;footer&gt;</code></a> 成为子元素"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受&nbsp;<a title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" class="only-in-en-us">flow content (en-US)</a> 的元素。注意 <code>&lt;header&gt;</code> 元素不能作为 <a href="/zh-CN/docs/Web/HTML/Element/address"><code>&lt;address&gt;</code></a>、<a href="/zh-CN/docs/Web/HTML/Element/footer"><code>&lt;footer&gt;</code></a> 或另一个 <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a> 元素的子元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    hr,
    Hr,
    HrArgs,
    r#""#,
    r#"`<hr>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/hr)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn>无，是一个&nbsp;<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn>必须有开始标签，不能有结束标签。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn> 所有接受<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流式元素</a>的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement)"#
);
tag!(
    html,
    Html,
    HtmlArgs,
    r#""#,
    r#"`<html>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/html)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"一个 <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a> 元素，后跟一个 <a href="/zh-CN/docs/Web/HTML/Element/body"><code>&lt;body&gt;</code></a> 元素"#,
    r#"- 忽略结束标签"#,
    r#"如果元素中的第一个元素不是注释，则可以忽略标签。"#,
    r#"- 允许的父类"#,
    r#"无（这是文档的根元素）"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLHtmlElement)"#
);
tag!(
    i,
    I,
    IArgs,
    r#""#,
    r#"`<i>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/i)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许量</dfn> <a href="/en-US/docs/Web/HTML/Content_categories#Phrasing_content">phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签忽略</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>Permitted parent elements</dfn> Any element that accepts <a href="/en-US/docs/Web/HTML/Content_categories#Phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    iframe,
    Iframe,
    IframeArgs,
    r#""#,
    r#"`<iframe>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/iframe)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    img,
    Img,
    ImgArgs,
    r#""#,
    r#"`<img>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/img)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"无，它是一个 <a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。"#,
    r#"- 忽略结束标签"#,
    r#"必须有开始标签，不可有结束标签。"#,
    r#"- 允许的父类"#,
    r#"接受嵌入内容的任意元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLImageElement)"#
);
tag!(
    input,
    Input,
    InputArgs,
    r#""#,
    r#"`<input>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/input)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"无，这是一个<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。"#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"<p>必须有开始标签但不必有结束标签。</p>"#,
    r#"- 允许的父类"#,
    r#"任何元素都可以包含语句型元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLInputElement)"#
);
tag!(
    ins,
    Ins,
    InsArgs,
    r#""#,
    r#"`<ins>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/ins)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许内容</dfn> <a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#透明内容模型（transparent_content_model）">透明内容模型</a>。"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签闭合</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父级标签</dfn>任意<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#短语元素（phrasing_content）">短语元素</a>"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement)"#
);
tag!(
    kbd,
    Kbd,
    KbdArgs,
    r#""#,
    r#"`<kbd>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/kbd)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    label,
    Label,
    LabelArgs,
    r#""#,
    r#"`<label>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/label)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>, but no descendant <code>label</code> elements. No labelable elements other than the labeled control are allowed."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLLabelElement)"#
);
tag!(
    legend,
    Legend,
    LegendArgs,
    r#""#,
    r#"`<legend>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/legend)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#Phrasing_content" title="HTML/Content_categories#Phrasing_content">语句内容（Phrasing content</a>）。"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/fieldset"><code>&lt;fieldset&gt;</code></a> ，并且<code>&lt;legend&gt;</code>作为第一个子元素"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement)"#
);
tag!(
    li,
    Li,
    LiArgs,
    r#""#,
    r#"`<li>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/li)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流式内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"如果列表元素的后面紧随另一个 <a href="/zh-CN/docs/Web/HTML/Element/li" aria-current="page"><code>&lt;li&gt;</code></a> 元素，或者它的父元素中没有更多内容，结束标签可以省略。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/ul"><code>&lt;ul&gt;</code></a>、 <a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a>、 或者 <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> 元素。过时的 <a href="/zh-CN/docs/Web/HTML/Element/dir"><code>&lt;dir&gt;</code></a> 也可以作为父元素，但是并不提倡。"#,
    r#"- [dom API]()"#
);
tag!(
    link,
    Link,
    LinkArgs,
    r#""#,
    r#"`<link>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/link)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"无，这是一个<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。"#,
    r#"- 忽略结束标签"#,
    r#"鉴于这是一个空元素，开始标签必须存在，结束标签必须不存在。"#,
    r#"- 允许的父类"#,
    r#"任何可以接受元数据的元素.。如果使用了 <a href="/zh-CN/docs/Web/HTML/Global_attributes/itemprop">itemprop</a>属性，，则其父元素可以是任何可接受&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLLinkElement)"#
);
tag!(
    main,
    Main,
    MainArgs,
    r#""#,
    r#"`<main>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/main)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许内容</dfn> <a title="HTML/Content_categories#Flow_content" style="font-family: 'Microsoft Yahei', sans-serif; background-color: rgba(255, 149, 0, 0.0980392);" href="/en-US/docs/Web/HTML/Content_categories#Flow_content">Flow content</a><span style="background-color: rgba(255, 149, 0, 0.0980392); font-family: microsoft yahei,sans-serif;">.</span>"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn>无; 开始与结束都是强制性。"#,
    r#"- 允许的父类"#,
    r#"<dfn>被允许的父级元素</dfn>任何支持流内容但可能不是继承元素的 元素<a href="/zh-CN/docs/Web/HTML/Element/article"><code>&lt;article&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/aside"><code>&lt;aside&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/footer"><code>&lt;footer&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a>, 或<a href="/zh-CN/docs/Web/HTML/Element/nav"><code>&lt;nav&gt;</code></a>&nbsp;"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    map,
    Map,
    MapArgs,
    r#""#,
    r#"`<map>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/map)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"任何<a href="/en-US/docs/Web/Guide/HTML/Content_categories#transparent_content_model">透明</a>元素。"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语内容</a>的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement)"#
);
tag!(
    mark,
    Mark,
    MarkArgs,
    r#""#,
    r#"`<mark>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/mark)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn> <a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn>任何接受<a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    menu,
    Menu,
    MenuArgs,
    r#""#,
    r#"`<menu>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/menu)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"If the element is in the <em>list menu</em> state: <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>, or alternatively, zero or more occurrences of <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a>, and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a>.<br>"#,
    r#"If the element is in the <em>context menu</em> state: zero or more occurrences, in any order, of <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> (<em>context menu</em> state only), <a href="/zh-CN/docs/Web/HTML/Element/menuitem"><code>&lt;menuitem&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/hr"><code>&lt;hr&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a>, and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement)"#
);
tag!(
    meta,
    Meta,
    MetaArgs,
    r#""#,
    r#"`<meta>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/meta)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn> 无，这是一个 <a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn>因为这是一个 void 元素，必须有开始标签而闭合标签可以省略"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn><code>&lt;meta charset&gt;</code>, <code>&lt;meta http-equiv&gt;</code>: <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a> 元素。如果 <a href="/zh-CN/docs/Web/HTML/Element/meta#attr-http-equiv" aria-current="page"><code>http-equiv</code></a> 不是编码声明，它也可以放在<a href="/zh-CN/docs/Web/HTML/Element/noscript"><code>&lt;noscript&gt;</code></a>元素内，它本身在 <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a>元素内部。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement)"#
);
tag!(
    meter,
    Meter,
    MeterArgs,
    r#""#,
    r#"`<meter>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/meter)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>, but there must be no <code>&lt;meter&gt;</code> element among its descendants."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)"#
);
tag!(
    nav,
    Nav,
    NavArgs,
    r#""#,
    r#"`<nav>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/nav)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" class="only-in-en-us">流 式内容 (en-US)</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"所有允许<a title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" class="only-in-en-us">流式内容 (en-US)</a>的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    noscript,
    Noscript,
    NoscriptArgs,
    r#""#,
    r#"`<noscript>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/noscript)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<p>当脚本被禁用并且它是 <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a>元素的后代时：以下顺序任意，零个或者多个<a href="/zh-CN/docs/Web/HTML/Element/link"><code>&lt;link&gt;</code></a>元素，零个或者多个<a href="/zh-CN/docs/Web/HTML/Element/style"><code>&lt;style&gt;</code></a>元素，零个或者多个<a href="/zh-CN/docs/Web/HTML/Element/meta"><code>&lt;meta&gt;</code></a>元素。</p>"#,
    r#"<p>当脚本被禁用并且它不是 <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a> 元素的子元素时：任何 transparent content 都可以，但是在它的后代中必须没有 <code>&lt;noscript&gt;</code>元素。</p>"#,
    r#"<p>否则：flow content 或 phrasing content 。</p>"#,
    r#"- 忽略结束标签"#,
    r#"<p>不允许，开始标签和结束标签都不能省略。</p>"#,
    r#"- 允许的父类"#,
    r#"<p>如果没有根元素&nbsp;<code>&lt;noscript&gt;</code>，或者在<a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a>元素中（仅用于 HTML 文档）也没有根元素&nbsp;<code>&lt;noscript&gt;</code>，允许任何&nbsp;<a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;元素。</p>"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    object,
    Object,
    ObjectArgs,
    r#""#,
    r#"`<object>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/object)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许内容</dfn> zero or more <a href="/zh-CN/docs/Web/HTML/Element/param"><code>&lt;param&gt;</code></a> elements, then <a title="HTML/Content categories#Transparent content models" href="/en-US/docs/Web/Guide/HTML/Content_categories#transparent_content_models">Transparent content</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签闭合</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父级元素</dfn> Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#embedded_content" title="HTML/Content categories#Embedded content">embedded content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)"#
);
tag!(
    ol,
    Ol,
    OlArgs,
    r#""#,
    r#"`<ol>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/ol)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"Zero or more <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a> elements."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement)"#
);
tag!(
    optgroup,
    Optgroup,
    OptgroupArgs,
    r#""#,
    r#"`<optgroup>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/optgroup)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"0或多个 <a href="/zh-CN/docs/Web/HTML/Element/option"><code>&lt;option&gt;</code></a> 元素"#,
    r#"- 忽略结束标签"#,
    r#"开始标签是必须的。当该元素后面也跟着一个 <span style="font-family: courier new;">&lt;optgroup&gt; </span>元素，或该元素的父元素没有其他内容时，结束标签可省略。"#,
    r#"- 允许的父类"#,
    r#"一个 <a href="/zh-CN/docs/Web/HTML/Element/select"><code>&lt;select&gt;</code></a> 元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLOptGroupElement)"#
);
tag!(
    option,
    Option_,
    OptionArgs,
    r#""#,
    r#"`<option>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/option)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn>带有最终转义字符（例如 <code>&amp;eacute;</code>）的文本"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标记省略</dfn> 开始标记是必需的。如果此元素紧接着是另一个 <code>&lt;option&gt;</code> 元素或<a href="/zh-CN/docs/Web/HTML/Element/optgroup"><code>&lt;optgroup&gt;</code></a>, 或者父元素没有其他内容，则结束标记是可选的。"#,
    r#"- 允许的父类"#,
    r#"<dfn>Implicit ARIA role</dfn><code><a href="https://w3c.github.io/aria/#option" class="external" rel=" noopener">option</a></code>"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLOptionElement)"#
);
tag!(
    output,
    Output,
    OutputArgs,
    r#""#,
    r#"`<output>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/output)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语元素 (en-US)</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"接受任何&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" class="only-in-en-us" title="Currently only available in English (US)">短语元素 (en-US)</a>"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)"#
);
tag!(
    p,
    P,
    PArgs,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn> <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn>起始标签是必需的，结束标签在以下情形中可以省略。&lt;p&gt;元素后紧跟<a href="/zh-CN/docs/Web/HTML/Element/address"><code>&lt;address&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/article"><code>&lt;article&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/aside"><code>&lt;aside&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/blockquote"><code>&lt;blockquote&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/div"><code>&lt;div&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/dl"><code>&lt;dl&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/fieldset"><code>&lt;fieldset&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/footer"><code>&lt;footer&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a>, <a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/Heading_Elements"><code>&lt;h1&gt;</code> <small>(en-US)</small></a>, <a href="/en-US/docs/Web/HTML/Element/Heading_Elements" class="only-in-en-us" title="Currently only available in English (US)"><code>&lt;h2&gt;</code> <small>(en-US)</small></a>, <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/Heading_Elements"><code>&lt;h3&gt;</code> <small>(en-US)</small></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/Heading_Elements" class="only-in-en-us"><code>&lt;h4&gt;</code> <small>(en-US)</small></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/Heading_Elements" class="only-in-en-us"><code>&lt;h5&gt;</code> <small>(en-US)</small></a>, <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/Heading_Elements"><code>&lt;h6&gt;</code> <small>(en-US)</small></a>, <a href="/zh-CN/docs/Web/HTML/Element/header"><code>&lt;header&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/hr"><code>&lt;hr&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/nav"><code>&lt;nav&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/pre"><code>&lt;pre&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/section"><code>&lt;section&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/ul"><code>&lt;ul&gt;</code></a>或另一 个<a href="/zh-CN/docs/Web/HTML/Element/p" aria-current="page"><code>&lt;p&gt;</code></a>元素；或者父元素中没有其他内容了，而且父元素不是<a href="/zh-CN/docs/Web/HTML/Element/a"><code>&lt;a&gt;</code></a>元素"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn>任何接受<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLParagraphElement)"#
);
tag!(
    picture,
    Picture,
    PictureArgs,
    r#""#,
    r#"`<picture>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/picture)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"零或多个 <a href="/zh-CN/docs/Web/HTML/Element/source"><code>&lt;source&gt;</code></a> 元素，以及紧随其后的一个 <a href="/zh-CN/docs/Web/HTML/Element/img"><code>&lt;img&gt;</code></a> 元素，可以混合一些脚本支持的元素。"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何可以包含嵌入内容的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPictureElement)"#
);
tag!(
    portal,
    Portal,
    PortalArgs,
    r#""#,
    r#"`<portal>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/portal)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a class="page-not-created" title="The documentation about this has not yet been written; please consider contributing!"><code>HTMLPortalElement</code></a>"#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    pre,
    Pre,
    PreArgs,
    r#""#,
    r#"`<pre>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/pre)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何可以接受流内容 (&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>)&nbsp;的元素"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement)"#
);
tag!(
    progress,
    Progress,
    ProgressArgs,
    r#""#,
    r#"`<progress>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/progress)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    q,
    Q,
    QArgs,
    r#""#,
    r#"`<q>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/q)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)"#
);
tag!(
    rp,
    Rp,
    RpArgs,
    r#""#,
    r#"`<rp>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/rp)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"文本"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/ruby"><code>&lt;ruby&gt;</code></a> 元素。&nbsp;<code>&lt;rp&gt;&nbsp;</code>必须位于 <a href="/zh-CN/docs/Web/HTML/Element/rt"><code>&lt;rt&gt;</code></a> 的前面和后面。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    rt,
    Rt,
    RtArgs,
    r#""#,
    r#"`<rt>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/rt)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"如果 <a href="/zh-CN/docs/Web/HTML/Element/rt" aria-current="page"><code>&lt;rt&gt;</code></a> 元素紧紧跟随&nbsp; <a href="/zh-CN/docs/Web/HTML/Element/rt" aria-current="page"><code>&lt;rt&gt;</code></a> 或者 <a href="/zh-CN/docs/Web/HTML/Element/rp"><code>&lt;rp&gt;</code></a> 元素，或者父元素中没有更多内容了，结束标签可以省略。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/ruby"><code>&lt;ruby&gt;</code></a> 元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    ruby,
    Ruby,
    RubyArgs,
    r#""#,
    r#"`<ruby>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/ruby)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" title="HTML/Content_categories#Phrasing_content">短语内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"See prose"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    s,
    S,
    SArgs,
    r#""#,
    r#"`<s>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/s)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语内容</a>的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    samp,
    Samp,
    SampArgs,
    r#""#,
    r#"`<samp>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/samp)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a style="line-height: 22px;" title="en/HTML/Content categories#Phrasing content" href="/en-US/HTML/Content_categories#phrasing_content">Phrasing content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts&nbsp;<a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    script,
    Script,
    ScriptArgs,
    r#""#,
    r#"`<script>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/script)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"动态脚本，如 <code>text/javascript</code>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"一些元素可以接受元数据内容，或者是一些元素可以接受短语元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLScriptElement)"#
);
tag!(
    section,
    Section,
    SectionArgs,
    r#""#,
    r#"`<section>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/section)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>. Note that a <code>&lt;section&gt;</code> element must not be a descendant of an <a href="/zh-CN/docs/Web/HTML/Element/address"><code>&lt;address&gt;</code></a> element."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    select,
    Select,
    SelectArgs,
    r#""#,
    r#"`<select>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/select)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"Zero or more <a href="/zh-CN/docs/Web/HTML/Element/option"><code>&lt;option&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/optgroup"><code>&lt;optgroup&gt;</code></a> elements."#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"Any element that accepts&nbsp;<a title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" class="only-in-en-us">phrasing content (en-US)</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLSelectElement)"#
);
tag!(
    shadow,
    Shadow,
    ShadowArgs,
    r#""#,
    r#"`<shadow>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/shadow)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流式内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受流式内容的元素 &nbsp; &nbsp;&nbsp;"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLShadowElement)"#
);
tag!(
    slot,
    Slot,
    SlotArgs,
    r#""#,
    r#"`<slot>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/slot)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#transparent_content_model">Transparent</a>"#,
    r#"- 忽略结束标签"#,
    r#"<code><a title="This is a link to an unwritten page" href="/zh-CN/docs/Web/Reference/Events/slotchange" class="page-not-created">slotchange</a></code>"#,
    r#"- 允许的父类"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLSlotElement)"#
);
tag!(
    small,
    Small,
    SmallArgs,
    r#""#,
    r#"`<small>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/small)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    source,
    Source,
    SourceArgs,
    r#""#,
    r#"`<source>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/source)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"A <a href="/zh-CN/docs/Web/HTML/Element/picture"><code>&lt;picture&gt;</code></a> element, and it should be placed&nbsp;before the <a href="/zh-CN/docs/Web/HTML/Element/img"><code>&lt;img&gt;</code></a> element."#,
    r#"- 忽略结束标签"#,
    r#"None."#,
    r#"- 允许的父类"#,
    r#"None, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)"#
);
tag!(
    span,
    Span,
    SpanArgs,
    r#""#,
    r#"`<span>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/span)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn><a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语元素</a>"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签省略</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn>任意可以包含&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" title="https://developer.mozilla.org/en-US/docs/HTML/Content_categories#Phrasing_content">表述内容</a>&nbsp;或&nbsp;<a title="https://developer.mozilla.org/en-US/docs/HTML/Content_categories#Flow_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流内容</a>&nbsp;的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLSpanElement)"#
);
tag!(
    strong,
    Strong,
    StrongArgs,
    r#""#,
    r#"`<strong>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/strong)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    style,
    Style,
    StyleArgs,
    r#""#,
    r#"`<style>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/style)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<dfn>允许的内容</dfn>与&nbsp;<code>type</code> 属性相匹配的文本内容，也就是&nbsp;<code>text/css</code>"#,
    r#"- 忽略结束标签"#,
    r#"<dfn>标签忽略</dfn> 不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<dfn>允许的父元素</dfn> 任意接受<a class="only-in-en-us" href="/en-US/docs/Web/Guide/HTML/Content_categories#metadata_content" title="Currently only available in English (US)">元数据内容 (en-US)</a>的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLStyleElement)"#
);
tag!(
    sub,
    Sub,
    SubArgs,
    r#""#,
    r#"`<sub>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/sub)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"短语内容"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"可以包含短语内容的任意元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    summary,
    Summary,
    SummaryArgs,
    r#""#,
    r#"`<summary>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/summary)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/summary" aria-current="page"><code>&lt;summary&gt;</code></a>元素是<a href="/zh-CN/docs/Web/HTML/Element/details"><code>&lt;details&gt;</code></a> 元素的子元素。"#,
    r#"- 忽略结束标签"#,
    r#"<a href="https://www.whatwg.org/specs/web-apps/current-work/multipage/interactive-elements.html#the-summary-element" class="external" rel="external nofollow noopener">HTML5, section 4.11.2</a>"#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.orghttps://www.whatwg.org/specs/web-apps/current-work/multipage/interactive-elements.html#the-summary-element)"#
);
tag!(
    sup,
    Sup,
    SupArgs,
    r#""#,
    r#"`<sup>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/sup)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"短语内容"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"可以包含短语内容的任意元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    table,
    Table,
    TableArgs,
    r#""#,
    r#"`<table>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/table)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<div class="content-models" id="sect3">"#,
    r#"<div id="table-mdls">按照这个顺序："#,
    r#"<ul>"#,
    r#"<li>一个可选的 <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a> 元素</li>"#,
    r#"<li>零个或多个的 <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> 元素</li>"#,
    r#"<li>一个可选的&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a> 元素</li>"#,
    r#"<li>下列任意一个："#,
    r#"<ul>"#,
    r#"<li>零个或多个 <a class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/tbody" title="Currently only available in English (US)"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a></li>"#,
    r#"<li>零个或多个 <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a></li>"#,
    r#"</ul>"#,
    r#"</li>"#,
    r#"<li>一个可选的&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> 元素</li>"#,
    r#"</ul>"#,
    r#"</div>"#,
    r#"</div>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何支持<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流内容</a>的元素"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLTableElement)"#
);
tag!(
    tbody,
    Tbody,
    TbodyArgs,
    r#""#,
    r#"`<tbody>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/tbody)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"Zero or more <a href="/en-US/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> elements."#,
    r#"- 忽略结束标签"#,
    r#"The `"#,
    r#"- 允许的父类"#,
    r#""#,
    r#"Within the required parent <a href="/en-US/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element,"#,
    r#"the <code>&lt;tbody&gt;</code> element can be added after a"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a>,"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a>, and a"#,
    r#"<a href="/en-US/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a> element."#,
    r#""#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)"#
);
tag!(
    td,
    Td,
    TdArgs,
    r#""#,
    r#"`<td>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/td)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>."#,
    r#"- 忽略结束标签"#,
    r#"The start tag is mandatory.<br>"#,
    r#"The end tag may be omitted, if it is immediately followed by a <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/td" aria-current="page"><code>&lt;td&gt;</code></a> element or if there are no more data in its parent element."#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)"#
);
tag!(
    template,
    Template,
    TemplateArgs,
    r#""#,
    r#"`<template>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/template)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"No restrictions"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/body"><code>&lt;body&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/frameset"><code>&lt;frameset&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/dl"><code>&lt;dl&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> without a <code>span</code> attribute"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLTemplateElement)"#
);
tag!(
    textarea,
    Textarea,
    TextareaArgs,
    r#""#,
    r#"`<textarea>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/textarea)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    tfoot,
    Tfoot,
    TfootArgs,
    r#""#,
    r#"`<tfoot>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/tfoot)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"0或多个<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素。"#,
    r#"- 忽略结束标签"#,
    r#"开始标签是必需的。.在父元素 <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 没有后续内容的情况下，结束标签可被省略 。"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 元素。<a href="/zh-CN/docs/Web/HTML/Element/tfoot" aria-current="page"><code>&lt;tfoot&gt;</code></a> 必须出现在一个或多个 <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a>，<a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a>，<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>，或 <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素之后。 注意这是自 HTML5 起有的要求。<br>"#,
    r#"<span class="badge inline html-version"><a href="/zh-CN/docs/Web/HTML">HTML 4</a></span> <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> 元素不能放在任何 <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> 或 <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素之后。注意，这与上述 HTML5 的标准相冲突。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)"#
);
tag!(
    th,
    Th,
    ThArgs,
    r#""#,
    r#"`<th>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/th)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<div id="sect1" class="content-models">"#,
    r#"<div id="table-mdls">流内容（除 header、footer、sectioning content 或 heading content 的继承。）</div>"#,
    r#"</div>"#,
    r#"- 忽略结束标签"#,
    r#"开始标签是必需要的，而结束标签有时可以省略：当其后是<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a>  或 <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> ，或者其后没有其他数据内容在其父元素内，。"#,
    r#"- 允许的父类"#,
    r#"&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)"#
);
tag!(
    thead,
    Thead,
    TheadArgs,
    r#""#,
    r#"`<thead>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/thead)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"零或多个<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a>元素。"#,
    r#"- 忽略结束标签"#,
    r#"开头的标签是强制的。如果<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a> 元素后直接跟 <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>或<a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>元素，结尾的标签可以被省略。"#,
    r#"- 允许的父类"#,
    r#"A <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element. The <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a> must appear after any <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> element, even implicitly defined, but before any <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>, <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element."#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)"#
);
tag!(
    time,
    Time,
    TimeArgs,
    r#""#,
    r#"`<time>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/time)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    title,
    Title,
    TitleArgs,
    r#""#,
    r#"`<title>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/title)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"非空字符或特殊字符（<a href="/en-US/docs/Glossary/Whitespace" title="Currently only available in English (US)" class="only-in-en-us">whitespace <small>(en-US)</small></a>）的文本"#,
    r#"- 忽略结束标签"#,
    r#"同时需要开标签和闭标签。注意：遗漏<code> &lt;/title&gt;</code> 标签会导致浏览器忽略掉页面的剩余部分。"#,
    r#"- 允许的父类"#,
    r#"一个 <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a> 元素只能包含一个 <a href="/zh-CN/docs/Web/HTML/Element/title" aria-current="page"><code>&lt;title&gt;</code></a> 元素"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement)"#
);
tag!(
    tr,
    Tr,
    TrArgs,
    r#""#,
    r#"`<tr>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/tr)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"Zero or more <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> elements, or a mix of them"#,
    r#"- 忽略结束标签"#,
    r#"Start tag is mandatory. End tag may be omitted if the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element is immediately followed by a <a href="/zh-CN/docs/Web/HTML/Element/tr" aria-current="page"><code>&lt;tr&gt;</code></a> element, or if the parent table group (<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/tbody"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> or <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>) element doesn't have any more content"#,
    r#"- 允许的父类"#,
    r#"<a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> or <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> element"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLTableRowElement)"#
);
tag!(
    track,
    Track,
    TrackArgs,
    r#""#,
    r#"`<track>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/track)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    u,
    U,
    UArgs,
    r#""#,
    r#"`<u>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/u)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语内容</a>"#,
    r#"- 忽略结束标签"#,
    r#"不允许，开始标签和结束标签都不能省略。"#,
    r#"- 允许的父类"#,
    r#"任何接受<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">短语内容</a>的元素。"#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
tag!(
    ul,
    Ul,
    UlArgs,
    r#""#,
    r#"`<ul>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/ul)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"<a href="/en-US/HTML/Content_categories#flow_content" title="en/HTML/Content categories#Flow content">流式内容</a>，如果 <code>&lt;ul&gt;</code> 包含至少一个 <code>&lt;li&gt;</code> 元素，那么它就是显性内容 <a href="/en-US/docs/Web/Guide/HTML/Content_categories#Palpable_content">Palpable content</a>。"#,
    r#"- 忽略结束标签"#,
    r#"允许的内容"#,
    r#"- 允许的父类"#,
    r#"零个或更多个 <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a> 元素，可以混合使用 <a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a> 与<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/ul"><code>&lt;ul&gt;</code></a> 元素。"#,
    r#"- [dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)"#
);
tag!(
    var,
    Var,
    VarArgs,
    r#""#,
    r#"`<var>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/var)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    video,
    Video,
    VideoArgs,
    r#""#,
    r#"`<video>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/video)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#""#,
    r#"- 忽略结束标签"#,
    r#""#,
    r#"- 允许的父类"#,
    r#""#,
    r#"- [dom API]()"#
);
tag!(
    wbr,
    Wbr,
    WbrArgs,
    r#""#,
    r#"`<wbr>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/wbr)"#,
    r#""#,
    r#"----"#,
    r#""#,
    r#"- 允许的内容"#,
    r#"Empty"#,
    r#"- 忽略结束标签"#,
    r#"It is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>; it must have a start tag, but must not have an end tag."#,
    r#"- 允许的父类"#,
    r#"Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>."#,
    r#"- [dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLElement)"#
);
