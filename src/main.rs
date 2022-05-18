// Copyright 2022 Mufeed Ali
// SPDX-License-Identifier: GPL-3.0-or-later

mod application;
#[rustfmt::skip]
mod config;
mod window;

use gettextrs::{gettext, LocaleCategory};
use gtk::{gio, glib};

use self::application::RameshApplication;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Ramesh"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = RameshApplication::new();
    app.run();
}
