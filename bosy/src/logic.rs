pub(crate) trait Logic {
    type Manager;
}

use smtlib;

impl Logic for smtlib::Term {
    type Manager = smtlib::Instance;
}
