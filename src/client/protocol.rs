pub mod core;#[cfg(feature = "wlr")]
        #[cfg_attr(docsrs, doc(cfg(feature = "wlr")))]
        pub mod wlr;
#[cfg(feature = "hyprland")]
        #[cfg_attr(docsrs, doc(cfg(feature = "hyprland")))]
        pub mod hyprland;
#[cfg(feature = "weston")]
        #[cfg_attr(docsrs, doc(cfg(feature = "weston")))]
        pub mod weston;
#[cfg(feature = "mesa")]
        #[cfg_attr(docsrs, doc(cfg(feature = "mesa")))]
        pub mod mesa;
#[cfg(feature = "cosmic")]
        #[cfg_attr(docsrs, doc(cfg(feature = "cosmic")))]
        pub mod cosmic;
#[cfg(feature = "unstable")]
        #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
        pub mod unstable;
#[cfg(feature = "stable")]
        #[cfg_attr(docsrs, doc(cfg(feature = "stable")))]
        pub mod stable;
#[cfg(feature = "plasma")]
        #[cfg_attr(docsrs, doc(cfg(feature = "plasma")))]
        pub mod plasma;
#[cfg(feature = "frog")]
        #[cfg_attr(docsrs, doc(cfg(feature = "frog")))]
        pub mod frog;
#[cfg(feature = "ivi")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ivi")))]
        pub mod ivi;
#[cfg(feature = "staging")]
        #[cfg_attr(docsrs, doc(cfg(feature = "staging")))]
        pub mod staging;
#[cfg(feature = "treeland")]
        #[cfg_attr(docsrs, doc(cfg(feature = "treeland")))]
        pub mod treeland;
