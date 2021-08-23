////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod functions;
mod macros;
mod structures;

////////////////////////////////////////////////////////////////////////////////////////////////////

// macros
#[macro_export]
macro_rules! load_me_anchor {
  ( $me_chimeric_read: expr, $orientation: tt ) => {
    let mut me_anchor = MEAnchor::new();
    me_anchor.orientation = OrientationEnum::$orientation;
    $me_chimeric_read.me_read.push(me_anchor);
  };

  ( $me_chimeric_read: expr, $position: expr, $orientation: tt ) => {
    let mut me_anchor = MEAnchor::new();
    me_anchor.orientation = OrientationEnum::$orientation;
    me_anchor.position = $position;
    $me_chimeric_read.me_read.push(me_anchor);
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
