
use crate::data;

pub struct GuberState {
    _initialized: bool,
    _gear: Vec<data::gear::Gear>,
}

impl Default for GuberState {
    fn default() -> Self {
        Self {
            _initialized: false,
            _gear: vec![],
        }
    }
}

impl GuberState {

}