use bevy::prelude::*;
use rand::{RngExt, rngs::StdRng};
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;